pub const LISTEN_DEFAULT_ADDRESS: &str = "0.0.0.0";
pub const LISTEN_DEFAULT_PORT: u16 = 12345;

pub fn is_ipv4_address(address: &str) -> bool {
    address.parse::<std::net::Ipv4Addr>().is_ok()
}

pub fn basename<'path>(path: &'path str) -> &'path str {
    path.rfind("/").map(|pos| &path[pos + 1..]).unwrap_or(path)
}

/// Ensures that the given folder does exist.
pub fn ensure_folder(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = std::path::Path::new(path);

    if !path.exists() {
        std::fs::create_dir_all(path)?;
    }

    Ok(())
}
