use tui::layout::Rect;
use tui::buffer::Buffer;
use tui::widgets::Widget;
use xrl::{ Line, Style, ThemeChanged };

use std::collections::HashMap;

use super::Chunk;
use core::u32_to_color;
use components::View;

pub struct LineWidget<'a, 'b, 'c, 'd> {
    styles: &'b HashMap<u64, Style>,
    theme: Option<&'c ThemeChanged>,
    _view: &'d View,
    line: Option<&'a Line>
}

impl <'a, 'b, 'c, 'd>LineWidget<'a, 'b, 'c, 'd> {

    pub fn new(styles: &'b HashMap<u64, Style>, _view: &'d View) -> LineWidget<'a, 'b, 'c, 'd> {
        LineWidget { styles, _view, line: None, theme: None }
    }

    pub fn line(mut self, line: &'a Line) -> LineWidget<'a, 'b, 'c, 'd> {
        self.line = Some(line);
        self
    }

    pub fn theme(mut self, theme: Option<&'c ThemeChanged>) -> LineWidget<'a, 'b, 'c, 'd> {
        self.theme = theme;
        self
    }
}

impl <'a, 'b, 'c, 'd>Widget for LineWidget<'a, 'b, 'c, 'd> {

    fn render(self, area: Rect, buf: &mut Buffer) {
        if let Some(line) = self.line {
            let mut current_step = area.x as i64;
            error!("Rect: {:?}", area);
            for style_def in &line.styles {
                if let Some(style) = self.styles.get(&style_def.style_id) {
                    let start = style_def.offset + current_step;
                    let chunk_rect = Rect {
                        x: start as u16,
                        y: area.y,
                        width: style_def.length as u16,
                        height: area.height
                    };

                    Chunk::new(&line.text[(start - area.x as i64) as usize..])
                        .background(style.bg_color.map(|item|u32_to_color(item)))
                        .foreground(style.fg_color.map(|item|u32_to_color(item)))
                        .theme(self.theme)
                        .italic(style.italic)
                        .underlined(style.underline)
                        .render(chunk_rect, buf);
                    current_step = start + style_def.length as i64;
                }
            }
        }
    }
}
