use serde_derive::{Deserialize, Serialize};

pub mod receive;
pub mod send;
pub mod mailbox;

/// The raven is the message that the client will send or receive.
/// It can be both a text message or a file.
///
/// TODO:
/// - Trust: used to stablish a trusthy connection between the clients.  
/// - Helo: used before sending a message to check if the target will accept it.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Raven {
    /// A text message
    Text { text: String },
    /// A file with the name and the content as a byte stream
    File { name: String, content: Vec<u8> },
}
