use kalosm::language::*;

#[allow(dead_code)]
pub struct ChatbotV2 {
    // What should you store inside your Chatbot type?
    // The model? The chat_session?
    chat_session: Chat<Llama>,
}

impl ChatbotV2 {
    #[allow(dead_code)]
    pub fn new(model: Llama) -> ChatbotV2 {
        return ChatbotV2 {
            // Whatever you decide to store in the struct
            // you need to make sure you pass here!
            chat_session: model.chat()
            .with_system_prompt("The assistant will act like a pirate"),
        };
        
    }

    #[allow(dead_code)]
    pub async fn chat_with_user(&mut self, message: String) -> String {
        // Add your code for chatting with the agent while keeping conversation history here.
        let asynchronous_output = self.chat_session.add_message(message);
        let output = asynchronous_output.await;
        let result = output.unwrap();
        return result;
    }
}