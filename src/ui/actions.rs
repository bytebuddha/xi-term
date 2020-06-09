use std::process::Command;

use super::XiTerm;
use components::{ Dev, Prompt, EditorResponse, Message };
use core::ActionHandler;
use actions::{ Action, SystemAction, UiAction, SettingsAction, ConfigAction };

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
                }
            },
            Action::Settings(action) => {
                match action {
                    SettingsAction::Config(action) => match action {
                        ConfigAction::Set(key, value) => {
                            self.editor.config.insert_value(&key, value);
                            self.prompt = None;
                        },
                        ConfigAction::UnSet(key) => {
                            if let Err(err) = self.editor.config.remove_value(&key) {
                                warn!("Unknown Config Error: {:?}", err);
                            }
                        },
                        ConfigAction::Get(key) => {
                            if let Ok(value) = self.editor.config.get(&key) {
                                let mut prompt = Prompt::default();
                                prompt.set_message(Message::info(format!("Config field: {} = {}", key, value)).title("Config Value"));
                                self.prompt = Some(prompt);
                            } else {
                                let mut prompt = Prompt::default();
                                prompt.set_message(Message::info(format!("Config field: {} = None", key)).title("Config Value"));
                                self.prompt = Some(prompt);
                            }
                        },
                        ConfigAction::Bind(event, actions) => {
                            if let Err(err) = self.actions.insert(event, actions) {
                                error!("Failed to insert new action binding: {:?}", err)
                            }
                            self.prompt = None;
                        },
                        ConfigAction::UnBind(event) => {
                            if let Err(err) = self.actions.remove(event) {
                                error!("Failed to remove event binding: {:?}", err)
                            }
                            self.prompt = None;
                        }
                    }
                }
            }
        }
    }
}
