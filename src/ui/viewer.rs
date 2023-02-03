use crate::{
    app::{InputMode, MainMode, NavMode, SearchMode, Viewer},
    ui::{ui::surrounding_block, utils::htmlparser::parse_html},
};
use dot_graph::Node;
use rayon::prelude::*;
use std::collections::HashSet;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{List, ListItem, Paragraph, Wrap},
    Frame,
};

// current tab
pub fn draw_viewer<B: Backend>(f: &mut Frame<B>, chunk: Rect, mode: &MainMode, viewer: &mut Viewer) {
    // inner blocks
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(35), Constraint::Percentage(65)].as_ref())
        .split(chunk);
    draw_left(f, chunks[0], mode, viewer);
    draw_right(f, chunks[1], mode, viewer);
}

fn draw_left<B: Backend>(f: &mut Frame<B>, chunk: Rect, mode: &MainMode, viewer: &mut Viewer) {
    match &mode {
        MainMode::Navigate(_) => draw_current(f, chunk, mode, viewer),
        MainMode::Input(input) => draw_matches(f, chunk, input, viewer),
    }
}

fn draw_right<B: Backend>(f: &mut Frame<B>, chunk: Rect, mode: &MainMode, viewer: &mut Viewer) {
    match &mode {
        MainMode::Navigate(_) => {
            // inner blocks
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(chunk);
            draw_adjacent(f, chunks[0], mode, viewer);
            draw_metadata(f, chunks[1], mode, viewer);
        }
        MainMode::Input(_) => draw_metadata(f, chunk, mode, viewer),
    }
}

fn draw_current<B: Backend>(f: &mut Frame<B>, chunk: Rect, mode: &MainMode, viewer: &mut Viewer) {
    // surrounding block
    let title = format!("Nodes {}", viewer.progress_current());
    let block = surrounding_block(title, *mode == MainMode::Navigate(NavMode::Current));

    let (froms, tos) = match &viewer.current() {
        Some(id) => (viewer.graph.froms(id).clone(), viewer.graph.tos(id)),
        None => (HashSet::new(), HashSet::new()),
    };

    let list: Vec<ListItem> = viewer
        .current
        .items
        .par_iter()
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
fn draw_adjacent<B: Backend>(f: &mut Frame<B>, chunk: Rect, mode: &MainMode, viewer: &mut Viewer) {
    // inner blocks
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunk);
    draw_prevs(f, chunks[0], mode, viewer);
    draw_nexts(f, chunks[1], mode, viewer);
}

// TODO modularize draw_prevs and draw_edges with impl in dot-graph
fn draw_prevs<B: Backend>(f: &mut Frame<B>, chunk: Rect, mode: &MainMode, viewer: &mut Viewer) {
    // surrounding block
    let block = surrounding_block(
        "Prev Nodes".to_string(),
        *mode == MainMode::Navigate(NavMode::Prevs),
    );

    let list: Vec<ListItem> = viewer
        .prevs
        .items
        .par_iter()
        .map(|id| ListItem::new(vec![Spans::from(Span::raw(id.as_str()))]))
        .collect();

    let list = List::new(list)
        .block(block)
        .highlight_style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");

    f.render_stateful_widget(list, chunk, &mut viewer.prevs.state);
}

// TODO modularize draw_prevs and draw_edges with impl in dot-graph
fn draw_nexts<B: Backend>(f: &mut Frame<B>, chunk: Rect, mode: &MainMode, viewer: &mut Viewer) {
    // surrounding block
    let block = surrounding_block(
        "Next Nodes".to_string(),
        *mode == MainMode::Navigate(NavMode::Nexts),
    );

    let list: Vec<ListItem> = viewer
        .nexts
        .items
        .par_iter()
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
fn draw_metadata<B: Backend>(f: &mut Frame<B>, chunk: Rect, mode: &MainMode, viewer: &mut Viewer) {
    // surrounding block
    let block = surrounding_block("Attrs".to_string(), false);

    let id = match mode {
        MainMode::Navigate(_) => viewer.current(),
        MainMode::Input(_) => viewer.matched(),
    };

    if let Some(id) = id {
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

// match result block
fn draw_matches<B: Backend>(f: &mut Frame<B>, chunk: Rect, input: &InputMode, viewer: &mut Viewer) {
    // surrounding block
    let title = match input {
        InputMode::Search(search) => match search {
            SearchMode::Fuzzy => "Fuzzy Searching...".to_string(),
            SearchMode::Regex => "Regex Searching...".to_string(),
        },
        InputMode::Filter => "Filtering...".to_string(),
    };
    let title = format!("{} {}", title, viewer.progress_matches());
    let block = surrounding_block(title, true);

    let list: Vec<ListItem> = viewer
        .matches
        .items
        .par_iter()
        .map(|(id, highlight)| {
            let mut spans: Vec<Span> = id.chars().map(|c| Span::raw(c.to_string())).collect();
            for &idx in highlight {
                spans[idx].style = Style::default()
                    .bg(Color::Black)
                    .add_modifier(Modifier::BOLD);
            }

            ListItem::new(Spans(spans))
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

    f.render_stateful_widget(list, chunk, &mut viewer.matches.state);
}
