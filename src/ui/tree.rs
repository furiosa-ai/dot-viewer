use crate::viewer::App;
use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders},
    Frame,
};
use tui_tree_widget::Tree as TUITree;

pub(super) fn draw_tree<B: Backend>(f: &mut Frame<B>, chunk: Rect, app: &mut App) {
    let view = app.tabs.selected();
    let subtree = &mut view.subtree;

    let tree = TUITree::new(subtree.tree.clone())
        .block(Block::default().borders(Borders::ALL))
        .highlight_style(
            Style::default().fg(Color::Black).bg(Color::LightGreen).add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    f.render_stateful_widget(tree, chunk, &mut subtree.state);
}
