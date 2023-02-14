mod app;
mod input;
mod popup;
mod tabs;
mod utils;
mod view;

use tui::{
    style::{Color, Style},
    widgets::{Block, Borders},
};

pub(crate) use crate::ui::app::draw_app;

pub(super) fn surrounding_block(title: String, highlight: bool) -> Block<'static> {
    let color = if highlight { Color::Yellow } else { Color::White };

    Block::default().borders(Borders::ALL).border_style(Style::default().fg(color)).title(title)
}
