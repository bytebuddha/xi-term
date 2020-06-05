use serde_json::Value;

use std::process::Command;

use super::XiTerm;
use components::{ Dev, Prompt, EditorResponse, Message };
use core::{ ActionHandler, consts::{ DEFAULT_DISPLAY_GUTTER, DEFAULT_DISPLAY_TITLE_BAR } };
use actions::{ Action, SystemAction, UiAction };

impl ActionHandler<Action> for XiTerm {

    fn perform_action(&mut self, action: Action) {
        match action {
            Action::System(SystemAction::Quit) => self.exit = true,
            Action::Editor(action) => {
                match self.editor.perform_action(action) {
                    EditorResponse::Message(msg) => {
                        if let Some(prompt) = &mut self.prompt {
                            prompt.set_message(msg);
                        }
                    },
                    EditorResponse::Cancel => {
                        self.prompt = None;
                    },
                    EditorResponse::Action(action) => self.perform_action(action),
                    EditorResponse::Continue => {},
                }
            },
            Action::ShellCommand(cmds) => {
                let sys_cmd = Command::new(&cmds[0]).args(&cmds[1..]).output();
                let msg = match sys_cmd {
                    Ok(msg) => {
                        String::from_utf8_lossy(&msg.stdout).to_string()
                    },
                    Err(err) => {
                        format!("{}", err)
                    }
                };

                if let Some(prompt) = &mut self.prompt {
                    prompt.set_message(Message::info(msg).title("Shell"));
                } else {
                    let mut prompt = Prompt::default();
                    prompt.set_message(Message::info(msg).title("Shell"));
                    self.prompt = Some(prompt);
                }
            },
            Action::Ui(action) => {
                match action {
                    UiAction::ShowPrompt => self.prompt = Some(Prompt::default()),
                    UiAction::ShowDebugWidget => {self.dev = Some(Dev::default());self.prompt = None},
                    UiAction::HideDebugWidget => self.dev = None,
                    UiAction::ToggleDebugWidget => {
                        if self.dev.is_none() {
                            self.dev = Some(Dev::default());
                        } else {
                            self.dev = None;
                        }
                    },
                    UiAction::HidePrompt => self.prompt = None,
                    UiAction::ToggleTitleBar => {
                        match self.editor.config.get_default("display_title_bar", DEFAULT_DISPLAY_TITLE_BAR) {
                            Value::Bool(true) => self.editor.config.insert_value("display_title_bar", Value::Bool(false)),
                            Value::Bool(false) => self.editor.config.insert_value("display_title_bar", Value::Bool(true)),
                            value => warn!("Invalid Value for `display_title_bar`: {:?}", value)
                        }
                        self.prompt = None;
                    },
                    UiAction::ToggleLineNumbers => {
                        match self.editor.config.get_default("display_gutter", DEFAULT_DISPLAY_GUTTER) {
                            Value::Bool(true) => self.editor.config.insert_value("display_gutter", Value::Bool(false)),
                            Value::Bool(false) => self.editor.config.insert_value("display_gutter", Value::Bool(true)),
                            value => warn!("Invalid Value for `display_gutter`: {:?}", value)
                        }
                        self.prompt = None
                    }
                }
            }
        }
    }
}
