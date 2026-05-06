use std::io;

use crossterm::event::{self, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::text::{Line, Text};
use ratatui::widgets::{Block, Borders, Paragraph, Widget};
use ratatui::Terminal;

use crate::domain::{ReadinessState, StatusReport, VibeError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatusTuiModel {
    report: StatusReport,
    should_quit: bool,
}

impl StatusTuiModel {
    pub fn new(report: StatusReport) -> Self {
        Self {
            report,
            should_quit: false,
        }
    }

    pub fn handle_key(&mut self, key: StatusTuiKey) {
        if matches!(key, StatusTuiKey::Quit | StatusTuiKey::Escape) {
            self.should_quit = true;
        }
    }

    pub fn should_quit(&self) -> bool {
        self.should_quit
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusTuiKey {
    Quit,
    Escape,
    Other,
}

pub struct StatusTuiView;

impl StatusTuiView {
    pub fn render(model: &StatusTuiModel, frame: &mut ratatui::Frame<'_>) {
        let area = frame.area();
        frame.render_widget(status_widget(&model.report), area);
    }
}

pub fn render_status_to_buffer(report: &StatusReport, area: Rect) -> Buffer {
    let mut buffer = Buffer::empty(area);
    status_widget(report).render(area, &mut buffer);
    buffer
}

pub fn run_status_tui(report: StatusReport) -> Result<(), VibeError> {
    let mut terminal = TerminalGuard::enter()?;
    let mut model = StatusTuiModel::new(report);

    while !model.should_quit() {
        terminal
            .terminal
            .draw(|frame| StatusTuiView::render(&model, frame))
            .map_err(|error| {
                VibeError::StatusEvaluationFailed(format!("could not draw TUI: {error}"))
            })?;

        if let Event::Key(key) = event::read().map_err(|error| {
            VibeError::StatusEvaluationFailed(format!("could not read TUI input: {error}"))
        })? {
            model.handle_key(map_key_code(key.code));
        }
    }

    Ok(())
}

fn status_widget(report: &StatusReport) -> Paragraph<'_> {
    let aggregate_state = if report.is_ready() {
        "ready"
    } else {
        "missing"
    };
    let mut lines = vec![
        Line::from(format!("{} status", report.project_name)),
        Line::from(format!("overall: {aggregate_state}")),
        Line::from(""),
    ];

    lines.extend(report.checks.iter().map(|check| {
        let state = match check.state {
            ReadinessState::Ready => "ready",
            ReadinessState::Missing => "missing",
        };
        Line::from(format!("{}: {} - {}", check.name, state, check.detail))
    }));

    Paragraph::new(Text::from(lines)).block(
        Block::default()
            .title("vibe-sentinel")
            .borders(Borders::ALL),
    )
}

fn map_key_code(code: KeyCode) -> StatusTuiKey {
    match code {
        KeyCode::Char('q') | KeyCode::Char('Q') => StatusTuiKey::Quit,
        KeyCode::Esc => StatusTuiKey::Escape,
        _ => StatusTuiKey::Other,
    }
}

struct TerminalGuard {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
}

impl TerminalGuard {
    fn enter() -> Result<Self, VibeError> {
        enable_raw_mode().map_err(|error| {
            VibeError::StatusEvaluationFailed(format!("could not enable raw mode: {error}"))
        })?;

        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen).map_err(|error| {
            let _ = disable_raw_mode();
            VibeError::StatusEvaluationFailed(format!("could not enter alternate screen: {error}"))
        })?;

        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend).map_err(|error| {
            let _ = disable_raw_mode();
            VibeError::StatusEvaluationFailed(format!("could not create TUI terminal: {error}"))
        })?;

        Ok(Self { terminal })
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(self.terminal.backend_mut(), LeaveAlternateScreen);
        let _ = self.terminal.show_cursor();
    }
}
