use crate::{
    ui::{surrounding_block, utils::htmlparser},
    viewer::{Focus, View},
};

use std::collections::{HashMap, HashSet};
use std::fmt::Write;

use dot_graph::Node;

use rayon::prelude::*;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, List, ListItem, Paragraph, Wrap},
    Frame,
};

pub(super) fn draw_view<B: Backend>(f: &mut Frame<B>, chunk: Rect, view: &mut View) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(35), Constraint::Percentage(65)].as_ref())
        .split(chunk);

    draw_left(f, chunks[0], view);
    draw_right(f, chunks[1], view);
}

fn draw_left<B: Backend>(f: &mut Frame<B>, chunk: Rect, view: &mut View) {
    if view.matches.items.is_empty() {
        draw_current(f, chunk, view);
    } else {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(99), Constraint::Percentage(1)].as_ref())
            .split(chunk);

        draw_current(f, chunks[0], view);
        draw_match(f, chunks[1], view);
    }
}

fn draw_right<B: Backend>(f: &mut Frame<B>, chunk: Rect, view: &mut View) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunk);

    draw_adjacent(f, chunks[0], view);
    draw_metadata(f, chunks[1], view);
}

fn draw_current<B: Backend>(f: &mut Frame<B>, chunk: Rect, view: &mut View) {
    let progress = view.progress_current();
    let title = format!("Nodes {progress}");
    let block = surrounding_block(title, view.focus == Focus::Current);

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
                    spans[idx].style =
                        Style::default().bg(Color::Rgb(120, 120, 120)).add_modifier(Modifier::BOLD);
                }
            }

            let mut item = ListItem::new(Spans(spans));

            if froms.contains(&id) {
                item = item.style(Style::default().fg(Color::Rgb(255, 150, 150)));
            } else if tos.contains(&id) {
                item = item.style(Style::default().fg(Color::Rgb(150, 150, 255)));
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

fn draw_match<B: Backend>(f: &mut Frame<B>, chunk: Rect, view: &mut View) {
    let title = if view.matches.items.is_empty() { String::new() } else { view.progress_matches() };
    let block = Block::default().title(title).title_alignment(Alignment::Right);

    f.render_widget(block, chunk);
}

fn draw_adjacent<B: Backend>(f: &mut Frame<B>, chunk: Rect, view: &mut View) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunk);

    draw_prevs(f, chunks[0], view);
    draw_nexts(f, chunks[1], view);
}

fn draw_prevs<B: Backend>(f: &mut Frame<B>, chunk: Rect, view: &mut View) {
    let block = surrounding_block("Prev Nodes".to_string(), view.focus == Focus::Prev);

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

fn draw_nexts<B: Backend>(f: &mut Frame<B>, chunk: Rect, view: &mut View) {
    let block = surrounding_block("Next Nodes".to_string(), view.focus == Focus::Next);

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

fn draw_metadata<B: Backend>(f: &mut Frame<B>, chunk: Rect, view: &mut View) {
    let block = surrounding_block("Attrs".to_string(), false);

    let id = view.current_id();
    let node = view.graph.search_node(&id).unwrap();

    let paragraph = Paragraph::new(pretty_metadata(node)).block(block).wrap(Wrap { trim: true });

    f.render_widget(paragraph, chunk);
}

fn pretty_metadata(node: &Node) -> String {
    let mut metadata = String::new();

    writeln!(metadata, "[{}]", node.id()).unwrap();
    writeln!(metadata).unwrap();

    let empty = String::new();
    let attrs = node.attrs();
    let attrs_label = attrs.get("label").unwrap_or(&empty);
    let attrs_label = htmlparser::parse(attrs_label);

    if attrs.is_empty() {
        for (key, value) in attrs {
            writeln!(metadata, "{} : {}", key, value).unwrap();
        }
    } else {
        for attr in attrs_label {
            if attr.starts_with("Input") {
                continue;
            }

            let vals = attr.split("\\l");
            for val in vals {
                writeln!(metadata, "{}", val).unwrap();
            }
        }
    }

    metadata
}
