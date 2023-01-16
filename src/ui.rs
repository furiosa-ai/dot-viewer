use crate::app::{ App, Mode };
use tui::{
    backend::Backend,
    layout::{ Alignment, Constraint, Direction, Layout, Rect },
    style::{ Color, Modifier, Style },
    text::{ Span, Spans, Text },
    widgets::{
        Block, Borders, BorderType, List, ListItem,
        Paragraph, Wrap,
    },
    Frame,
};
use dot_graph::structs::Node;

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let size = f.size();

    // surrounding block
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Dot Viewer (Dev)")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded);
    f.render_widget(block, size);

    // inner blocks
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(95),
                Constraint::Percentage(5),
            ].as_ref()
        )
        .split(size);
    draw_viewer(f, chunks[0], app);
    draw_command(f, chunks[1], app);
}

fn draw_command<B: Backend>(f: &mut Frame<B>, chunk: Rect, app: &mut App) {
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
    draw_help(f, chunks[0], app);
    draw_input(f, chunks[1], app);
}

fn draw_help<B: Backend>(f: &mut Frame<B>, chunk: Rect, app: &mut App) {
    let (msg, style) = match app.mode {
        Mode::Normal => (
            vec![
                Span::raw("Press "),
                Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to exit, "),
                Span::styled("!", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to start editing."),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        Mode::Command => (
            vec![
                Span::raw("Press "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to stop editing, "),
                Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to fire the command"),
            ],
            Style::default(),
        ),
    };

    let mut text = Text::from(Spans::from(msg));
    text.patch_style(style);

    let help = Paragraph::new(text);
    f.render_widget(help, chunk);
}

fn draw_input<B: Backend>(f: &mut Frame<B>, chunk: Rect, app: &mut App) {
    let input = Paragraph::new(app.command.as_ref())
        .style(match app.mode {
            Mode::Normal => Style::default(),
            Mode::Command => Style::default().fg(Color::Yellow),
        });
    f.render_widget(input, chunk);
    match app.mode {
        Mode::Normal => {}
        Mode::Command => {
            f.set_cursor(
                chunk.x + app.command.len() as u16,
                chunk.y,
            )
        }
    }
}

fn draw_viewer<B: Backend>(f: &mut Frame<B>, chunk: Rect, app: &mut App) {
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

fn draw_list<B: Backend>(f: &mut Frame<B>, chunk: Rect, app: &mut App) {
    let list: Vec<ListItem> = app
        .nodes
        .items
        .iter()
        .map(|i| ListItem::new(vec![Spans::from(Span::raw(i.as_str()))]))
        .collect();
    let list = List::new(list)
        .block(Block::default().borders(Borders::ALL).title("Nodes"))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");
    f.render_stateful_widget(list, chunk, &mut app.nodes.state);
}

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

fn draw_attrs<B: Backend>(f: &mut Frame<B>, chunk: Rect, app: &mut App) {
    let block = Block::default().borders(Borders::ALL).title("Attrs");

    if let Some(node) = selected(app) {
        let paragraph = Paragraph::new(node.to_string()).block(block).wrap(Wrap { trim: true });
        f.render_widget(paragraph, chunk);
    } else {
        f.render_widget(block, chunk);
    }
}

fn draw_edges<B: Backend>(f: &mut Frame<B>, chunk: Rect, app: &mut App) {
    // TODO unnecessary block to prevent multiple mutable borrows
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

fn draw_prevs<B: Backend>(f: &mut Frame<B>, chunk: Rect, node: &Node, app: &mut App) {
    // surrounding block
    let block = Block::default().borders(Borders::ALL).title("Prev Nodes");

    let idx = app.graph.lookup.get_by_left(&node.id).unwrap();
    if let Some(froms) = app.graph.bwdmap.get(idx) {
        let mut text = String::from("");
        for from in froms {
            let from = app.graph.lookup.get_by_right(from).unwrap();
            text.push_str(from);
            text.push_str("\n");
        }

        let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
        f.render_widget(paragraph, chunk);
    } else {
        f.render_widget(block, chunk);
    }
}

fn draw_nexts<B: Backend>(f: &mut Frame<B>, chunk: Rect, node: &Node, app: &mut App) {
    // surrounding block
    let block = Block::default().borders(Borders::ALL).title("Next Nodes");

    let idx = app.graph.lookup.get_by_left(&node.id).unwrap();
    if let Some(tos) = app.graph.fwdmap.get(idx) {
        let mut text = String::from("");
        for to in tos {
            let to = app.graph.lookup.get_by_right(to).unwrap();
            text.push_str(to);
            text.push_str("\n");
        }

        let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
        f.render_widget(paragraph, chunk);
    } else {
        f.render_widget(block, chunk);
    }
}

fn selected(app: &App) -> Option<&Node> {
    match app.nodes.state.selected() {
        Some(idx) => {
            let id = &app.nodes.items[idx]; 
            app.graph.lookup(id)
        }
        None => None
    }
}
