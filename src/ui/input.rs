use crate::app::app::{App, Input, Mode};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

// input block
pub fn draw_input<B: Backend>(f: &mut Frame<B>, chunk: Rect, app: &mut App) {
    // surrounding block
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(match app.mode {
            Mode::Input(_) => Color::Yellow,
            _ => Color::White,
        }))
        .title(match &app.mode {
            Mode::Navigate(_) => "Navigate",
            Mode::Input(input) => match input {
                Input::Search => "Search",
                Input::Filter => "Filter",
            },
        });
    f.render_widget(block, chunk);

    // inner blocks
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunk);
    draw_help(f, chunks[0], app);
    match app.mode {
        Mode::Navigate(_) => draw_error(f, chunks[1], app),
        Mode::Input(_) => draw_form(f, chunks[1], app),
    };
}

// search block
pub fn draw_search<B: Backend>(f: &mut Frame<B>, chunk: Rect, app: &mut App) {
    // surrounding block
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(match app.mode {
            Mode::Input(_) => Color::Yellow,
            _ => Color::White,
        }))
        .title("Search");
    f.render_widget(block, chunk);

    // inner blocks
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunk);
    draw_help(f, chunks[0], app);
    match app.mode {
        Mode::Navigate(_) => draw_error(f, chunks[1], app),
        Mode::Input(_) => draw_input(f, chunks[1], app),
    };
}

// help block
fn draw_help<B: Backend>(f: &mut Frame<B>, chunk: Rect, app: &mut App) {
    let (msg, style) = match &app.mode {
        Mode::Navigate(_) => (
            vec![
                Span::raw("Press "),
                Span::styled(
                    "q",
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                ),
                Span::raw(" to exit, "),
                Span::styled(
                    "/",
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                ),
                Span::raw(" to fuzzy search, "),
                Span::styled(
                    "f",
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                ),
                Span::raw(" to apply filter with prefix, "),
                Span::styled(
                    "e",
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                ),
                Span::raw(" to export the current subgraph, "),
                Span::styled(
                    "0-9",
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                ),
                Span::raw(" to export the current neighbors with specified degree, "),
                Span::styled(
                    "c",
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                ),
                Span::raw(" to close tab, and "),
                Span::styled(
                    "tab/backtab",
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                ),
                Span::raw(" to change tab."),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        Mode::Input(input) => match input {
            Input::Search => (
                vec![
                    Span::raw("Press "),
                    Span::styled(
                        "Esc",
                        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                    ),
                    Span::raw(" to stop search, "),
                    Span::styled(
                        "Enter",
                        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                    ),
                    Span::raw(" to goto selected node."),
                ],
                Style::default(),
            ),
            Input::Filter => (
                vec![
                    Span::raw("Press "),
                    Span::styled(
                        "Esc",
                        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                    ),
                    Span::raw(" to stop filter, "),
                    Span::styled(
                        "Enter",
                        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                    ),
                    Span::raw(" to apply filter."),
                ],
                Style::default(),
            ),
        },
    };
    let mut text = Text::from(Spans::from(msg));
    text.patch_style(style);

    let help = Paragraph::new(text);
    f.render_widget(help, chunk);
}

// error block
fn draw_error<B: Backend>(f: &mut Frame<B>, chunk: Rect, app: &mut App) {
    let msg = match &app.result {
        Ok(Some(msg)) => Some(msg.clone()),
        Err(err) => Some(format!("{}", err)),
        _ => None,
    };

    if let Some(msg) = msg {
        let msg =
            Paragraph::new(msg).style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD));
        f.render_widget(msg, chunk);
    }
}

// input block
fn draw_form<B: Backend>(f: &mut Frame<B>, chunk: Rect, app: &mut App) {
    let input = Paragraph::new(app.input.as_ref()).style(match app.mode {
        Mode::Navigate(_) => Style::default(),
        Mode::Input(_) => Style::default().fg(Color::Yellow),
    });
    f.render_widget(input, chunk);

    // cursor
    match app.mode {
        Mode::Navigate(_) => {}
        Mode::Input(_) => f.set_cursor(chunk.x + app.input.len() as u16, chunk.y),
    }
}
