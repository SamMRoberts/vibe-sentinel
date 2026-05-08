use vibe_sentinel::domain::{ReadinessState, StatusCheck, StatusReport};
use vibe_sentinel::tui::{render_status_to_buffer, StatusTuiKey, StatusTuiModel};

#[test]
fn status_tui_model_quits_on_q_or_escape() {
    let report = ready_report();
    let mut model = StatusTuiModel::new(report.clone());

    assert!(!model.should_quit());

    model.handle_key(StatusTuiKey::Other);
    assert!(!model.should_quit());

    model.handle_key(StatusTuiKey::Quit);
    assert!(model.should_quit());

    let mut escape_model = StatusTuiModel::new(report);
    escape_model.handle_key(StatusTuiKey::Escape);
    assert!(escape_model.should_quit());
}

#[test]
fn status_tui_render_includes_project_ready_state_and_checks() {
    let buffer = render_status_to_buffer(&ready_report(), ratatui::layout::Rect::new(0, 0, 80, 12));
    let contents = buffer_text(&buffer);

    assert!(contents.contains("vibe-sentinel"));
    assert!(contents.contains("ready"));
    assert!(contents.contains("harness docs"));
    assert!(contents.contains("required harness docs present"));
}

fn ready_report() -> StatusReport {
    StatusReport {
        project_name: "vibe-sentinel".to_string(),
        checks: vec![StatusCheck {
            name: "harness docs".to_string(),
            state: ReadinessState::Ready,
            detail: "required harness docs present".to_string(),
        }],
    }
}

fn buffer_text(buffer: &ratatui::buffer::Buffer) -> String {
    buffer
        .content()
        .iter()
        .map(|cell| cell.symbol())
        .collect::<Vec<_>>()
        .join("")
}
