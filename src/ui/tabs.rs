use crate::ui::view::draw_view;
use crate::viewer::App;

use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Tabs},
    Frame,
};

pub(super) fn draw_tabs<B: Backend>(
    f: &mut Frame<B>,
    chunk: Rect,
    app: &mut App,
) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(chunk);

    draw_nav_bar(f, chunks[0], app);

    let view = app.tabs.selected();
    draw_view(f, chunks[1], view);
}

fn draw_nav_bar<B: Backend>(f: &mut Frame<B>, chunk: Rect, app: &mut App) {
    let block = Block::default().borders(Borders::ALL).title("Views");

    let titles: Vec<String> = app.tabs.tabs.iter().map(|view| view.title.clone()).collect();
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
