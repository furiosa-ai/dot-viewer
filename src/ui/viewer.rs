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
use crate::app::app::{ Viewer, Focus };

// current tab 
pub fn draw_viewer<B: Backend>(f: &mut Frame<B>, chunk: Rect, viewer: &mut Viewer) {
    // inner blocks
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(60),
                Constraint::Percentage(40),
            ].as_ref()
        )
        .split(chunk);
    draw_left(f, chunks[0], viewer);
    draw_right(f, chunks[1], viewer);
}

fn draw_left<B: Backend>(f: &mut Frame<B>, chunk: Rect, viewer: &mut Viewer) {
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
    draw_current(f, chunks[0], viewer);
    draw_adjacent(f, chunks[1], viewer);
}

fn draw_right<B: Backend>(f: &mut Frame<B>, chunk: Rect, viewer: &mut Viewer) {
    match viewer.focus {
        Focus::Search => draw_result(f, chunk, viewer),
        _ => draw_metadata(f, chunk, viewer)
    }
}

fn draw_highlighted_block(current: Focus, expected: Focus, title: String) -> Block<'static> {
    let color = if current == expected { Color::Yellow } else { Color::White };

    Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(color))
        .title(title)
}

fn draw_current<B: Backend>(f: &mut Frame<B>, chunk: Rect, viewer: &mut Viewer) {
    // surrounding block 
    let title = {
        let idx = viewer.idx().unwrap();
        let len = viewer.count();
        let percentage = (idx as f32 / len as f32) * 100 as f32;
        format!("Nodes [{} / {} ({:.3}%)]", idx, len, percentage)
    };
    let block = draw_highlighted_block(viewer.focus.clone(), Focus::Current, title);

    let (froms, tos) = match &viewer.current() {
        Some(id) => (viewer.graph.froms(&id).clone(), viewer.graph.tos(&id)),
        None => (HashSet::new(), HashSet::new())
    };

    let list: Vec<ListItem> = viewer 
        .current
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

    f.render_stateful_widget(list, chunk, &mut viewer.current.state);
}

// adjacent nodes block
fn draw_adjacent<B: Backend>(f: &mut Frame<B>, chunk: Rect, viewer: &mut Viewer) {
    // inner blocks
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ].as_ref()
        )
        .split(chunk);
    draw_prevs(f, chunks[0], viewer);
    draw_nexts(f, chunks[1], viewer);
}

// TODO modularize draw_prevs and draw_edges with impl in dot-graph
fn draw_prevs<B: Backend>(f: &mut Frame<B>, chunk: Rect, viewer: &mut Viewer) {
    // surrounding block
    let block = draw_highlighted_block(viewer.focus.clone(), Focus::Prevs, "Prev Nodes".to_string());

    let list: Vec<ListItem> = viewer 
        .prevs
        .items
        .iter()
        .map(|id| ListItem::new(vec![Spans::from(Span::raw(id.as_str()))]))
        .collect();
    
    let list = List::new(list)
        .block(block)
        .highlight_style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");
    
    f.render_stateful_widget(list, chunk, &mut viewer.prevs.state);
}

// TODO modularize draw_prevs and draw_edges with impl in dot-graph
fn draw_nexts<B: Backend>(f: &mut Frame<B>, chunk: Rect, viewer: &mut Viewer) {
    // surrounding block
    let block = draw_highlighted_block(viewer.focus.clone(), Focus::Nexts, "Next Nodes".to_string());

    let list: Vec<ListItem> = viewer 
        .nexts
        .items
        .iter()
        .map(|id| ListItem::new(vec![Spans::from(Span::raw(id.as_str()))]))
        .collect();

    let list = List::new(list)
        .block(block)
        .highlight_style(Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");

    f.render_stateful_widget(list, chunk, &mut viewer.nexts.state);
}

// node attr block
fn draw_metadata<B: Backend>(f: &mut Frame<B>, chunk: Rect, viewer: &mut Viewer) {
    // surrounding block
    let block = Block::default().borders(Borders::ALL).title("Attrs");

    if let Some(id) = viewer.current() {
        let node = viewer.graph.search(&id).unwrap(); 
        let paragraph = Paragraph::new(node.to_string()).block(block).wrap(Wrap { trim: true });
        f.render_widget(paragraph, chunk);
    } else {
        f.render_widget(block, chunk);
    }
}

// search result block
fn draw_result<B: Backend>(f: &mut Frame<B>, chunk: Rect, viewer: &mut Viewer) {
    // surrounding block
    let block = draw_highlighted_block(viewer.focus.clone(), Focus::Search, "Searching...".to_string());

    let list: Vec<ListItem> = viewer
        .search
        .items
        .iter()
        .map(|id| ListItem::new(vec![Spans::from(Span::raw(id.as_str()))]))
        .collect();

    let list = List::new(list)
        .block(block)
        .highlight_style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");

    f.render_stateful_widget(list, chunk, &mut viewer.search.state);
}
