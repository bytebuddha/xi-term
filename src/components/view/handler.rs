use crossterm::event::{ Event, KeyCode, MouseEvent, MouseButton };

use super::View;
use core::{ EventHandler, ActionHandler };
use actions::{ ViewAction, CursorAction };

impl EventHandler for View {

    fn handle_event(&mut self, event: Event) {
        match event {
            Event::Key(key) => {
                match key.code {
                    KeyCode::Char(c) => match c {
                        '\n' => self.insert_newline(),
                        '\t' => self.insert_tab(),
                        _ => self.insert(c),
                    },
                    KeyCode::Enter => self.client.insert_newline(),
                    KeyCode::Backspace => self.perform_action(ViewAction::Cursor(CursorAction::Backspace)),
                    KeyCode::Delete => self.perform_action(ViewAction::Cursor(CursorAction::Delete)),
                    KeyCode::Left => self.perform_action(ViewAction::Cursor(CursorAction::Left)),
                    KeyCode::Right => self.perform_action(ViewAction::Cursor(CursorAction::Right)),
                    KeyCode::Up => self.perform_action(ViewAction::Cursor(CursorAction::Up)),
                    KeyCode::Down => self.perform_action(ViewAction::Cursor(CursorAction::Down)),
                    KeyCode::Home => self.perform_action(ViewAction::Cursor(CursorAction::Home)),
                    KeyCode::End => self.perform_action(ViewAction::Cursor(CursorAction::End)),
                    KeyCode::PageUp => self.perform_action(ViewAction::Cursor(CursorAction::PageUp)),
                    KeyCode::PageDown => self.perform_action(ViewAction::Cursor(CursorAction::PageDown)),
                    k => error!("un-handled key {:?}", k),
                }
            },
            Event::Mouse(mouse) => {
                match mouse {
                    MouseEvent::ScrollUp(_, _, _) => self.client.up(),
                    MouseEvent::ScrollDown(_, _, _) => self.client.down(),
                    MouseEvent::Down(btn, x, y, _) => {
                        match btn {
                            MouseButton::Left => {
                                if let Some(rect) = self.rect {
                                    error!("Click Rect: {:?}", rect);
                                    self.click(rect, u64::from(y), u64::from(x));
                                }
                            },
                            _ => {}
                        }
                    },
                    _ => {}
                }
            },
            ev => error!("un-handled event {:?}", ev),
        }
    }
}
