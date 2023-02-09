use crate::{
    app::{InputMode, MainMode, NavMode, SearchMode, View},
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
pub fn draw_view<B: Backend>(
    f: &mut Frame<B>,
    chunk: Rect,
    mmode: &MainMode,
    view: &mut View,
) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(35), Constraint::Percentage(65)].as_ref())
        .split(chunk);

    draw_left(f, chunks[0], mmode, view);
    draw_right(f, chunks[1], mmode, view);
}

fn draw_left<B: Backend>(f: &mut Frame<B>, chunk: Rect, mmode: &MainMode, view: &mut View) {
    match &mmode {
        MainMode::Navigate(_) => draw_current(f, chunk, mmode, view),
        MainMode::Input(imode) => draw_matches(f, chunk, imode, view),
    }
}

fn draw_right<B: Backend>(f: &mut Frame<B>, chunk: Rect, mmode: &MainMode, view: &mut View) {
    match &mmode {
        MainMode::Navigate(_) => {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(chunk);

            draw_adjacent(f, chunks[0], mmode, view);
            draw_metadata(f, chunks[1], mmode, view);
        }
        MainMode::Input(_) => draw_metadata(f, chunk, mmode, view),
    }
}

fn draw_current<B: Backend>(f: &mut Frame<B>, chunk: Rect, mmode: &MainMode, view: &mut View) {
    let title = format!("Nodes {}", view.progress_current());
    let block = surrounding_block(title, *mmode == MainMode::Navigate(NavMode::Current));

    let (froms, tos) = match &view.current_id() {
        Some(id) => {
            let froms = view.graph.froms(id).unwrap();
            let tos = view.graph.tos(id).unwrap();
            (froms, tos)
        }
        None => (HashSet::new(), HashSet::new()),
    };

    let list: Vec<ListItem> = view
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
        .highlight_style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");

    f.render_stateful_widget(list, chunk, &mut view.current.state);
}

// adjacent nodes block
fn draw_adjacent<B: Backend>(f: &mut Frame<B>, chunk: Rect, mmode: &MainMode, view: &mut View) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunk);

    draw_prevs(f, chunks[0], mmode, view);
    draw_nexts(f, chunks[1], mmode, view);
}

// TODO modularize draw_prevs and draw_edges with impl in dot-graph
fn draw_prevs<B: Backend>(f: &mut Frame<B>, chunk: Rect, mmode: &MainMode, view: &mut View) {
    let block =
        surrounding_block("Prev Nodes".to_string(), *mmode == MainMode::Navigate(NavMode::Prevs));

    let list: Vec<ListItem> = view
        .prevs
        .items
        .par_iter()
        .map(|id| ListItem::new(vec![Spans::from(Span::raw(id.as_str()))]))
        .collect();

    let list = List::new(list)
        .block(block)
        .highlight_style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");

    f.render_stateful_widget(list, chunk, &mut view.prevs.state);
}

// TODO modularize draw_prevs and draw_edges with impl in dot-graph
fn draw_nexts<B: Backend>(f: &mut Frame<B>, chunk: Rect, mmode: &MainMode, view: &mut View) {
    let block =
        surrounding_block("Next Nodes".to_string(), *mmode == MainMode::Navigate(NavMode::Nexts));

    let list: Vec<ListItem> = view
        .nexts
        .items
        .par_iter()
        .map(|id| ListItem::new(vec![Spans::from(Span::raw(id.as_str()))]))
        .collect();

    let list = List::new(list)
        .block(block)
        .highlight_style(Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");

    f.render_stateful_widget(list, chunk, &mut view.nexts.state);
}

// node attr block
fn draw_metadata<B: Backend>(f: &mut Frame<B>, chunk: Rect, mmode: &MainMode, view: &mut View) {
    let block = surrounding_block("Attrs".to_string(), false);

    let id = match mmode {
        MainMode::Navigate(_) => view.current_id(),
        MainMode::Input(_) => view.matched_id(),
    };

    if let Some(id) = id {
        let node = view.graph.search_node(&id).unwrap();
        let paragraph =
            Paragraph::new(pretty_metadata(node)).block(block).wrap(Wrap { trim: true });

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
fn draw_matches<B: Backend>(f: &mut Frame<B>, chunk: Rect, input: &InputMode, view: &mut View) {
    // surrounding block
    let title = match input {
        InputMode::Search(smode) => match smode {
            SearchMode::Fuzzy => "Fuzzy Searching...".to_string(),
            SearchMode::Regex => "Regex Searching...".to_string(),
        },
        InputMode::Filter => "Filtering...".to_string(),
    };
    let title = format!("{} {}", title, view.progress_matches());
    let block = surrounding_block(title, true);

    let list: Vec<ListItem> = view
        .matches
        .items
        .par_iter()
        .map(|(id, highlight)| {
            let mut spans: Vec<Span> = id.chars().map(|c| Span::raw(c.to_string())).collect();
            for &idx in highlight {
                spans[idx].style = Style::default().bg(Color::Black).add_modifier(Modifier::BOLD);
            }

            ListItem::new(Spans(spans))
        })
        .collect();

    let list = List::new(list)
        .block(block)
        .highlight_style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");

    f.render_stateful_widget(list, chunk, &mut view.matches.state);
}