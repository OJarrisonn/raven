use std::{error::Error, path::Display};

use serde_derive::{Deserialize, Serialize};
use toml::value::Datetime;

use crate::{config::Config, util};

/// The mailbox is the structure that holds the messages and files that the client has received.
/// 
/// The mailbox is filled by the `receive` subcommand, while can be managed by the `mailbox` subcommand.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MailBox {
    pub messages: Vec<MailMessage>,
    pub files: Vec<MailFile>,
}

/// A message is a text message that the client has received.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MailMessage {
    pub from: String,
    pub when: Datetime,
    pub text: String,
}

/// A file is a file that the client has received.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MailFile {
    pub from: String,
    pub when: Datetime,
    pub name: String,
}

pub trait Summarizable {
    fn summary(&self) -> String;
}

impl MailBox {
    /// Creates a new empty mailbox.
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
            files: Vec::new(),
        }
    }

    pub fn open(config: &Config) -> Result<Self, Box<dyn Error>> {
        if !std::path::Path::new(&format!("{}/mailbox.toml", config.raven_home)).exists() {
            return Ok(Self::new());
        }

        let content = std::fs::read_to_string(format!("{}/mailbox.toml", config.raven_home))?;
        Ok(toml::from_str::<Self>(&content)?)
    }

    pub fn save(&self, config: &Config) -> Result<(), Box<dyn Error>> {
        let content = toml::to_string(self)?;
        util::ensure_folder(&config.raven_home)?;
        std::fs::write(format!("{}/mailbox.toml", config.raven_home), content)?;

        Ok(())
    }

    /// Adds a new message to the mailbox.
    pub fn add_message(&mut self, from: String, when: Datetime, text: String) {
        self.messages.push(MailMessage { from, when, text });
    }

    /// Adds a new file to the mailbox.
    pub fn add_file(&mut self, from: String, when: Datetime, name: String) {
        self.files.push(MailFile { from, when, name });
    }

    /// Removes a message from the mailbox.
    pub fn remove_message(&mut self, index: usize) {
        self.messages.remove(index);
    }

    /// Removes a file from the mailbox.
    pub fn remove_file(&mut self, index: usize) {
        self.files.remove(index);
    }

    pub fn show(&self, messages: bool, files: bool) {
        if messages {
            self.show_messages();
        }

        if files {
            self.show_files();
        }
    }

    fn show_messages(&self) {
        println!("Messages:");
        for (i, message) in self.messages.iter().enumerate() {
            println!("{}: {}", i, message.summary());
        }
    }

    fn show_files(&self) {
        println!("Files:");
        for (i, file) in self.files.iter().enumerate() {
            println!("{}: {}", i, file.summary());
        }
    }

    pub fn show_message(&self, index: usize) {
        if let Some(message) = self.messages.get(index) {
            println!("{}", message.text);
        } else {
            println!("Message `{}` not found", index);
        }
    }

    pub fn show_file(&self, index: usize) {
        if let Some(file) = self.files.get(index) {
            println!("File: {}", file.name);
        } else {
            println!("File `{}` not found", index);
        }
    }
}

impl Summarizable for MailMessage {
    fn summary(&self) -> String {
        const summary_len: usize = 32;
        let summary = self.text.chars().take(summary_len).collect::<String>();
        let dots = if self.text.len() > summary_len { "..." } else { "" };

        format!("[{}] From: {} :: {}{}", self.when, self.from, summary, dots)
    }
}

impl Summarizable for MailFile {
    fn summary(&self) -> String {
        format!("[{}] From: {} :: {}", self.when, self.from, self.name)
    }
}