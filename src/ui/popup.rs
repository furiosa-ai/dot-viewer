use crate::ui::{centered_rect, surrounding_block};
use crate::viewer::{App, PopupMode};
use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders},
    Frame,
};
use tui_tree_widget::Tree as TUITree;

pub(super) fn draw_popup<B: Backend>(f: &mut Frame<B>, size: Rect, pmode: &PopupMode, app: &mut App) { 
    let popup = centered_rect(70, 70, size);

    match pmode {
        PopupMode::Tree => draw_tree(f, popup, app),
        PopupMode::Help => draw_help(f, popup, app),
    };
}

fn draw_tree<B: Backend>(f: &mut Frame<B>, chunk: Rect, app: &mut App) {
    let block = surrounding_block("Filter by Subgraph".to_string(), false);

    let view = app.tabs.selected();
    let subtree = &mut view.subtree;

    let tree = TUITree::new(subtree.tree.clone())
        .block(Block::default().borders(Borders::ALL))
        .highlight_style(
            Style::default().fg(Color::Black).bg(Color::LightGreen).add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    f.render_stateful_widget(tree, chunk, &mut subtree.state);

    f.render_widget(block, chunk);
}

fn draw_help<B: Backend>(f: &mut Frame<B>, chunk: Rect, _app: &mut App) {
    let block = surrounding_block("Help".to_string(), false);

    f.render_widget(block, chunk);
}
