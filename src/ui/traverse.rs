use tui::{
    backend::Backend,
    layout::{ Constraint, Direction, Layout, Rect },
    style::{ Color, Modifier, Style },
    text::{ Span, Spans },
    widgets::{ Block, Borders, Tabs },
    Frame,
};
use crate::app::app::App;
use crate::ui::tab::draw_tab;

// main block
pub fn draw_traverse<B: Backend>(f: &mut Frame<B>, chunk: Rect, app: &mut App) {
    // inner blocks
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(chunk);
    draw_tabs(f, chunks[0], app);
    let tab = &mut app.ctxts[app.tab];
    draw_tab(f, chunks[1], tab);
}

pub fn draw_tabs<B: Backend>(f: &mut Frame<B>, chunk: Rect, app: &mut App) {
    // surrounding block
    let block = Block::default().borders(Borders::ALL).title("Tabs");

    let titles: Vec<String> = app.ctxts.iter().map(|ctxt| ctxt.title.clone()).collect();
    let titles = titles.iter()
        .map(|title| Spans::from(vec![
                Span::styled(title, Style::default().fg(Color::Yellow))
        ]))
        .collect();

    let tabs = Tabs::new(titles)
        .block(block)
        .select(app.tab)
        .highlight_style(Style::default().add_modifier(Modifier::BOLD).bg(Color::Black));

    f.render_widget(tabs, chunk)
}
