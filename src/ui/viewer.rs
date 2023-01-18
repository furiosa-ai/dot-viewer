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
use crate::app::app::{ App, Ctxt, Mode, Focus };

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
    draw_ctxt(f, chunks[0], &mut app.ctxt);
    match app.mode {
        Mode::Traverse => draw_metadata(f, chunks[1], &mut app.ctxt),
        Mode::Search => draw_result(f, chunks[1], &mut app.ctxt),
    } 
}

// node list (topologically sorted) block
fn draw_ctxt<B: Backend>(f: &mut Frame<B>, chunk: Rect, ctxt: &mut Ctxt) {
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
    draw_nodes(f, chunks[0], ctxt);
    draw_edges(f, chunks[1], ctxt);
}

fn draw_highlighted_block(current: Focus, expected: Focus, title: String) -> Block<'static> {
    let color = if current == expected { Color::Yellow } else { Color::White };

    Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(color))
        .title(title)
}

fn draw_nodes<B: Backend>(f: &mut Frame<B>, chunk: Rect, ctxt: &mut Ctxt) {
    // surrounding block 
    let title = {
        let idx = ctxt.idx().unwrap();
        let len = ctxt.count();
        let percentage = (idx as f32 / len as f32) * 100 as f32;
        format!("Nodes [{} / {} ({:.3}%)]", idx, len, percentage)
    };
    let block = draw_highlighted_block(ctxt.focus.clone(), Focus::Current, title);

    let (froms, tos) = match &ctxt.current() {
        Some(id) => (ctxt.graph.froms(&id).clone(), ctxt.graph.tos(&id)),
        None => (HashSet::new(), HashSet::new())
    };

    let list: Vec<ListItem> = ctxt
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

    f.render_stateful_widget(list, chunk, &mut ctxt.current.state);
}

// adjacent nodes block
fn draw_edges<B: Backend>(f: &mut Frame<B>, chunk: Rect, ctxt: &mut Ctxt) {
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
    draw_prevs(f, chunks[0], ctxt);
    draw_nexts(f, chunks[1], ctxt);
}

// TODO modularize draw_prevs and draw_edges with impl in dot-graph
fn draw_prevs<B: Backend>(f: &mut Frame<B>, chunk: Rect, ctxt: &mut Ctxt) {
    // surrounding block
    let block = draw_highlighted_block(ctxt.focus.clone(), Focus::Prevs, "Prev Nodes".to_string());

    let list: Vec<ListItem> = ctxt
        .prevs
        .items
        .iter()
        .map(|id| ListItem::new(vec![Spans::from(Span::raw(id.as_str()))]))
        .collect();
    
    let list = List::new(list)
        .block(block)
        .highlight_style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");
    
    f.render_stateful_widget(list, chunk, &mut ctxt.prevs.state);
}

// TODO modularize draw_prevs and draw_edges with impl in dot-graph
fn draw_nexts<B: Backend>(f: &mut Frame<B>, chunk: Rect, ctxt: &mut Ctxt) {
    // surrounding block
    let block = draw_highlighted_block(ctxt.focus.clone(), Focus::Nexts, "Next Nodes".to_string());

    let list: Vec<ListItem> = ctxt
        .nexts
        .items
        .iter()
        .map(|id| ListItem::new(vec![Spans::from(Span::raw(id.as_str()))]))
        .collect();

    let list = List::new(list)
        .block(block)
        .highlight_style(Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");

    f.render_stateful_widget(list, chunk, &mut ctxt.nexts.state);
}

// node attr block
fn draw_metadata<B: Backend>(f: &mut Frame<B>, chunk: Rect, ctxt: &mut Ctxt) {
    // surrounding block
    let block = Block::default().borders(Borders::ALL).title("Attrs");

    if let Some(id) = ctxt.current() {
        let node = ctxt.graph.search(&id).unwrap(); 
        let paragraph = Paragraph::new(node.to_string()).block(block).wrap(Wrap { trim: true });
        f.render_widget(paragraph, chunk);
    } else {
        f.render_widget(block, chunk);
    }
}

// search result block
fn draw_result<B: Backend>(f: &mut Frame<B>, chunk: Rect, ctxt: &mut Ctxt) {
    // surrounding block
    let block = draw_highlighted_block(ctxt.focus.clone(), Focus::Search, "Searching...".to_string());

    let list: Vec<ListItem> = ctxt
        .search
        .items
        .iter()
        .map(|id| ListItem::new(vec![Spans::from(Span::raw(id.as_str()))]))
        .collect();

    let list = List::new(list)
        .block(block)
        .highlight_style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");

    f.render_stateful_widget(list, chunk, &mut ctxt.search.state);
}
