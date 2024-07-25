use std::{ffi::OsStr, path::Path};

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

/// Returns a filename that doesn't collide with the existing files.
pub fn non_colliding_filename(path: &str) -> String {
    let mut i = 1;
    let path = Path::new(path);

    if !path.exists() {
        return path.to_str().unwrap().to_string();
    }

    let filename = path.file_stem().unwrap_or(OsStr::new("")).to_str().unwrap();
    let extension = path.extension().unwrap_or(OsStr::new("")).to_str().unwrap();
    let source = path.parent().unwrap_or(Path::new(".")).to_str().unwrap();

    let mut path = format!("{}/{}{}", source, filename, extension);

    while Path::new(&path).exists() {
        path = format!("{}/{}_{}.{}", source, filename, i, extension);
        i += 1;
    }

    path
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_name_collision() {
        // Some example files
        let files = vec!["/tmp/file.txt", "/tmp/file_1.txt", "/tmp/file.txt.gz", "/tmp/file_3.txt"];
        
        files.iter().for_each(|file| {
            if !std::path::Path::new(file).exists() {
                std::fs::File::create(file).expect("Failed to create file");
            }
        });   

        let non_colliding = files.iter().map(|file| super::non_colliding_filename(file)).collect::<Vec<String>>();
        
        non_colliding.iter().for_each(|file| {
            assert!(!files.contains(&file.as_str()), "The file `{}` is colliding with the original files", file);
            assert!(!std::path::Path::new(&file).exists(), "The file `{}` souldn't exist", file);
        });
    }
}