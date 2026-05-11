//! Helix-style fuzzy file picker overlay.
//!
//! Rendered on top of the normal layout while `app.input_mode == FilePicker`.
//! The picker pulls its haystack from `app.file_picker.haystack` (a snapshot
//! of the current diff's file paths) and ranks matches via nucleo.

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Flex, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
};

use crate::app::App;
use crate::ui::styles;

pub fn render_file_picker(frame: &mut Frame, app: &mut App) {
    let Some(picker) = app.file_picker.as_mut() else {
        return;
    };

    let area = centered_rect(60, 60, frame.area());
    frame.render_widget(Clear, area);

    let theme = &app.theme;
    let block = Block::default()
        .title(" Files ")
        .borders(Borders::ALL)
        .style(styles::popup_style(theme))
        .border_style(styles::border_style(theme, true));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    // Layout: prompt (1 row), separator (1 row), list (rest), footer (1 row).
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .split(inner);

    // Prompt
    let prompt = Line::from(vec![
        Span::styled("> ", Style::default().fg(theme.border_focused)),
        Span::raw(picker.query.clone()),
    ]);
    frame.render_widget(Paragraph::new(prompt), chunks[0]);

    // Separator
    let sep = Paragraph::new(Line::from(Span::styled(
        "─".repeat(chunks[1].width as usize),
        Style::default().fg(theme.fg_secondary),
    )));
    frame.render_widget(sep, chunks[1]);

    // List
    let list_area = chunks[2];
    let visible_rows = list_area.height as usize;

    // Auto-scroll so selected row is in view.
    if picker.selected < picker.scroll_offset {
        picker.scroll_offset = picker.selected;
    } else if picker.selected >= picker.scroll_offset + visible_rows && visible_rows > 0 {
        picker.scroll_offset = picker.selected + 1 - visible_rows;
    }

    let mut lines: Vec<Line> = Vec::with_capacity(visible_rows);
    for (visible_idx, match_idx) in picker
        .matches
        .iter()
        .enumerate()
        .skip(picker.scroll_offset)
        .take(visible_rows)
    {
        let path = picker
            .haystack
            .get(*match_idx)
            .map(|s| s.as_str())
            .unwrap_or("");
        let is_selected = visible_idx == picker.selected;
        let pointer = if is_selected { "▌ " } else { "  " };
        let style = if is_selected {
            Style::default()
                .fg(theme.fg_primary)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(theme.fg_primary)
        };
        let pointer_style = if is_selected {
            Style::default().fg(theme.border_focused)
        } else {
            Style::default()
        };
        lines.push(Line::from(vec![
            Span::styled(pointer, pointer_style),
            Span::styled(path.to_string(), style),
        ]));
    }

    if picker.matches.is_empty() {
        lines.push(Line::from(Span::styled(
            "  (no matches)",
            Style::default().fg(theme.fg_secondary),
        )));
    }

    frame.render_widget(Paragraph::new(lines), list_area);

    // Footer
    let footer = Line::from(vec![Span::styled(
        format!(
            " {}/{}    Enter open · ↑↓ move · Esc cancel ",
            if picker.matches.is_empty() {
                0
            } else {
                picker.selected + 1
            },
            picker.matches.len(),
        ),
        Style::default().fg(theme.fg_secondary),
    )]);
    frame.render_widget(Paragraph::new(footer), chunks[3]);
}

fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}
