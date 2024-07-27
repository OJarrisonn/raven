use anyhow::{bail, Result};
use rv_raven::{config::Config, raven::mailbox::MailBox};

use crate::cli::MailboxSubcommands;

pub fn manage(command: MailboxSubcommands, config: &Config) -> Result<()> {
    let mut mailbox = MailBox::open(config)?;

    match command {
        MailboxSubcommands::List { files, messages } => mailbox.list(files, messages),
        MailboxSubcommands::Delete {
            index,
            file,
            message,
        } => {
            if file && message {
                bail!("You can't delete a file and a message at the same time");
            }

            if file {
                mailbox.remove_file(index);
            } else if message {
                mailbox.remove_message(index);
            } else {
                bail!("You must specify if you want to delete a file or a message");
            }

            mailbox.save(&config)?;
        }
        MailboxSubcommands::Show {
            index,
            file,
            message,
        } => {
            if file && message {
                bail!("You can't show a file and a message at the same time");
            }

            if file {
                mailbox.show_file(index);
            } else if message {
                mailbox.show_message(index);
            } else {
                bail!("You must specify if you want to see a file or a message");
            }
        }
    }

    Ok(())
}
