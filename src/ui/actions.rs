use std::process::Command;

use super::XiTerm;
use components::{ Dev, Prompt, EditorResponse, Message };
use core::ActionHandler;
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
                        if self.editor.display_title_bar {
                            if let Some((width, height)) = self.current_size {
                                self.handle_resize((width, height + 1));
                            }
                        } else if let Some((width, height)) = self.current_size {
                            self.handle_resize((width, height - 1));
                        }
                        self.editor.display_title_bar = !self.editor.display_title_bar;
                        self.prompt = None;
                    },
                    UiAction::ToggleLineNumbers => {
                        if self.editor.display_gutter {
                            if let Some((width, height)) = self.current_size {
                                self.handle_resize((width, height + 4));
                            }
                        } else if let Some((width, height)) = self.current_size {
                            self.handle_resize((width, height - 1));
                        }
                        self.editor.display_gutter = !self.editor.display_gutter;
                        self.prompt = None
                    }
                }
            }
        }
    }
}
