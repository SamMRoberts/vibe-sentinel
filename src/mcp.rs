use std::io::{BufRead, Write};
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::adapters::fs::FsWorkspaceProbe;
use crate::core::StatusService;
use crate::domain::{StatusCheck, StatusReport, VibeError};
use crate::ports::WorkspaceProbe;

pub const STATUS_TOOL_NAME: &str = "vibe_sentinel_status";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct McpServerConfig {
    pub root: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct McpToolDescriptor {
    pub name: String,
    pub description: String,
    pub read_only: bool,
    pub idempotent: bool,
    pub local_only: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum McpStatusRequest {
    Status,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct McpStatusResponse {
    pub project_name: String,
    pub ready: bool,
    pub checks: Vec<StatusCheck>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum McpErrorCode {
    InvalidRequest,
    WorkspaceUnreadable,
    InternalError,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct McpErrorResponse {
    pub code: McpErrorCode,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct JsonRpcRequest {
    jsonrpc: String,
    id: Option<Value>,
    method: String,
    params: Option<Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct JsonRpcResponse {
    jsonrpc: String,
    id: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<JsonRpcError>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct JsonRpcError {
    code: i64,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Value>,
}

struct StdioServer<R: BufRead, W: Write> {
    config: McpServerConfig,
    reader: R,
    writer: W,
}

impl<R: BufRead, W: Write> StdioServer<R, W> {
    fn new(config: McpServerConfig, reader: R, writer: W) -> Self {
        Self {
            config,
            reader,
            writer,
        }
    }

    fn run(&mut self) -> Result<(), VibeError> {
        while let Some(payload) = read_content_length_message(&mut self.reader)? {
            let response = match serde_json::from_str::<JsonRpcRequest>(&payload) {
                Ok(request) => handle_json_rpc_request(&self.config, request)?,
                Err(error) => Some(json_rpc_error(
                    None,
                    -32700,
                    format!("invalid JSON-RPC payload: {error}"),
                    None,
                )),
            };

            if let Some(response) = response {
                let payload = serde_json::to_string(&response).map_err(|error| {
                    VibeError::StatusEvaluationFailed(format!(
                        "could not serialize MCP response: {error}"
                    ))
                })?;
                write_content_length_message(&mut self.writer, &payload)?;
            }
        }

        Ok(())
    }
}

pub fn status_tool_descriptor() -> McpToolDescriptor {
    McpToolDescriptor {
        name: STATUS_TOOL_NAME.to_string(),
        description: "Read local vibe-sentinel harness readiness status".to_string(),
        read_only: true,
        idempotent: true,
        local_only: true,
    }
}

pub fn evaluate_status_tool<P: WorkspaceProbe>(probe: P) -> Result<McpStatusResponse, VibeError> {
    StatusService::new(probe)
        .evaluate()
        .map(response_from_report)
}

pub fn response_from_report(report: StatusReport) -> McpStatusResponse {
    let ready = report.is_ready();
    McpStatusResponse {
        project_name: report.project_name,
        ready,
        checks: report.checks,
    }
}

pub fn map_error(error: VibeError) -> McpErrorResponse {
    let code = match error {
        VibeError::InvalidArguments(_) => McpErrorCode::InvalidRequest,
        VibeError::WorkspaceUnreadable(_) => McpErrorCode::WorkspaceUnreadable,
        VibeError::StatusEvaluationFailed(_) => McpErrorCode::InternalError,
    };

    McpErrorResponse {
        code,
        message: error.to_string(),
    }
}

pub fn run_stdio_server(config: McpServerConfig) -> Result<(), VibeError> {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    run_stdio_session(config, stdin.lock(), stdout.lock())
}

fn run_stdio_session<R: BufRead, W: Write>(
    config: McpServerConfig,
    reader: R,
    writer: W,
) -> Result<(), VibeError> {
    StdioServer::new(config, reader, writer).run()
}

fn read_content_length_message<R: BufRead>(reader: &mut R) -> Result<Option<String>, VibeError> {
    let mut content_length = None;
    let mut saw_header = false;

    loop {
        let mut line = String::new();
        let bytes_read = reader.read_line(&mut line).map_err(|error| {
            VibeError::StatusEvaluationFailed(format!("could not read MCP header: {error}"))
        })?;
        if bytes_read == 0 {
            return if saw_header {
                Err(VibeError::InvalidArguments(
                    "unexpected end of MCP headers".to_string(),
                ))
            } else {
                Ok(None)
            };
        }

        saw_header = true;
        let header = line.trim_end_matches(['\r', '\n']);
        if header.is_empty() {
            break;
        }

        if let Some((name, value)) = header.split_once(':') {
            if name.eq_ignore_ascii_case("Content-Length") {
                let length = value.trim().parse::<usize>().map_err(|error| {
                    VibeError::InvalidArguments(format!(
                        "invalid MCP Content-Length header: {error}"
                    ))
                })?;
                content_length = Some(length);
            }
        }
    }

    let length = content_length.ok_or_else(|| {
        VibeError::InvalidArguments("missing MCP Content-Length header".to_string())
    })?;
    let mut body = Vec::with_capacity(length);
    while body.len() < length {
        let buffer = reader.fill_buf().map_err(|error| {
            VibeError::InvalidArguments(format!("could not read MCP message body: {error}"))
        })?;
        if buffer.is_empty() {
            return Err(VibeError::InvalidArguments(
                "could not read MCP message body: unexpected end of stream".to_string(),
            ));
        }

        let remaining = length - body.len();
        let chunk_len = remaining.min(buffer.len());
        body.extend_from_slice(&buffer[..chunk_len]);
        reader.consume(chunk_len);
    }

    String::from_utf8(body).map(Some).map_err(|error| {
        VibeError::InvalidArguments(format!("MCP message body is not UTF-8: {error}"))
    })
}

fn write_content_length_message<W: Write>(writer: &mut W, payload: &str) -> Result<(), VibeError> {
    write!(
        writer,
        "Content-Type: application/vscode-jsonrpc; charset=utf-8\r\nContent-Length: {}\r\n\r\n{}",
        payload.len(),
        payload
    )
    .map_err(|error| {
        VibeError::StatusEvaluationFailed(format!("could not write MCP message: {error}"))
    })?;
    writer.flush().map_err(|error| {
        VibeError::StatusEvaluationFailed(format!("could not flush MCP message: {error}"))
    })
}

fn handle_json_rpc_request(
    config: &McpServerConfig,
    request: JsonRpcRequest,
) -> Result<Option<JsonRpcResponse>, VibeError> {
    if request.jsonrpc != "2.0" {
        return Ok(Some(json_rpc_error(
            request.id,
            -32600,
            "invalid JSON-RPC version: expected 2.0",
            None,
        )));
    }

    match request.method.as_str() {
        "initialize" => handle_initialize(request.id, request.params).map(Some),
        "notifications/initialized" => Ok(None),
        "tools/list" => handle_tools_list(request.id).map(Some),
        "tools/call" => handle_tools_call(config, request.id, request.params).map(Some),
        method => Ok(Some(json_rpc_error(
            request.id,
            -32601,
            format!("unsupported MCP method `{method}`"),
            None,
        ))),
    }
}

fn handle_initialize(
    id: Option<Value>,
    params: Option<Value>,
) -> Result<JsonRpcResponse, VibeError> {
    let protocol_version = initialize_protocol_version(params.as_ref());
    Ok(json_rpc_success(
        id,
        serialize_protocol_result(serde_json::json!({
            "protocolVersion": protocol_version,
            "capabilities": {
                "tools": {
                    "listChanged": false
                }
            },
            "serverInfo": {
                "name": "vibe-sentinel",
                "version": env!("CARGO_PKG_VERSION")
            }
        }))?,
    ))
}

fn initialize_protocol_version(params: Option<&Value>) -> String {
    params
        .and_then(|params| params.get("protocolVersion"))
        .and_then(Value::as_str)
        .unwrap_or("2025-06-18")
        .to_string()
}

fn handle_tools_list(id: Option<Value>) -> Result<JsonRpcResponse, VibeError> {
    let descriptor = status_tool_descriptor();
    Ok(json_rpc_success(
        id,
        serialize_protocol_result(serde_json::json!({
            "tools": [{
                "name": descriptor.name,
                "description": descriptor.description,
                "inputSchema": {
                    "type": "object",
                    "properties": {},
                    "additionalProperties": false
                },
                "annotations": {
                    "readOnlyHint": descriptor.read_only,
                    "idempotentHint": descriptor.idempotent,
                    "destructiveHint": false,
                    "openWorldHint": !descriptor.local_only
                }
            }]
        }))?,
    ))
}

fn handle_tools_call(
    config: &McpServerConfig,
    id: Option<Value>,
    params: Option<Value>,
) -> Result<JsonRpcResponse, VibeError> {
    let params = params
        .and_then(|value| value.as_object().cloned())
        .ok_or_else(|| {
            VibeError::InvalidArguments("MCP tools/call requires object params".to_string())
        })?;
    let name = params.get("name").and_then(Value::as_str).ok_or_else(|| {
        VibeError::InvalidArguments("MCP tools/call requires a tool name".to_string())
    })?;

    if name != STATUS_TOOL_NAME {
        return Ok(json_rpc_error(
            id,
            -32602,
            format!("unknown MCP tool `{name}`"),
            None,
        ));
    }

    let result = match evaluate_status_tool(FsWorkspaceProbe::new(config.root.clone())) {
        Ok(response) => serde_json::json!({
            "content": [{
                "type": "text",
                "text": serde_json::to_string(&response).map_err(|error| {
                    VibeError::StatusEvaluationFailed(format!(
                        "could not serialize MCP status content: {error}"
                    ))
                })?
            }],
            "structuredContent": response,
            "isError": false
        }),
        Err(error) => {
            let response = map_error(error);
            let message = response.message.clone();
            serde_json::json!({
                "content": [{
                    "type": "text",
                    "text": message
                }],
                "structuredContent": response,
                "isError": true
            })
        }
    };

    Ok(json_rpc_success(id, serialize_protocol_result(result)?))
}

fn json_rpc_success(id: Option<Value>, result: Value) -> JsonRpcResponse {
    JsonRpcResponse {
        jsonrpc: "2.0".to_string(),
        id,
        result: Some(result),
        error: None,
    }
}

fn json_rpc_error(
    id: Option<Value>,
    code: i64,
    message: impl Into<String>,
    data: Option<Value>,
) -> JsonRpcResponse {
    JsonRpcResponse {
        jsonrpc: "2.0".to_string(),
        id,
        result: None,
        error: Some(JsonRpcError {
            code,
            message: message.into(),
            data,
        }),
    }
}

fn serialize_protocol_result<T: Serialize>(value: T) -> Result<Value, VibeError> {
    serde_json::to_value(value).map_err(|error| {
        VibeError::StatusEvaluationFailed(format!(
            "could not serialize MCP protocol value: {error}"
        ))
    })
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;
    use std::io::{Cursor, Write};
    use std::path::{Path, PathBuf};
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::{
        evaluate_status_tool, map_error, read_content_length_message, run_stdio_session,
        status_tool_descriptor, write_content_length_message, McpErrorCode, McpServerConfig,
        STATUS_TOOL_NAME,
    };
    use crate::adapters::test_support::FakeWorkspaceProbe;
    use crate::domain::ReadinessState;
    use crate::domain::VibeError;

    #[test]
    fn mcp_skeleton_exposes_status_tool_name() {
        assert_eq!(STATUS_TOOL_NAME, "vibe_sentinel_status");
    }

    #[test]
    fn status_tool_descriptor_is_read_only_idempotent_and_local() {
        let descriptor = status_tool_descriptor();

        assert_eq!(descriptor.name, STATUS_TOOL_NAME);
        assert!(descriptor.read_only);
        assert!(descriptor.idempotent);
        assert!(descriptor.local_only);
        assert!(descriptor.description.contains("readiness"));
    }

    #[test]
    fn status_tool_response_matches_status_report_shape() {
        let response = evaluate_status_tool(
            FakeWorkspaceProbe::new()
                .with_path("AGENTS.md")
                .with_path("docs/harness/scope.md")
                .with_path("docs/harness/operating-model.md")
                .with_path("Cargo.toml")
                .with_active_plan(true),
        )
        .expect("status response");

        assert_eq!(response.project_name, "vibe-sentinel");
        assert!(response.ready);
        assert_eq!(response.checks.len(), 3);
        assert!(response
            .checks
            .iter()
            .all(|check| check.state == ReadinessState::Ready));
    }

    #[test]
    fn status_tool_maps_workspace_errors_to_mcp_errors() {
        let response = map_error(VibeError::WorkspaceUnreadable(
            "could not read active plan directory".to_string(),
        ));

        assert_eq!(response.code, McpErrorCode::WorkspaceUnreadable);
        assert_eq!(response.message, "could not read active plan directory");
    }

    #[test]
    fn mcp_stdio_session_exits_cleanly_after_empty_input() {
        let mut output = Vec::new();
        let result = run_stdio_session(
            McpServerConfig {
                root: PathBuf::from("."),
            },
            Cursor::new(Vec::new()),
            &mut output,
        );

        assert_eq!(result, Ok(()));
        assert!(output.is_empty());
    }

    #[test]
    fn content_length_round_trips_json_payload() {
        let mut output = Vec::new();

        write_content_length_message(&mut output, "{\"jsonrpc\":\"2.0\"}")
            .expect("write framed payload");
        let mut input = Cursor::new(output);
        let payload = read_content_length_message(&mut input).expect("read framed payload");

        assert_eq!(payload, Some("{\"jsonrpc\":\"2.0\"}".to_string()));
    }

    #[test]
    fn content_length_round_trips_json_payload_with_bufreader() {
        let mut output = Vec::new();

        write_content_length_message(&mut output, "{\"jsonrpc\":\"2.0\"}")
            .expect("write framed payload");
        let mut input = BufReader::with_capacity(8, Cursor::new(output));
        let payload = read_content_length_message(&mut input).expect("read framed payload");

        assert_eq!(payload, Some("{\"jsonrpc\":\"2.0\"}".to_string()));
    }

    #[test]
    fn content_length_reader_accepts_case_insensitive_headers_and_extensions() {
        let input = b"content-type: application/vscode-jsonrpc; charset=utf-8\r\ncontent-length: 17\r\n\r\n{\"jsonrpc\":\"2.0\"}";
        let mut cursor = Cursor::new(input.to_vec());

        let payload = read_content_length_message(&mut cursor).expect("read framed payload");

        assert_eq!(payload, Some("{\"jsonrpc\":\"2.0\"}".to_string()));
    }

    #[test]
    fn content_length_writer_flushes_response() {
        let mut writer = FlushRecordingWriter::default();

        write_content_length_message(&mut writer, "{\"jsonrpc\":\"2.0\"}")
            .expect("write framed payload");

        assert!(writer.flushed);
        assert_eq!(
            String::from_utf8(writer.written).expect("utf8 payload"),
            "Content-Type: application/vscode-jsonrpc; charset=utf-8\r\nContent-Length: 17\r\n\r\n{\"jsonrpc\":\"2.0\"}"
        );
    }

    #[test]
    fn session_handles_initialize_and_tools_list_requests() {
        let workspace = TestWorkspace::new();
        let input = framed_messages(&[
            r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}"#,
            r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#,
            r#"{"jsonrpc":"2.0","id":2,"method":"tools/list"}"#,
        ]);
        let mut output = Vec::new();

        run_stdio_session(
            McpServerConfig {
                root: workspace.root().to_path_buf(),
            },
            Cursor::new(input),
            &mut output,
        )
        .expect("stdio session");

        let responses = decode_framed_responses(&output);
        assert_eq!(responses.len(), 2);
        assert_eq!(responses[0]["id"], 1);
        assert_eq!(
            responses[0]["result"]["serverInfo"]["name"],
            "vibe-sentinel"
        );
        assert_eq!(
            responses[0]["result"]["capabilities"]["tools"]["listChanged"],
            false
        );
        assert_eq!(responses[1]["id"], 2);
        assert_eq!(responses[1]["result"]["tools"][0]["name"], STATUS_TOOL_NAME);
        assert_eq!(
            responses[1]["result"]["tools"][0]["annotations"]["readOnlyHint"],
            true
        );
    }

    #[test]
    fn initialize_echoes_client_protocol_version() {
        let workspace = TestWorkspace::new();
        let input = framed_messages(&[
            r#"{"jsonrpc":"2.0","id":3,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"vscode","version":"test"}}}"#,
        ]);
        let mut output = Vec::new();

        run_stdio_session(
            McpServerConfig {
                root: workspace.root().to_path_buf(),
            },
            Cursor::new(input),
            &mut output,
        )
        .expect("stdio session");

        let responses = decode_framed_responses(&output);
        assert_eq!(responses[0]["id"], 3);
        assert_eq!(responses[0]["result"]["protocolVersion"], "2025-06-18");
        assert_eq!(
            responses[0]["result"]["serverInfo"]["name"],
            "vibe-sentinel"
        );
    }

    #[test]
    fn session_handles_status_tool_call_request() {
        let workspace = TestWorkspace::new();
        let input = framed_messages(&[
            r#"{"jsonrpc":"2.0","id":"status-1","method":"tools/call","params":{"name":"vibe_sentinel_status","arguments":{}}}"#,
        ]);
        let mut output = Vec::new();

        run_stdio_session(
            McpServerConfig {
                root: workspace.root().to_path_buf(),
            },
            Cursor::new(input),
            &mut output,
        )
        .expect("stdio session");

        let responses = decode_framed_responses(&output);
        let structured = &responses[0]["result"]["structuredContent"];
        assert_eq!(responses[0]["id"], "status-1");
        assert_eq!(responses[0]["result"]["isError"], false);
        assert_eq!(structured["project_name"], "vibe-sentinel");
        assert_eq!(structured["ready"], true);
        assert_eq!(structured["checks"].as_array().expect("checks").len(), 3);
    }

    #[test]
    fn session_maps_unknown_tool_to_invalid_request_error() {
        let workspace = TestWorkspace::new();
        let input = framed_messages(&[
            r#"{"jsonrpc":"2.0","id":7,"method":"tools/call","params":{"name":"unknown_tool","arguments":{}}}"#,
        ]);
        let mut output = Vec::new();

        run_stdio_session(
            McpServerConfig {
                root: workspace.root().to_path_buf(),
            },
            Cursor::new(input),
            &mut output,
        )
        .expect("stdio session");

        let responses = decode_framed_responses(&output);
        assert_eq!(responses[0]["id"], 7);
        assert_eq!(responses[0]["error"]["code"], -32602);
        assert!(responses[0]["error"]["message"]
            .as_str()
            .expect("message")
            .contains("unknown MCP tool"));
    }

    #[test]
    fn session_maps_workspace_probe_errors_to_tool_error_payload() {
        let workspace = TestWorkspace::new_with_unreadable_active_plan();
        let input = framed_messages(&[
            r#"{"jsonrpc":"2.0","id":8,"method":"tools/call","params":{"name":"vibe_sentinel_status","arguments":{}}}"#,
        ]);
        let mut output = Vec::new();

        run_stdio_session(
            McpServerConfig {
                root: workspace.root().to_path_buf(),
            },
            Cursor::new(input),
            &mut output,
        )
        .expect("stdio session");

        let responses = decode_framed_responses(&output);
        assert_eq!(responses[0]["id"], 8);
        assert_eq!(responses[0]["result"]["isError"], true);
        assert_eq!(
            responses[0]["result"]["structuredContent"]["code"],
            "workspace_unreadable"
        );
    }

    fn framed_messages(messages: &[&str]) -> Vec<u8> {
        let mut output = Vec::new();
        for message in messages {
            write!(
                output,
                "Content-Length: {}\r\n\r\n{}",
                message.len(),
                message
            )
            .expect("framed message");
        }
        output
    }

    fn decode_framed_responses(output: &[u8]) -> Vec<serde_json::Value> {
        let mut cursor = Cursor::new(output.to_vec());
        let mut responses = Vec::new();
        while let Some(payload) = read_content_length_message(&mut cursor).expect("framed response")
        {
            responses.push(serde_json::from_str(&payload).expect("json response"));
        }
        responses
    }

    #[derive(Default)]
    struct FlushRecordingWriter {
        written: Vec<u8>,
        flushed: bool,
    }

    impl Write for FlushRecordingWriter {
        fn write(&mut self, buffer: &[u8]) -> std::io::Result<usize> {
            self.written.extend_from_slice(buffer);
            Ok(buffer.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            self.flushed = true;
            Ok(())
        }
    }

    struct TestWorkspace {
        root: PathBuf,
    }

    impl TestWorkspace {
        fn new() -> Self {
            let root = unique_test_root();
            write_ready_workspace(&root);
            Self { root }
        }

        fn new_with_unreadable_active_plan() -> Self {
            let root = unique_test_root();
            write_ready_workspace(&root);
            std::fs::remove_dir_all(root.join("docs/exec-plans/active"))
                .expect("remove active dir");
            std::fs::write(root.join("docs/exec-plans/active"), "not a directory")
                .expect("active file");
            Self { root }
        }

        fn root(&self) -> &Path {
            &self.root
        }
    }

    impl Drop for TestWorkspace {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.root);
        }
    }

    fn write_ready_workspace(root: &Path) {
        std::fs::create_dir_all(root.join("docs/harness")).expect("harness docs dir");
        std::fs::create_dir_all(root.join("docs/exec-plans/active")).expect("active plan dir");
        std::fs::write(root.join("AGENTS.md"), "# AGENTS\n").expect("agents doc");
        std::fs::write(root.join("docs/harness/scope.md"), "# Scope\n").expect("scope doc");
        std::fs::write(
            root.join("docs/harness/operating-model.md"),
            "# Operating Model\n",
        )
        .expect("operating model doc");
        std::fs::write(root.join("docs/exec-plans/active/README.md"), "# Active\n")
            .expect("active readme");
        std::fs::write(root.join("docs/exec-plans/active/test-plan.md"), "# Plan\n")
            .expect("active plan");
        std::fs::write(root.join("Cargo.toml"), "[package]\nname = \"fixture\"\n")
            .expect("cargo manifest");
    }

    fn unique_test_root() -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time")
            .as_nanos();
        let base = std::env::var_os("TMPDIR")
            .map(PathBuf::from)
            .unwrap_or_else(std::env::temp_dir);
        base.join(format!("vibe-sentinel-mcp-test-{nanos}"))
    }
}
