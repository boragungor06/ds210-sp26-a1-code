use kalosm::language::*;
use std::collections::HashMap;

#[allow(dead_code)]
pub struct ChatbotV3 {
    // What should you store inside your Chatbot type?
    // The model? The chat_session?
    // Storing a single chat session is not enough: it mixes messages from different users
    // together!
    // Need to store one chat session per user.
    // Think of some kind of data structure that can help you with this.
    model: Llama,
    session_map: HashMap<String, Chat<Llama>>,

}

impl ChatbotV3 {
    #[allow(dead_code)]
    pub fn new(model: Llama) -> ChatbotV3 {
        return ChatbotV3 {
            model: model,
            session_map: HashMap::new(),
            // Make sure you initialize your struct members here
        };
    }

    #[allow(dead_code)]
    pub async fn chat_with_user(&mut self, username: String, message: String) -> String {
        if self.session_map.contains_key(&username) {
            let session = self.session_map.get_mut(&username).unwrap(); // sets the session to the one corresponding with the username on the session map
            
            let output = session.add_message(message).await; 
            let result = output.unwrap(); // formulates response to a message
            return result;
        }
        else {
        let mut new_session = self.model.chat().with_system_prompt("The assistant will act like a pirate"); // creates new session for a new user

        let output = new_session.add_message(message).await;
        let result = output.unwrap(); // formulates response to a message
        
        self.session_map.insert(username, new_session); // saves username and username's session in the session map
        return result;
    
    }
}

    #[allow(dead_code)]
    pub fn get_history(&self, username: String) -> Vec<String> {
            if self.session_map.contains_key(&username) {
                let chat = self.session_map.get(&username).unwrap();
                let session = chat.session().unwrap();
                let history = session.history();

                let mut out: Vec<String> = Vec::new();

                for message in history {
                    out.push(format!("{:?}", message));
                }

                return out;
            }

            return Vec::new();
    }
}
