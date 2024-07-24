pub const LISTEN_DEFAULT_ADDRESS: &str = "0.0.0.0";
pub const LISTEN_DEFAULT_PORT: u16 = 12345;

pub fn get_config_file_name() -> String {
    std::env::var("RAVEN_CONFIG").unwrap_or(format!(
        "{}/.config/Raven.toml",
        std::env::var("HOME").expect(
            "No HOME environment variable defined. Problably running on unsupported platform"
        )
    ))
}

pub fn is_ipv4_address(address: &str) -> bool {
    address.parse::<std::net::Ipv4Addr>().is_ok()
}
