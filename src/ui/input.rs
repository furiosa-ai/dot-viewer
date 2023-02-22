use crate::ui::surrounding_block;
use crate::viewer::{App, Mode, SearchMode};

use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::Paragraph,
    Frame,
};

pub(super) fn draw_input<B: Backend>(f: &mut Frame<B>, chunk: Rect, app: &mut App) {
    let title = match &app.mode {
        Mode::Normal => "Normal",
        Mode::Command => "Command",
        Mode::Search(smode) => match smode {
            SearchMode::Fuzzy => "Fuzzy Search",
            SearchMode::Regex => "Regex Search",
        },
        _ => unreachable!(),
    };

    let block = surrounding_block(
        title.to_string(),
        matches!(app.mode, Mode::Command) || matches!(app.mode, Mode::Search(_)),
    );

    f.render_widget(block, chunk);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunk);

    draw_result(f, chunks[0], app);
    draw_form(f, chunks[1], app);
}

fn draw_result<B: Backend>(f: &mut Frame<B>, chunk: Rect, app: &mut App) {
    let (msg, color) = match &app.result {
        Ok(succ) => (succ.to_string(), Color::Green),
        Err(err) => (err.to_string(), Color::Red),
    };

    if !msg.is_empty() {
        let msg =
            Paragraph::new(msg).style(Style::default().fg(color).add_modifier(Modifier::BOLD));
        f.render_widget(msg, chunk);
    }
}

fn draw_form<B: Backend>(f: &mut Frame<B>, chunk: Rect, app: &mut App) {
    let input = Paragraph::new(app.input.key.clone()).style(match &app.mode {
        Mode::Normal => Style::default(),
        Mode::Command | Mode::Search(_) => Style::default().fg(Color::Yellow),
        _ => unreachable!(),
    });
    f.render_widget(input, chunk);

    // cursor
    match &app.mode {
        Mode::Normal => {}
        Mode::Command | Mode::Search(_) => f.set_cursor(chunk.x + app.input.cursor as u16, chunk.y),
        _ => unreachable!(),
    }
}
