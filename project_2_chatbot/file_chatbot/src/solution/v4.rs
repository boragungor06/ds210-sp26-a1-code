use kalosm::language::*;
use crate::solution::file_library;

pub struct ChatbotV4 {
    model: Llama,
}

impl ChatbotV4 {
    pub fn new(model: Llama) -> ChatbotV4 {
        return ChatbotV4 {
            model: model,
        };
    }

    pub async fn chat_with_user(&mut self, username: String, message: String) -> String {
        let filename = &format!("{}.txt", username);

        let mut chat_session: Chat<Llama> = self.model
            .chat()
            .with_system_prompt("The assistant will act like a pirate");

        if let Some(session) = file_library::load_chat_session_from_file(filename) {
            chat_session = self.model
                .chat()
                .with_session(session); // Chat and system prompt is as is, I got with_session part from the link below

        }
        let output = chat_session.add_message(message).await;
        let result = output.unwrap();

        match chat_session.session() {
            Ok(session) => file_library::save_chat_session_to_file(filename, &session),
            Err(_) => {}
        }

        // TODO: You have to implement the rest:
        // You need to load the chat session from the file using file_library::load_chat_session_from_file(...).
        // Think about what needs to happen if the function returns None vs Some(session).
        // Hint: look at https://docs.rs/kalosm/latest/kalosm/language/struct.Chat.html#method.with_session

        return result;
    }

    pub fn get_history(&self, username: String) -> Vec<String> {
        let filename = &format!("{}.txt", username);

        match file_library::load_chat_session_from_file(&filename) {
            None => {
                return Vec::new();
            },
            Some(session) => {
                let mut hist: Vec<String> = Vec::new();

                let history = session.history();

                for message in history.iter().skip(1) {
                    let x = message.content().to_string();
                    hist.push(x)
                }
                return hist;
            }
        }
    }
}