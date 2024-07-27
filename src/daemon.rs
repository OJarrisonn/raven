use std::{sync::Arc, thread};

use anyhow::Result;
use rv_raven::config::Config;

mod remote;
mod local;

fn main() -> Result<()> {
    let config = Arc::new(Config::load()?);
    
    let remote = Arc::clone(&config);
    let local = Arc::clone(&config);
    
    let _remote = thread::spawn(move || { remote::remote(remote) });
    let _local = thread::spawn(move || { local::local(local) });

    Ok(())
}