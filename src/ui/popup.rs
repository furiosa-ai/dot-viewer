use crate::app::App;
use crate::ui::ui::surrounding_block;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

pub fn draw_popup<B: Backend>(f: &mut Frame<B>, size: Rect, _app: &mut App) {
    // surrounding block
    let block = surrounding_block("Popup".to_string(), false);
    
    let popup = centered_rect(70, 70, size);
    f.render_widget(block, popup);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}
