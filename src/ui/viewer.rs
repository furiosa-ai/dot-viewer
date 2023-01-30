use crate::{
    app::{Input, Navigate, Mode, Viewer},
    ui::utils::htmlparser::parse_html
};
use std::collections::HashSet;
use dot_graph::Node;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

// current tab
pub fn draw_viewer<B: Backend>(f: &mut Frame<B>, chunk: Rect, mode: &Mode, viewer: &mut Viewer) {
    // inner blocks
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(35), Constraint::Percentage(65)].as_ref())
        .split(chunk);
    draw_left(f, chunks[0], mode, viewer);
    draw_right(f, chunks[1], mode, viewer);
}

fn draw_left<B: Backend>(f: &mut Frame<B>, chunk: Rect, mode: &Mode, viewer: &mut Viewer) {
    match &mode {
        Mode::Navigate(_) => {
            draw_current(f, chunk, mode, viewer);
        }
        Mode::Input(input) => match input {
            Input::Search | Input::Regex => {
                draw_search_match(f, chunk, mode, viewer);
            }
            Input::Filter => {
                draw_filter_match(f, chunk, mode, viewer);
            }
        },
    }
}

fn draw_right<B: Backend>(f: &mut Frame<B>, chunk: Rect, mode: &Mode, viewer: &mut Viewer) {
    match &mode {
        Mode::Navigate(_) => {
            // inner blocks
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(chunk);
            draw_adjacent(f, chunks[0], mode, viewer);
            draw_metadata(f, chunks[1], mode, viewer);
        }
        Mode::Input(_) => {}
    }
}

fn draw_current<B: Backend>(f: &mut Frame<B>, chunk: Rect, mode: &Mode, viewer: &mut Viewer) {
    // surrounding block
    let title = viewer.progress_current();
    let block = draw_highlighted_block(mode.clone(), Mode::Navigate(Navigate::Current), title);

    let (froms, tos) = match &viewer.current() {
        Some(id) => (viewer.graph.froms(id).clone(), viewer.graph.tos(id)),
        None => (HashSet::new(), HashSet::new()),
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
        .highlight_style(
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ");

    f.render_stateful_widget(list, chunk, &mut viewer.current.state);
}

// adjacent nodes block
fn draw_adjacent<B: Backend>(f: &mut Frame<B>, chunk: Rect, mode: &Mode, viewer: &mut Viewer) {
    // inner blocks
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunk);
    draw_prevs(f, chunks[0], mode, viewer);
    draw_nexts(f, chunks[1], mode, viewer);
}

// TODO modularize draw_prevs and draw_edges with impl in dot-graph
fn draw_prevs<B: Backend>(f: &mut Frame<B>, chunk: Rect, mode: &Mode, viewer: &mut Viewer) {
    // surrounding block
    let block = draw_highlighted_block(
        mode.clone(),
        Mode::Navigate(Navigate::Prevs),
        "Prev Nodes".to_string(),
    );

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
fn draw_nexts<B: Backend>(f: &mut Frame<B>, chunk: Rect, mode: &Mode, viewer: &mut Viewer) {
    // surrounding block
    let block = draw_highlighted_block(
        mode.clone(),
        Mode::Navigate(Navigate::Nexts),
        "Next Nodes".to_string(),
    );

    let list: Vec<ListItem> = viewer
        .nexts
        .items
        .iter()
        .map(|id| ListItem::new(vec![Spans::from(Span::raw(id.as_str()))]))
        .collect();

    let list = List::new(list)
        .block(block)
        .highlight_style(
            Style::default()
                .fg(Color::Blue)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ");

    f.render_stateful_widget(list, chunk, &mut viewer.nexts.state);
}

// node attr block
fn draw_metadata<B: Backend>(f: &mut Frame<B>, chunk: Rect, _mode: &Mode, viewer: &mut Viewer) {
    // surrounding block
    let block = Block::default().borders(Borders::ALL).title("Attrs");

    if let Some(id) = viewer.current() {
        let node = viewer.graph.search(&id).unwrap();
        let paragraph = Paragraph::new(pretty_metadata(node))
            .block(block)
            .wrap(Wrap { trim: true });
        f.render_widget(paragraph, chunk);
    } else {
        f.render_widget(block, chunk);
    }
}

fn pretty_metadata(node: &Node) -> String {
    let mut metadata = "".to_string();

    metadata.push_str(&format!("[{}]\n\n", node.id));

    let empty = "".to_string();
    let attrs = node.attrs.get("label").unwrap_or(&empty);
    let attrs = parse_html(attrs);
    
    for attr in attrs {
        if attr.starts_with("Input") {
            continue;
        }

        let vals = attr.split("\\l");
        for val in vals {
            metadata.push_str(&format!("{}\n", val));
        }
    }

    metadata
}

// search result block
fn draw_search_match<B: Backend>(f: &mut Frame<B>, chunk: Rect, mode: &Mode, viewer: &mut Viewer) {
    // surrounding block
    let title = viewer.progress_search();
    let block = draw_highlighted_block(mode.clone(), Mode::Input(Input::Search), title);

    let list: Vec<ListItem> = viewer
        .search
        .items
        .iter()
        .map(|item| {
            let mut spans = Vec::new();
            let id = &item.0;
            let highlight = &item.1;
            for (idx, c) in id.chars().enumerate() {
                let span = if highlight.contains(&idx) {
                    Span::styled(
                        c.to_string(),
                        Style::default()
                            .bg(Color::Black)
                            .add_modifier(Modifier::BOLD),
                    )
                } else {
                    Span::raw(c.to_string())
                };

                spans.push(span);
            }

            ListItem::new(vec![Spans::from(spans)])
        })
        .collect();

    let list = List::new(list)
        .block(block)
        .highlight_style(
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ");

    f.render_stateful_widget(list, chunk, &mut viewer.search.state);
}

// search result block
fn draw_filter_match<B: Backend>(f: &mut Frame<B>, chunk: Rect, mode: &Mode, viewer: &mut Viewer) {
    // surrounding block
    let block = draw_highlighted_block(
        mode.clone(),
        Mode::Input(Input::Filter),
        "Filtering...".to_string(),
    );

    let list: Vec<ListItem> = viewer
        .filter
        .items
        .iter()
        .map(|item| {
            let mut spans = Vec::new();
            let id = &item.0;
            let highlight = &item.1;
            for (idx, c) in id.chars().enumerate() {
                let span = if highlight.contains(&idx) {
                    Span::styled(
                        c.to_string(),
                        Style::default()
                            .bg(Color::Black)
                            .add_modifier(Modifier::BOLD),
                    )
                } else {
                    Span::raw(c.to_string())
                };

                spans.push(span);
            }

            ListItem::new(vec![Spans::from(spans)])
        })
        .collect();

    let list = List::new(list)
        .block(block)
        .highlight_style(
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ");

    f.render_stateful_widget(list, chunk, &mut viewer.filter.state);
}

fn draw_highlighted_block(current: Mode, expected: Mode, title: String) -> Block<'static> {
    let color = if current == expected {
        Color::Yellow
    } else {
        Color::White
    };

    Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(color))
        .title(title)
}
