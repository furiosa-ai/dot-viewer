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
use dot_graph::structs::Node;
use crate::app::app::App;

pub fn selected(app: &App) -> Option<&Node> {
    match app.nodes.state.selected() {
        Some(idx) => {
            let id = &app.nodes.items[idx]; 
            app.graph.search(id)
        }
        None => None
    }
}

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
    draw_list(f, chunks[0], app);
    draw_node(f, chunks[1], app);
}

// node list (topologically sorted) block
fn draw_list<B: Backend>(f: &mut Frame<B>, chunk: Rect, app: &mut App) {
    let (froms, tos) = match selected(app) {
        Some(node) => (app.graph.froms(&node.id), app.graph.tos(&node.id)),
        None => (HashSet::new(), HashSet::new())
    };

    let list: Vec<ListItem> = app
        .nodes
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
        .block(Block::default().borders(Borders::ALL).title("Nodes"))
        .highlight_style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");
    f.render_stateful_widget(list, chunk, &mut app.nodes.state);
}

// node metadata block
fn draw_node<B: Backend>(f: &mut Frame<B>, chunk: Rect, app: &mut App) {
    // surrounding block
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Node Info");
    f.render_widget(block, chunk);

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
    draw_attrs(f, chunks[0], app);
    draw_edges(f, chunks[1], app);
}

// node attr block
fn draw_attrs<B: Backend>(f: &mut Frame<B>, chunk: Rect, app: &mut App) {
    let block = Block::default().borders(Borders::ALL).title("Attrs");

    if let Some(node) = selected(app) {
        let paragraph = Paragraph::new(node.to_string()).block(block).wrap(Wrap { trim: true });
        f.render_widget(paragraph, chunk);
    } else {
        f.render_widget(block, chunk);
    }
}

// adjacent nodes block
fn draw_edges<B: Backend>(f: &mut Frame<B>, chunk: Rect, app: &mut App) {
    // TODO remove this unnecessary block to prevent multiple mutable borrows
    let node = match selected(app) {
        Some(node) => Some(node.clone()),
        None => None,
    };

    if let Some(node) = node {
        // surrounding block
        let block = Block::default().borders(Borders::ALL).title("Edges");
        f.render_widget(block, chunk);

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
        draw_prevs(f, chunks[0], &node, app);
        draw_nexts(f, chunks[1], &node, app);
    }
}

// TODO modularize draw_prevs and draw_edges with impl in dot-graph
fn draw_prevs<B: Backend>(f: &mut Frame<B>, chunk: Rect, node: &Node, app: &mut App) {
    // surrounding block
    let block = Block::default().borders(Borders::ALL).title("Prev Nodes");

    let mut text = String::from("");
    let froms = app.graph.froms(&node.id); 
    for from in froms {
        text.push_str(from);
        text.push_str("\n");
    }

    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
    f.render_widget(paragraph, chunk);
}

// TODO modularize draw_prevs and draw_edges with impl in dot-graph
fn draw_nexts<B: Backend>(f: &mut Frame<B>, chunk: Rect, node: &Node, app: &mut App) {
    // surrounding block
    let block = Block::default().borders(Borders::ALL).title("Next Nodes");

    let mut text = String::from("");
    let tos = app.graph.tos(&node.id); 
    for to in tos {
        text.push_str(to);
        text.push_str("\n");
    }

    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
    f.render_widget(paragraph, chunk);
}
