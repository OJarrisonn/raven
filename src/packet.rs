use speedy::{Readable, Writable};

#[derive(Readable, Writable)]
pub struct Packet {
    pub version: u8,
    pub feather: Feather,
    pub data: Data
}

#[derive(Readable, Writable)]
pub enum Data {
    Message(String),
    Auth
}
