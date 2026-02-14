use iced::widget::{checkbox, column, container, row, text, text_editor, text_input};
use iced::{Element, Length, Theme};

use crate::app::App;
use crate::message::Message;
use crate::ui::dialog_button;

impl App {
    pub fn find_all_matches(&mut self) {
        self.find_matches.clear();
        self.current_match = None;

        if self.find_query.is_empty() {
            return;
        }

        let content_text = self.content.text();
        let (haystack, needle) = if self.case_sensitive {
            (content_text.clone(), self.find_query.clone())
        } else {
            (content_text.to_lowercase(), self.find_query.to_lowercase())
        };

        let mut start = 0;
        while let Some(pos) = haystack[start..].find(&needle) {
            let abs_pos = start + pos;
            let line = content_text[..abs_pos].matches('\n').count();
            let line_start = if line == 0 {
                0
            } else {
                content_text[..abs_pos].rfind('\n').unwrap() + 1
            };
            let col = abs_pos - line_start;
            self.find_matches.push((line, col));
            start = abs_pos + 1;
        }
    }

    pub fn navigate_to_match(&mut self, index: usize) {
        if let Some(&(line, col)) = self.find_matches.get(index) {
            self.current_match = Some(index);
            let end_col = col + self.find_query.len();
            self.content.move_to(text_editor::Cursor {
                position: text_editor::Position { line, column: col },
                selection: Some(text_editor::Position {
                    line,
                    column: end_col,
                }),
            });
        }
    }

    pub fn search_panel(&self) -> Element<'_, Message> {
        let match_info = if self.find_query.is_empty() {
            String::new()
        } else if self.find_matches.is_empty() {
            String::from("No matches")
        } else {
            let idx = self.current_match.map(|i| i + 1).unwrap_or(0);
            format!("{}/{}", idx, self.find_matches.len())
        };

        let line_count = self.content.line_count();

        container(
            column![
                row![
                    text("Find:").size(14).width(60),
                    text_input("Search...", &self.find_query)
                        .size(14)
                        .on_input(Message::FindQueryChanged)
                        .on_submit(Message::FindNext)
                        .width(Length::Fill),
                    text(match_info).size(12).width(80),
                    checkbox(self.case_sensitive).label("Aa").on_toggle(Message::ToggleCaseSensitive).size(14),
                    dialog_button("Find Next", Message::FindNext),
                    dialog_button("Find Prev", Message::FindPrevious),
                ]
                .spacing(6)
                .align_y(iced::Alignment::Center),
                row![
                    text("Replace:").size(14).width(60),
                    text_input("Replace with...", &self.replace_text)
                        .size(14)
                        .on_input(Message::ReplaceTextChanged)
                        .width(Length::Fill),
                    dialog_button("Replace", Message::ReplaceOne),
                    dialog_button("Replace All", Message::ReplaceAll),
                ]
                .spacing(6)
                .align_y(iced::Alignment::Center),
                row![
                    text(format!("Go To Line (1-{}):", line_count)).size(14),
                    text_input("Line number...", &self.goto_line)
                        .size(14)
                        .on_input(Message::GoToLineChanged)
                        .on_submit(Message::GoToLineSubmit)
                        .width(200),
                    dialog_button("Go", Message::GoToLineSubmit),
                    iced::widget::Space::new().width(Length::Fill),
                    dialog_button("X", Message::ClosePanel),
                ]
                .spacing(6)
                .align_y(iced::Alignment::Center),
            ]
            .spacing(4),
        )
        .padding([6, 8])
        .style(|theme: &Theme| container::Style {
            background: Some(theme.extended_palette().background.weak.color.into()),
            ..Default::default()
        })
        .into()
    }
}
