use crate::ui::surrounding_block;
use crate::viewer::{App, InputMode, MainMode, SearchMode};

use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::Paragraph,
    Frame,
};

// input block
pub(super) fn draw_input<B: Backend>(
    f: &mut Frame<B>,
    chunk: Rect,
    mmode: &MainMode,
    app: &mut App,
) {
    // surrounding block
    let title = match mmode {
        MainMode::Navigate(_) => "Navigate",
        MainMode::Input(imode) => match imode {
            InputMode::Search(smode) => match smode {
                SearchMode::Fuzzy => "Fuzzy Search",
                SearchMode::Regex => "Regex Search",
            },
            InputMode::Filter => "Filter",
        },
    };

    let block = surrounding_block(title.to_string(), matches!(mmode, MainMode::Input(_)));

    f.render_widget(block, chunk);

    // inner blocks
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunk);
    draw_help(f, chunks[0]);
    match mmode {
        MainMode::Navigate(_) => draw_error(f, chunks[1], app),
        MainMode::Input(_) => draw_form(f, chunks[1], mmode, app),
    };
}

// help block
fn draw_help<B: Backend>(f: &mut Frame<B>, chunk: Rect) {
    let text = vec![
        Span::raw("Press "),
        Span::styled("?", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
        Span::raw(" for help. "),
        Span::raw("Press "),
        Span::styled("q", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
        Span::raw(" to exit"),
    ];
    let mut text = Text::from(Spans::from(text));
    text.patch_style(Style::default().add_modifier(Modifier::RAPID_BLINK));

    let help = Paragraph::new(text);
    f.render_widget(help, chunk);
}

// error block
fn draw_error<B: Backend>(f: &mut Frame<B>, chunk: Rect, app: &mut App) {
    let msg = match &app.result {
        Ok(succ) => succ.to_string(),
        Err(err) => err.to_string(),
    };

    if !msg.is_empty() {
        let msg =
            Paragraph::new(msg).style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD));
        f.render_widget(msg, chunk);
    }
}

// input block
fn draw_form<B: Backend>(f: &mut Frame<B>, chunk: Rect, mmode: &MainMode, app: &mut App) {
    let input = Paragraph::new(app.input.key.clone()).style(match mmode {
        MainMode::Navigate(_) => Style::default(),
        MainMode::Input(_) => Style::default().fg(Color::Yellow),
    });
    f.render_widget(input, chunk);

    // cursor
    match mmode {
        MainMode::Navigate(_) => {}
        MainMode::Input(_) => f.set_cursor(chunk.x + app.input.cursor as u16, chunk.y),
    }
}
