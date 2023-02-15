use crate::{
    ui::{surrounding_block, utils::htmlparser},
    viewer::{MainMode, NavMode, View},
};

use std::collections::{HashMap, HashSet};
use std::fmt::Write;

use dot_graph::Node;

use rayon::prelude::*;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{List, ListItem, Paragraph, Wrap},
    Frame,
};

pub(super) fn draw_view<B: Backend>(
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
    draw_current(f, chunk, mmode, view);
}

fn draw_right<B: Backend>(f: &mut Frame<B>, chunk: Rect, mmode: &MainMode, view: &mut View) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunk);

    draw_adjacent(f, chunks[0], mmode, view);
    draw_metadata(f, chunks[1], mmode, view);
}

fn draw_current<B: Backend>(f: &mut Frame<B>, chunk: Rect, mmode: &MainMode, view: &mut View) {
    let progress = view.progress_current();
    let title = format!("Nodes {progress}");
    let block = surrounding_block(title, *mmode == MainMode::Navigate(NavMode::Current));

    let froms: HashSet<&String> = HashSet::from_iter(&view.prevs.items);
    let tos: HashSet<&String> = HashSet::from_iter(&view.nexts.items);
    let mut matches = HashMap::new();
    for (idx, highlight) in &view.matches.items {
        matches.insert(*idx, highlight);
    }

    let list: Vec<ListItem> = view
        .current
        .items
        .par_iter()
        .enumerate()
        .map(|(idx, id)| {
            let mut spans: Vec<Span> = id.chars().map(|c| Span::raw(c.to_string())).collect();
            if let Some(&highlight) = matches.get(&idx) {
                for &idx in highlight {
                    spans[idx].style = Style::default().bg(Color::Black).add_modifier(Modifier::BOLD);
                }
            }

            let mut item = ListItem::new(Spans(spans));

            if froms.contains(&id) {
                item = item.style(Style::default().fg(Color::Red));
            } else if tos.contains(&id) {
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

fn draw_adjacent<B: Backend>(f: &mut Frame<B>, chunk: Rect, mmode: &MainMode, view: &mut View) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunk);

    draw_prevs(f, chunks[0], mmode, view);
    draw_nexts(f, chunks[1], mmode, view);
}

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

fn draw_metadata<B: Backend>(f: &mut Frame<B>, chunk: Rect, mmode: &MainMode, view: &mut View) {
    let block = surrounding_block("Attrs".to_string(), false);

    let id = match mmode {
        MainMode::Navigate(_) => Some(view.current_id()),
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
    let mut metadata = String::new();

    writeln!(metadata, "[{}]", node.id()).unwrap();
    writeln!(metadata).unwrap();

    let empty = String::new();
    let attrs = node.attrs().get("label").unwrap_or(&empty);
    let attrs = htmlparser::parse(attrs);

    for attr in attrs {
        if attr.starts_with("Input") {
            continue;
        }

        let vals = attr.split("\\l");
        for val in vals {
            writeln!(metadata, "{}", val).unwrap();
        }
    }

    metadata
}
