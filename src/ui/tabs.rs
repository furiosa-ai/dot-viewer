use crate::app::{App, MainMode};
use crate::ui::viewer::draw_viewer;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Tabs},
    Frame,
};

// main block
pub fn draw_tabs<B: Backend>(f: &mut Frame<B>, chunk: Rect, mode: &MainMode, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(chunk);

    draw_nav(f, chunks[0], app);

    let viewer = app.tabs.selected();
    draw_viewer(f, chunks[1], mode, viewer);
}

pub fn draw_nav<B: Backend>(f: &mut Frame<B>, chunk: Rect, app: &mut App) {
    let block = Block::default().borders(Borders::ALL).title("Tabs");

    let titles: Vec<String> = app.tabs.tabs.iter().map(|viewer| viewer.title.clone()).collect();
    let titles = titles
        .iter()
        .map(|title| Spans::from(vec![Span::styled(title, Style::default().fg(Color::Yellow))]))
        .collect();

    let tabs = Tabs::new(titles)
        .block(block)
        .select(app.tabs.state)
        .highlight_style(Style::default().add_modifier(Modifier::BOLD).bg(Color::Black));

    f.render_widget(tabs, chunk)
}
