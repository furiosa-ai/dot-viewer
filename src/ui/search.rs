use tui::{
    backend::Backend,
    layout::{ Constraint, Direction, Layout, Rect },
    style::{ Color, Modifier, Style },
    text::{ Span, Spans, Text },
    widgets::{ Block, Borders, Paragraph },
    Frame,
};
use crate::app::app::{ App, Mode };

// search block
pub fn draw_search<B: Backend>(f: &mut Frame<B>, chunk: Rect, app: &mut App) {
    // surrounding block
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(match app.mode {
            Mode::Search => Color::Yellow,
            _ => Color::White,
        }))
        .title("Search");
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
    draw_help(f, chunks[0], app);
    match app.mode {
        Mode::Traverse => draw_error(f, chunks[1], app),
        Mode::Search => draw_input(f, chunks[1], app)
    };
}

// help block
fn draw_help<B: Backend>(f: &mut Frame<B>, chunk: Rect, app: &mut App) {
    let (msg, style) = match app.mode {
        Mode::Traverse => (
            vec![
                Span::raw("Press "),
                Span::styled("q", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
                Span::raw(" to exit, "),
                Span::styled("/", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
                Span::raw(" to start search, "),
                Span::styled("tab", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
                Span::raw(" to navigate blocks."),

            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        Mode::Search => (
            vec![
                Span::raw("Press "),
                Span::styled("Esc", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
                Span::raw(" to stop search, "),
                Span::styled("Enter", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
                Span::raw(" to search"),
            ],
            Style::default(),
        ),
    };
    let mut text = Text::from(Spans::from(msg));
    text.patch_style(style);

    let help = Paragraph::new(text);
    f.render_widget(help, chunk);
}

// error block
fn draw_error<B: Backend>(f: &mut Frame<B>, chunk: Rect, app: &mut App) {
    if let Some(msg) = &app.errormsg {
        let msg = Paragraph::new(msg.to_string())
            .style(
                Style::default()
                    .fg(Color::Red)
                    .add_modifier(Modifier::BOLD)
            );
        f.render_widget(msg, chunk);
    }
}

// input block
fn draw_input<B: Backend>(f: &mut Frame<B>, chunk: Rect, app: &mut App) {
    let input = Paragraph::new(app.input.as_ref())
        .style(match app.mode {
            Mode::Traverse => Style::default(),
            Mode::Search => Style::default().fg(Color::Yellow),
        });
    f.render_widget(input, chunk);
    
    // cursor
    match app.mode {
        Mode::Traverse => {}
        Mode::Search => {
            f.set_cursor(
                chunk.x + app.input.len() as u16,
                chunk.y,
            )
        }
    }
}
