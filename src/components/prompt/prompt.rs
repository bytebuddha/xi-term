use actions::parse_action;
use super::{ PromptResponse, Message };

#[derive(Default)]
pub struct Prompt {
    pub message: Option<Message>,
    pub dex: usize,
    pub chars: String
}

impl Prompt {

    pub fn set_message(&mut self, msg: Message) {
        self.message = Some(msg);
    }

    pub fn left(&mut self) -> PromptResponse {
        if self.dex > 0 {
            self.dex -= 1;
        }
        PromptResponse::Continue
    }

    pub fn right(&mut self) -> PromptResponse {
        if self.dex < self.chars.len() {
            self.dex += 1;
        }
        PromptResponse::Continue
    }

    pub fn delete(&mut self) -> PromptResponse {
        if self.dex < self.chars.len() {
            self.chars.remove(self.dex);
        }
        PromptResponse::Continue
    }

    pub fn back(&mut self) -> PromptResponse {
        if !self.chars.is_empty() && self.dex > 0 {
            self.dex -= 1;
            self.chars.remove(self.dex);
            PromptResponse::Continue
        } else {
            PromptResponse::Cancel
        }
    }

    pub fn new_key(&mut self, chr: char) -> PromptResponse {
        self.chars.insert(self.dex, chr);
        self.dex += 1;
        PromptResponse::Continue
    }

    /// Gets called when return is pressed,
    pub fn finalize(&mut self) -> PromptResponse {
        info!("Received input: {}", self.chars);
        parse_action(&self.chars)
    }
}
