use std::process::Command;

use super::XiTerm;
use components::{ Prompt, PromptResponse, Message };
use core::ActionHandler;
use actions::{ Action, SystemAction, UiAction };

impl ActionHandler<Action> for XiTerm {

    fn perform_action(&mut self, action: Action) {
        match action {
            Action::System(SystemAction::Quit) => self.exit = true,
            Action::Editor(action) => {
                match self.editor.perform_action(action) {
                    PromptResponse::Message(msg) => {
                        if let Some(prompt) = &mut self.prompt {
                            prompt.set_message(msg);
                        }
                    },
                    PromptResponse::Cancel => {
                        self.prompt = None;
                    },
                    PromptResponse::Action(action) => self.perform_action(action),
                    _ => {},
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
                    let mut prompt = Prompt::new();
                    prompt.set_message(Message::info(msg).title("Shell"));
                    self.prompt = Some(prompt);
                }
            },
            Action::Ui(action) => {
                match action {
                    UiAction::ShowPrompt => self.prompt = Some(Prompt::new()),
                    UiAction::HidePrompt => self.prompt = None
                }
            }
        }
    }
}
