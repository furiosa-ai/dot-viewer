use crate::app::App;
use tui::{
    backend::Backend,
    layout::{ Alignment, Constraint, Direction, Layout, Rect },
    style::{ Color, Modifier, Style },
    text::{ Span, Spans },
    widgets::canvas::{ Canvas, Line, Map, MapResolution, Rectangle },
    widgets::{
        Axis, BarChart, Block, Borders, BorderType, Cell, Chart, Dataset, Gauge, LineGauge, List, ListItem,
        Paragraph, Row, Sparkline, Table, Tabs, Wrap,
    },
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
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ].as_ref()
        )
        .split(size);
    draw_list(f, chunks[0], app);
    draw_attrs(f, chunks[1], app);
}

fn draw_list<B: Backend>(f: &mut Frame<B>, chunk: Rect, app: &mut App) {
    let list: Vec<ListItem> = app
        .nodes
        .items
        .iter()
        .map(|i| ListItem::new(vec![Spans::from(Span::raw(i.as_str()))]))
        .collect();
    let list = List::new(list)
        .block(Block::default().borders(Borders::ALL).title("Nodes"))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");
    f.render_stateful_widget(list, chunk, &mut app.nodes.state);
}

fn draw_attrs<B: Backend>(f: &mut Frame<B>, chunk: Rect, app: &mut App) {
    if let Some(idx) = app.nodes.state.selected() {
        let id = &app.nodes.items[idx];
        if let Some(node) = app.graph.lookup(id) {
            let block = Block::default().borders(Borders::ALL).title("Attrs");
            let paragraph = Paragraph::new(node.to_string()).block(block).wrap(Wrap { trim: true });
            f.render_widget(paragraph, chunk);
        }
    }
}
