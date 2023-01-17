use std::collections::HashSet;
use tui::{
    backend::Backend,
    layout::{ Constraint, Direction, Layout, Rect },
    style::{ Color, Modifier, Style },
    text::{ Span, Spans },
    widgets::{
        Block, Borders, List, ListItem,
        Paragraph, Wrap,
    },
    Frame,
};
use crate::app::app::{ App, Mode, Focus };

// viewer (main) block
pub fn draw_viewer<B: Backend>(f: &mut Frame<B>, chunk: Rect, app: &mut App) {
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
        .split(chunk);
    draw_lists(f, chunks[0], app);
    draw_metadata(f, chunks[1], app);
}

// node list (topologically sorted) block
fn draw_lists<B: Backend>(f: &mut Frame<B>, chunk: Rect, app: &mut App) {
    // surrounding block
    let block = Block::default().borders(Borders::ALL).title("Graph Traversal");
    f.render_widget(block, chunk);

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
        .split(chunk);
    draw_nodes(f, chunks[0], app);
    draw_edges(f, chunks[1], app);
}

fn draw_nodes<B: Backend>(f: &mut Frame<B>, chunk: Rect, app: &mut App) {
    // surrounding block
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(match app.mode {
            Mode::Traverse(Focus::All) => Color::Yellow,
            _ => Color::White,
        }))
        .title("Nodes");

    let (froms, tos) = match app.all.selected() {
        Some(id) => (app.graph.froms(&id), app.graph.tos(&id)),
        None => (HashSet::new(), HashSet::new())
    };

    let list: Vec<ListItem> = app
        .all
        .items
        .iter()
        .map(|id| {
            let mut item = ListItem::new(vec![Spans::from(Span::raw(id.as_str()))]);
            if froms.contains(id.as_str()) {
                item = item.style(Style::default().fg(Color::Red));
            } else if tos.contains(id.as_str()) {
                item = item.style(Style::default().fg(Color::Blue));
            } 

            item
        })
        .collect();

    let list = List::new(list)
        .block(block)
        .highlight_style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");
    f.render_stateful_widget(list, chunk, &mut app.all.state);
}

// adjacent nodes block
fn draw_edges<B: Backend>(f: &mut Frame<B>, chunk: Rect, app: &mut App) {
    // surrounding block
    let block = Block::default().borders(Borders::ALL).title("Edges");
    f.render_widget(block, chunk);

    if let Some(id) = app.all.selected() {
        // inner blocks
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                ].as_ref()
            )
            .split(chunk);
        draw_prevs(f, chunks[0], &id, app);
        draw_nexts(f, chunks[1], &id, app);
    }
}

// TODO modularize draw_prevs and draw_edges with impl in dot-graph
fn draw_prevs<B: Backend>(f: &mut Frame<B>, chunk: Rect, id: &str, app: &mut App) {
    // surrounding block
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(match app.mode {
            Mode::Traverse(Focus::Prevs) => Color::Yellow,
            _ => Color::White,
        }))
        .title("Prev Nodes");

    let list: Vec<ListItem> = app
        .prevs
        .items
        .iter()
        .map(|id| ListItem::new(vec![Spans::from(Span::raw(id.as_str()))]))
        .collect();
    
    let list = List::new(list)
        .block(block)
        .highlight_style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");
    
    f.render_stateful_widget(list, chunk, &mut app.prevs.state);
}

// TODO modularize draw_prevs and draw_edges with impl in dot-graph
fn draw_nexts<B: Backend>(f: &mut Frame<B>, chunk: Rect, id: &str, app: &mut App) {
    // surrounding block
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(match app.mode {
            Mode::Traverse(Focus::Nexts) => Color::Yellow,
            _ => Color::White,
        }))
        .title("Next Nodes");

    let list: Vec<ListItem> = app
        .nexts
        .items
        .iter()
        .map(|id| ListItem::new(vec![Spans::from(Span::raw(id.as_str()))]))
        .collect();

    let list = List::new(list)
        .block(block)
        .highlight_style(Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");
    f.render_stateful_widget(list, chunk, &mut app.nexts.state);
}

// node attr block
fn draw_metadata<B: Backend>(f: &mut Frame<B>, chunk: Rect, app: &mut App) {
    // surrounding block
    let block = Block::default().borders(Borders::ALL).title("Attrs");

    if let Some(id) = app.all.selected() {
        let node = app.graph.search(&id).unwrap(); 
        let paragraph = Paragraph::new(node.to_string()).block(block).wrap(Wrap { trim: true });
        f.render_widget(paragraph, chunk);
    } else {
        f.render_widget(block, chunk);
    }
}


