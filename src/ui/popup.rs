use crate::app::App;
use crate::ui::ui::surrounding_block;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders},
    Frame,
};
use tui_tree_widget::Tree as TUITree;

pub fn draw_popup<B: Backend>(f: &mut Frame<B>, size: Rect, app: &mut App) {
    // surrounding block
    let block = surrounding_block("Filter by Subgraph".to_string(), false);

    let popup = centered_rect(70, 70, size);

    draw_tree(f, popup, app);

    f.render_widget(block, popup);
}

fn draw_tree<B: Backend>(f: &mut Frame<B>, chunk: Rect, app: &mut App) {
    let viewer = app.tabs.selected();
    let tree = &mut viewer.tree;

    let widget = TUITree::new(tree.tree.clone())
        .block(
            Block::default().borders(Borders::ALL).title(format!("Tree Widget {:?}", tree.state)),
        )
        .highlight_style(
            Style::default().fg(Color::Black).bg(Color::LightGreen).add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    f.render_stateful_widget(widget, chunk, &mut tree.state);
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
