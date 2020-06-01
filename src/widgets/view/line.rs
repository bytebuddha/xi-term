use tui::layout::Rect;
use tui::buffer::Buffer;
use tui::widgets::Widget;
use xrl::Line;

use super::Chunk;
use core::u32_to_color;
use components::View;
use components::Editor;

pub struct LineWidget<'a, 'b, 'c> {
    editor: &'b Editor,
    _view: &'c View,
    line: Option<&'a Line>
}

impl <'a, 'b, 'c>LineWidget<'a, 'b, 'c> {

    pub fn new(editor: &'b Editor, _view: &'c View) -> LineWidget<'a, 'b, 'c> {
        LineWidget { editor, _view, line: None }
    }

    pub fn line(mut self, line: &'a Line) -> LineWidget<'a, 'b, 'c> {
        self.line = Some(line);
        self
    }
}

impl <'a, 'b, 'c>Widget for LineWidget<'a, 'b, 'c> {

    fn render(self, area: Rect, buf: &mut Buffer) {
        if let Some(line) = self.line {
            let mut current_step = area.x as usize;
            for style_def in &line.styles {
                if let Some(style) = self.editor.styles.get(&style_def.style_id) {
                    let start = style_def.offset as usize + current_step;
                    let chunk_rect = Rect {
                        x: start as u16,
                        y: area.y,
                        width: style_def.length as u16,
                        height: area.height
                    };

                    Chunk::new(&line.text[start - area.x as usize..])
                            .background(style.bg_color.map(|item|u32_to_color(item)))
                            .foreground(style.fg_color.map(|item|u32_to_color(item)))
                            .theme(self.editor.theme.as_ref())
                            .italic(style.italic)
                            .underlined(style.underline)
                            .render(chunk_rect, buf);

                    current_step = start + style_def.length as usize;
                }
            }
        }
    }
}
