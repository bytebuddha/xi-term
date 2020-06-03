use tui::layout::Rect;
use tui::buffer::Buffer;
use tui::widgets::Widget;

use components::Prompt;
use super::{ InputWidget, MessageBox };

pub struct PromptWidget<'a> {
    prompt: &'a Prompt
}

impl <'a>PromptWidget<'a> {
    pub fn new(prompt: &'a Prompt) -> PromptWidget<'a> {
        PromptWidget { prompt }
    }
}

impl <'a>Widget for PromptWidget<'a> {

    fn render(self, area: Rect, buf: &mut Buffer) {

        if let Some(msg) = &self.prompt.message {
//            let lines = msg.text.split("\n").collect::<Vec<&str>>();
            let mut lines = vec![];
            for original_line in msg.text.split("\n").collect::<Vec<&str>>() {
                for line in textwrap::wrap(original_line, area.width as usize - 2) {
                    lines.push(line);
                }
            }
            let line_count = lines.len() as u16;
            let y = area.y + area.height - 3 - line_count;
            let msg_box_rect = Rect { x: area.x, y, width: area.width, height: line_count + 2};
            MessageBox::new(&msg).wrapped(lines.iter().map(|item| item.to_string()).collect()).render(msg_box_rect, buf);
        }

        let input_rect = Rect { x: area.x, y: area.y + area.height - 1, width: area.width, height: 1};
        InputWidget::new(&self.prompt.chars).render(input_rect, buf);
    }
}
