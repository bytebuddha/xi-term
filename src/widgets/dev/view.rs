use tui::layout::Rect;
use tui::buffer::Buffer;
use tui::widgets::{ Text, List, Widget, Borders, Block };
use tui::style::{ Style, Color, Modifier };
use tui::layout::{ Layout, Direction, Constraint };
use components::View;

pub struct CurrentViewWidget<'a> {
    view: &'a View
}

impl <'a>CurrentViewWidget<'a> {

    pub fn new(view: &'a View) -> CurrentViewWidget<'a> {
        CurrentViewWidget { view }
    }

    fn get_left_box<'b>(&self) -> Vec<Text<'b>> {
        let mut items = vec![
            Text::Raw(Cow::Owned(format!(" "))),
            Text::Styled(Cow::Owned(format!("View Id: {}", self.view.client.view_id)),
            Style::default().fg(Color::Magenta)),
            Text::Raw(Cow::Owned(format!(" "))),
            Text::Styled(Cow::Owned(format!("Pristine: {}", self.view.pristine)), Style::default().fg(Color::Magenta)),
            Text::Raw(Cow::Owned(format!(" "))),
            Text::Styled(Cow::Owned(format!("Cursor: ({}, {})", self.view.cursor.column, self.view.cursor.line)), Style::default().fg(Color::Magenta)),
            Text::Raw(Cow::Owned(format!(" "))),
            Text::Styled(Cow::Owned(format!("Rect: {:?}", self.view.rect)), Style::default().fg(Color::Magenta))
        ];

        if let Some(cfg) = &self.view.cfg {
            items.push(Text::Raw(Cow::Owned(format!(" "))));
            items.push(Text::Styled(Cow::Owned(format!("Font Face: {:?}", cfg.font_face)), Style::default().fg(Color::Magenta)));
            items.push(Text::Raw(Cow::Owned(format!(" "))));
            items.push(Text::Styled(Cow::Owned(format!("Font Size: {:?}", cfg.font_size)), Style::default().fg(Color::Magenta)));
            items.push(Text::Raw(Cow::Owned(format!(" "))));
            items.push(Text::Styled(Cow::Owned(format!("Line Endings: {:?}", cfg.line_ending)), Style::default().fg(Color::Magenta)));
            items.push(Text::Raw(Cow::Owned(format!(" "))));
            items.push(Text::Styled(Cow::Owned(format!("Plugin Search Path: {:?}", cfg.plugin_search_path)), Style::default().fg(Color::Magenta)));
            items.push(Text::Raw(Cow::Owned(format!(" "))));
            items.push(Text::Styled(Cow::Owned(format!("Tab Size: {:?}", cfg.tab_size)), Style::default().fg(Color::Magenta)));
            items.push(Text::Raw(Cow::Owned(format!(" "))));
            items.push(Text::Styled(Cow::Owned(format!("Tabes To Spaces: {:?}", cfg.translate_tabs_to_spaces)), Style::default().fg(Color::Magenta)));
            items.push(Text::Raw(Cow::Owned(format!(" "))));
            items.push(Text::Styled(Cow::Owned(format!("Word Wrap: {:?}", cfg.word_wrap)), Style::default().fg(Color::Magenta)));
        }
        items
    }

    fn get_right_box<'b>(&self) -> Vec<Text<'b>> {
        let items = vec![
            Text::Raw(Cow::Owned(format!(" "))),
            Text::Styled(Cow::Owned(format!("Window Start: {}", self.view.window.start())), Style::default().fg(Color::Magenta)),
            Text::Raw(Cow::Owned(format!(" "))),
            Text::Styled(Cow::Owned(format!("Window Start: {}", self.view.window.size())), Style::default().fg(Color::Magenta)),
            Text::Raw(Cow::Owned(format!(" "))),
            Text::Styled(Cow::Owned(format!("Line Cache Before: {}", self.view.cache.before())), Style::default().fg(Color::Magenta)),
            Text::Raw(Cow::Owned(format!(" "))),
            Text::Styled(Cow::Owned(format!("Line Cache After: {}", self.view.cache.after())), Style::default().fg(Color::Magenta)),
            Text::Raw(Cow::Owned(format!(" "))),
            Text::Styled(Cow::Owned(format!("Line Count: {}", self.view.cache.lines().len())), Style::default().fg(Color::Magenta)),
            Text::Raw(Cow::Owned(format!(" "))),
            Text::Styled(Cow::Owned(format!("Total Lines: {}",
                self.view.cache.lines().len()  as u64 +
                self.view.cache.before() +
                self.view.cache.after()
            )), Style::default().fg(Color::Magenta)),
        ];
        items
    }
}

use std::borrow::Cow;

impl <'a>Widget for CurrentViewWidget<'a> {

    fn render(self, area: Rect, buf: &mut Buffer) {

        Block::default()
                .borders(Borders::ALL)
                //.border_style(Style::default().bg(Color::DarkGray).fg(Color::Cyan))
                .style(Style::default().bg(Color::DarkGray))
                .render(area, buf);
        let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)].as_ref())
                .split(area);

        error!("{:?}", chunks);


        List::new(self.get_left_box().into_iter())
            .block(Block::default().title("Active View").borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().modifier(Modifier::ITALIC))
            .highlight_symbol(">>")
            .render(chunks[0], buf);
        List::new(self.get_right_box().into_iter())
            .block(Block::default().title("Line Cache").borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().modifier(Modifier::ITALIC))
            .highlight_symbol(">>")
            .render(chunks[1], buf);
    }
}
