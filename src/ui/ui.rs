use crate::app::{App, Mode};
use crate::ui::{input::draw_input, popup::draw_popup, tabs::draw_tabs};
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders},
    Frame,
};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let size = f.size();

    // surrounding block
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Dot Viewer (Dev)")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded);
    f.render_widget(block, size);

    match &app.mode {
        Mode::Navigate(_) | Mode::Input(_) => draw_app(f, size, app),
        Mode::Popup => draw_popup(f, size, app),
    }
}

pub fn draw_app<B: Backend>(f: &mut Frame<B>, size: Rect, app: &mut App) {
    // inner blocks
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(90), Constraint::Percentage(10)].as_ref())
        .split(size);
    draw_tabs(f, chunks[0], app);
    draw_input(f, chunks[1], app);
}

pub fn surrounding_block(title: String, highlight: bool) -> Block<'static> {
    let color = if highlight {
        Color::Yellow
    } else {
        Color::White
    };

    Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(color))
        .title(title)
}
