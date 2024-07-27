//! This is the module responsible for the backend-frontend integration in raven
//! 
//! `rv` commands delegate IO operations to `rvd` through a local socket.

use std::sync::Arc;

use anyhow::Result;
use interprocess::local_socket::ListenerOptions;
use rv_raven::config::Config;

pub fn local(config: Arc<Config>) -> Result<()> {
    let _local_listener = ListenerOptions::new().name(config.raven_sock_name()?).create_sync()?;
    println!("Local socket created");

    Ok(())
}