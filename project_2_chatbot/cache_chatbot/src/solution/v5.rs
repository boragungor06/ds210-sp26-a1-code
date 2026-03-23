use kalosm::language::*;
use file_chatbot::solution::file_library;
use std::fs;

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

    pub fn load_chat_session_from_file(filename: &str) -> Option<LlamaChatSession> {
    // look at fs::read(...)
    // also look at LlamaChatSession::from_bytes(...)

    let result = fs::read(filename);
    let bytes = match result {
        Ok(data) => data,
        Err(bruh) => return None,
    };
    let deserialized_session_result = LlamaChatSession::from_bytes(&bytes);

    let chat_session = match deserialized_session_result {
        Ok(deserialized_session) => deserialized_session,
        Err(bruh) => panic!("Failed to deserialize chat session from {filename}: {bruh}"),
    };

    let out = Some(chat_session);
    // while it may be more idiomatic to just return this without binding
    // explicitly defining variables makes type visible; huge for debugging
    return out;
    }

    pub fn save_chat_session_to_file(filename: &str, session: &LlamaChatSession) {
        let session_as_bytes = session.to_bytes().unwrap(); 
        fs::write(filename, session_as_bytes).unwrap(); 
    }

    pub async fn chat_with_user(&mut self, username: String, message: String) -> String {
        let filename = &format!("{}.txt", username);
        let cached_chat = self.cache.get_chat(&username);
        
        // get the Chat<Llama> object
        let mut active_chat = match cached_chat {
            None => {
                println!("chat_with_user: {username} is not in the cache!");
                // first, check if a file exists
                let mut chat_session = 
                    if let Some(session) = Self::load_chat_session_from_file(filename) {
                        // if it does, load from it
                        self.model.chat().with_session(session) 
                    }
                    else {
                        // if it does not, create a new session
                        self.model.chat().with_system_prompt("The assistant will act like a pirate")
                    };
                    chat_session
                    }
            Some(chat_session) => {
                println!("chat_with_user: {username} is in the cache! Nice!");
                chat_session.clone() 
                // rust wants a Chat<Llama> object, not a mutable reference. so we clone.
            }
        };

        // hopefully, active_chat is the Chat<Llama> object that represents the session. 
        // now, formulate the response and add it to the session
        let output = active_chat.add_message(message).await;
        let result = output.unwrap();

        // then, write to the file
        let clone = active_chat.clone();
        let session_unwrapped = clone.session().unwrap();
        // dereferencing the var clone_unwrapped (a smart pointer) yields a LlamaChatsession. 
        let referenced_session = &*session_unwrapped;

        Self::save_chat_session_to_file(filename, referenced_session);

        // next, this must become the most recently used cache conversation
        self.cache.insert_chat(username, active_chat);

        // return the bot's result
        return result
    }

    pub fn get_history(&mut self, username: String) -> Vec<String> {
        let filename = &format!("{}.txt", username);
        let cached_chat = self.cache.get_chat(&username);

        match cached_chat {
            None => {
                println!("get_history: {username} is not in the cache!");
                if let Some(session) = file_library::load_chat_session_from_file(filename) { // to see if we have the info in the files
                    let history = session.history();
                    let mut output = Vec::new();
                    for message in history.iter().skip(1) {
                        output.push(message.content().to_string());
                    }
                    let chat = self.model
                        .chat()
                        .with_system_prompt("The assistant will act like a pirate")
                        .with_session(session); // getting back the session from the file
                    self.cache.insert_chat(username, chat); // putting in the cache it has been used for the next time
                    return output;
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