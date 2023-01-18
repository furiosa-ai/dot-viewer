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
use crate::app::app::{ App, Lists, Mode, Focus };

// viewer (main) block
pub fn draw_viewer<B: Backend>(f: &mut Frame<B>, chunk: Rect, app: &mut App) {
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
    match app.mode {
        Mode::Traverse => {
            draw_lists(f, chunks[0], &mut app.lists);
            draw_metadata(f, chunks[1], &mut app.lists);
        },
        Mode::Search => draw_result(f, chunk, &mut app.lists),
    } 
}

// node list (topologically sorted) block
fn draw_lists<B: Backend>(f: &mut Frame<B>, chunk: Rect, lists: &mut Lists) {
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
    draw_nodes(f, chunks[0], lists);
    draw_edges(f, chunks[1], lists);
}

fn draw_highlighted_block(current: Focus, expected: Focus, title: String) -> Block<'static> {
    let color = if current == expected { Color::Yellow } else { Color::White };

    Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(color))
        .title(title)
}

fn draw_nodes<B: Backend>(f: &mut Frame<B>, chunk: Rect, lists: &mut Lists) {
    // surrounding block 
    let title = {
        let idx = lists.idx().unwrap();
        let len = lists.count();
        let percentage = (idx as f32 / len as f32) * 100 as f32;
        format!("Nodes [{} / {} ({:.3}%)]", idx, len, percentage)
    };
    let block = draw_highlighted_block(lists.focus.clone(), Focus::Current, title);

    let (froms, tos) = match &lists.current() {
        Some(id) => (lists.graph.froms(&id).clone(), lists.graph.tos(&id)),
        None => (HashSet::new(), HashSet::new())
    };

    let list: Vec<ListItem> = lists
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

    f.render_stateful_widget(list, chunk, &mut lists.current.state);
}

// adjacent nodes block
fn draw_edges<B: Backend>(f: &mut Frame<B>, chunk: Rect, lists: &mut Lists) {
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
    draw_prevs(f, chunks[0], lists);
    draw_nexts(f, chunks[1], lists);
}

// TODO modularize draw_prevs and draw_edges with impl in dot-graph
fn draw_prevs<B: Backend>(f: &mut Frame<B>, chunk: Rect, lists: &mut Lists) {
    // surrounding block
    let block = draw_highlighted_block(lists.focus.clone(), Focus::Prevs, "Prev Nodes".to_string());

    let list: Vec<ListItem> = lists
        .prevs
        .items
        .iter()
        .map(|id| ListItem::new(vec![Spans::from(Span::raw(id.as_str()))]))
        .collect();
    
    let list = List::new(list)
        .block(block)
        .highlight_style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");
    
    f.render_stateful_widget(list, chunk, &mut lists.prevs.state);
}

// TODO modularize draw_prevs and draw_edges with impl in dot-graph
fn draw_nexts<B: Backend>(f: &mut Frame<B>, chunk: Rect, lists: &mut Lists) {
    // surrounding block
    let block = draw_highlighted_block(lists.focus.clone(), Focus::Nexts, "Next Nodes".to_string());

    let list: Vec<ListItem> = lists
        .nexts
        .items
        .iter()
        .map(|id| ListItem::new(vec![Spans::from(Span::raw(id.as_str()))]))
        .collect();

    let list = List::new(list)
        .block(block)
        .highlight_style(Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");

    f.render_stateful_widget(list, chunk, &mut lists.nexts.state);
}

// node attr block
fn draw_metadata<B: Backend>(f: &mut Frame<B>, chunk: Rect, lists: &mut Lists) {
    // surrounding block
    let block = Block::default().borders(Borders::ALL).title("Attrs");

    if let Some(id) = lists.current() {
        let node = lists.graph.search(&id).unwrap(); 
        let paragraph = Paragraph::new(node.to_string()).block(block).wrap(Wrap { trim: true });
        f.render_widget(paragraph, chunk);
    } else {
        f.render_widget(block, chunk);
    }
}

// search result block
fn draw_result<B: Backend>(f: &mut Frame<B>, chunk: Rect, lists: &mut Lists) {
    // surrounding block
    let block = draw_highlighted_block(lists.focus.clone(), Focus::Nexts, "Searching...".to_string());

    let list: Vec<ListItem> = lists
        .search
        .items
        .iter()
        .map(|id| ListItem::new(vec![Spans::from(Span::raw(id.as_str()))]))
        .collect();

    let list = List::new(list)
        .block(block);

    f.render_stateful_widget(list, chunk, &mut lists.search.state);
}
