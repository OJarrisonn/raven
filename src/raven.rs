use serde_derive::{Deserialize, Serialize};

/// The raven is the message that the client will send or receive.
/// It can be both a text message or a file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Raven {
    /// A text message
    Text { text: String },
    /// A file with the name and the content as a byte stream
    File { name: String, content: Vec<u8> },
}
