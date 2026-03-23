use kalosm::language::*;
use file_chatbot::solution::file_library;

use crate::solution::Cache;

pub struct ChatbotV5 {
    model: Llama,
    cache: Cache<Chat<Llama>>,
}

impl ChatbotV5 {
    pub fn new(model: Llama) -> ChatbotV5 {
        return ChatbotV5 {
            model: model,
            cache: Cache::new(3),
        };
    }

    pub async fn chat_with_user(&mut self, username: String, message: String) -> String {
        let filename = &format!("{}.txt", username);
        let cached_chat = self.cache.get_chat(&username);

        match cached_chat {
            None => {
                println!("chat_with_user: {username} is not in the cache!");
                // The cache does not have the chat. What should you do?
                return String::from("Hello, I am not a bot (yet)!");
            }
            Some(chat_session) => {
                println!("chat_with_user: {username} is in the cache! Nice!");
                // The cache has this chat. What should you do?
                return String::from("Hello, I am not a bot (yet)!");

            }
        }
    }

    pub fn get_history(&mut self, username: String) -> Vec<String> {
        let filename = &format!("{}.txt", username);
        let cached_chat = self.cache.get_chat(&username);

        match cached_chat {
            None => {
                println!("get_history: {username} is not in the cache!");
                if let Some(session) = file_library::load_chat_session_from_file(filename) { // to see if we have the info in the files
                    let chat = self.model
                        .chat()
                        .with_system_prompt("The assistant will act like a pirate")
                        .with_session(session); // getting back the session from the file
                    self.cache.insert_chat(username, chat); // putting in the cache it has been used for the next time
                }
                // TODO: The cache does not have the chat. What should you do?
                // Your code goes here.
                return Vec::new();
            }
            Some(chat_session) => {
                println!("get_history: {username} is in the cache! Nice!");
                let session = chat_session.session().unwrap(); //get the current session
                let history = session.history(); // getting all the emssages
                let mut output = Vec::new(); // creating an empty output list
                for message in history.iter().skip(1) {
                    output.push(message.content().to_string()); // adding all the messages other than the system messag in to the vector
                }
                return output;

                // TODO: The cache has this chat. What should you do?
                // Your code goes here.
            }
        }
    }
}