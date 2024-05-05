use speedy::{Readable, Writable};


#[derive(Readable, Writable)]
pub enum Packet {
    Message(String),
    Conf(String, String)
}
