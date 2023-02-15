use crate::ui::surrounding_block;
use crate::viewer::{App, InputMode, MainMode, SearchMode};

use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
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
    let view = app.tabs.selected();

    // surrounding block
    let title = match mmode {
        MainMode::Navigate(_) => if view.matches.items.is_empty() {
            "Navigate".to_string()
        } else {
            format!("Navigate {}", view.progress_matches())
        }
        MainMode::Input(imode) => match imode {
            InputMode::Search(smode) => match smode {
                SearchMode::Fuzzy => if view.matches.items.is_empty() {
                    "Fuzzy Search".to_string()
                } else {
                    format!("Fuzzy Search {}", view.progress_matches())
                }
                SearchMode::Regex => if view.matches.items.is_empty() {
                    "Regex Search".to_string()
                } else {
                    format!("Regex Search {}", view.progress_matches())
                }
            },
            InputMode::Filter => "Filter".to_string(),
        },
    };

    let block = surrounding_block(title, matches!(mmode, MainMode::Input(_)));

    f.render_widget(block, chunk);

    // inner blocks
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunk);
    draw_error(f, chunks[0], app);
    draw_form(f, chunks[1], mmode, app);
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
