use crate::app::app::App;
use crate::ui::{input::draw_input, tabs::draw_tabs};
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
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

    // inner blocks
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(90), Constraint::Percentage(10)].as_ref())
        .split(size);
    draw_tabs(f, chunks[0], app);
    draw_input(f, chunks[1], app);
}
