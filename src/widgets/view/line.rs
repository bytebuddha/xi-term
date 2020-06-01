use tui::layout::Rect;
use tui::buffer::Buffer;
use tui::widgets::Widget;
use tui::style::Color;
use xrl::Line;

use super::Chunk;
use components::View;
use components::Editor;

pub struct LineWidget<'a, 'b, 'c> {
    editor: &'b Editor,
    _view: &'c View,
    line: Option<&'a Line>
}

impl <'a, 'b, 'c>LineWidget<'a, 'b, 'c> {

    pub fn new(editor: &'b Editor, _view: &'c View, line: Option<&'a Line>) -> LineWidget<'a, 'b, 'c> {
        LineWidget { editor, _view, line }
    }
}

fn get_color(argb_color: u32) -> Color {
    let r = ((argb_color & 0x00ff_0000) >> 16) as u8;
    let g = ((argb_color & 0x0000_ff00) >> 8) as u8;
    let b = (argb_color & 0x0000_00ff) as u8;
    Color::Rgb(r, g, b)
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

                    Chunk::new(&line.text[start..])
                            .background(style.bg_color.map(|item|get_color(item)))
                            .foreground(style.fg_color.map(|item|get_color(item)))
                            .italic(style.italic)
                            .underlined(style.underline)
                            .render(chunk_rect, buf);

                    current_step = start + style_def.length as usize;
                }
            }
        } else {
            buf.set_background(area, tui::style::Style::default().bg);
        }
    }
}
