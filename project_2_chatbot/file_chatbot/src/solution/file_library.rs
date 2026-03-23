use kalosm::language::*;

// Look at the docs for std::fs
// https://doc.rust-lang.org/std/fs/index.html
// std::fs provides functions that write to a file, read from a file,
// check if a file exists, etc.
use std::fs;

// LlamaChatSession provides helpful functions for loading and storing sessions.
// Look at https://docs.rs/kalosm/latest/kalosm/language/trait.ChatSession.html#saving-and-loading-sessions
// for some examples!

// Implement this
pub fn save_chat_session_to_file(filename: &str, session: &LlamaChatSession) {
    // look at fs::write(...)
    unimplemented!("Saving chat session to file {filename}");
}

// Implement this
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