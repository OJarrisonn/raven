use std::error::Error;

use chrono::{DateTime, Datelike, Timelike, Utc};
use serde_derive::{Deserialize, Serialize};
use toml::value::{Date, Datetime, Time};

use crate::{cli::MailboxSubcommands, config::Config, util};

/// The mailbox is the structure that holds the messages and files that the client has received.
///
/// The mailbox is filled by the `receive` subcommand, while can be managed by the `mailbox` subcommand.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MailBox {
    messages: Vec<MailMessage>,
    files: Vec<MailFile>,
}

/// A message is a text message that the client has received.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct MailMessage {
    pub from: String,
    pub when: Datetime,
    pub text: String,
}

/// A file is a file that the client has received.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct MailFile {
    pub from: String,
    pub when: Datetime,
    pub name: String,
    // TODO: Store the file hash to check when deleting
}

trait Summarizable {
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
    pub fn add_message(&mut self, from: String, when: DateTime<Utc>, text: String) {
        let when = util::chrono_to_toml_datetime(when);

        self.messages.push(MailMessage { from, when, text });
    }

    /// Adds a new file to the mailbox.
    pub fn add_file(&mut self, from: String, when: DateTime<Utc>, name: String) {
        let when = util::chrono_to_toml_datetime(when);

        self.files.push(MailFile { from, when, name });
    }

    /// Removes a message from the mailbox.
    pub fn remove_message(&mut self, index: usize) {
        if index >= self.messages.len() {
            println!("Message `{}` not found", index);
            return;
        }
        self.messages.remove(index);
    }

    /// Removes a file from the mailbox.
    pub fn remove_file(&mut self, index: usize) {
        if index >= self.files.len() {
            println!("File `{}` not found", index);
            return;
        }
        let file = self.files.remove(index);
        let _ =
            std::fs::remove_file(&file.name).map_err(|e| println!("Failed to remove file: {}", e));
    }

    pub fn list(&self, mut messages: bool, mut files: bool) {
        if !messages && !files {
            messages = true;
            files = true;
        }

        if messages {
            self.list_messages();
        }

        if files {
            self.list_files();
        }
    }

    fn list_messages(&self) {
        println!("Messages:");
        for (i, message) in self.messages.iter().enumerate() {
            println!("{}: {}", i, message.summary());
        }
    }

    fn list_files(&self) {
        println!("Files:");
        for (i, file) in self.files.iter().enumerate() {
            println!("{}: {}", i, file.summary());
        }
    }

    pub fn show_message(&self, index: usize) {
        if let Some(message) = self.messages.get(index) {
            println!("Message from: {}", message.from);
            println!("When: {}", util::fmt_datetime(util::toml_to_chrono_datetime(message.when)));
            println!("{}", message.text);
        } else {
            println!("Message `{}` not found", index);
        }
    }

    pub fn show_file(&self, index: usize) {
        if let Some(file) = self.files.get(index) {
            println!("File from: {}", file.from);
            println!("When: {}", util::fmt_datetime(util::toml_to_chrono_datetime(file.when)));
            println!("File: {}", file.name);
        } else {
            println!("File `{}` not found", index);
        }
    }
}

impl Summarizable for MailMessage {
    fn summary(&self) -> String {
        const SUMMARY_LEN: usize = 32;

        let summary = self.text.chars().take(SUMMARY_LEN).collect::<String>();
        let dots = if self.text.len() > SUMMARY_LEN { "..." } else { "" };

        format!("[{}] From: {} :: {}{}", util::fmt_datetime(util::toml_to_chrono_datetime(self.when)), self.from, summary, dots)
    }
}

impl Summarizable for MailFile {
    fn summary(&self) -> String {
        format!("[{}] From: {} :: {}", util::fmt_datetime(util::toml_to_chrono_datetime(self.when)), self.from, self.name)
    }
}

pub fn manage(command: MailboxSubcommands, config: Config) -> Result<(), Box<dyn Error>> {
    let mut mailbox = MailBox::open(&config)?;

    match command {
        MailboxSubcommands::List { files, messages } => mailbox.list(files, messages),
        MailboxSubcommands::Delete {
            index,
            file,
            message,
        } => {
            if file && message {
                return Err("You can't delete a file and a message at the same time".into());
            }

            if file {
                mailbox.remove_file(index);
            }

            if message {
                mailbox.remove_message(index);
            }

            mailbox.save(&config)?;
        }
        MailboxSubcommands::Show {
            index,
            file,
            message,
        } => {
            if file && message {
                return Err("You can't show a file and a message at the same time".into());
            }

            if file {
                mailbox.show_file(index);
            }

            if message {
                mailbox.show_message(index);
            }
        }
    }

    Ok(())
}
