use std::path::{Path, PathBuf};
use std::{env, fs};

pub fn list(path: &Path) -> Result<Vec<String>, std::io::Error> {
    let path = &expand_tilde(path);
    if !path.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Path does not exists",
        ));
    }

    let mut entries: Vec<String> = Vec::new();

    for entry in fs::read_dir(path)? {
        match entry {
            Ok(entry) => {
                let p = entry.path();
                entries.push(
                    p.into_os_string()
                        .into_string()
                        .expect("Failed to convert path into string"),
                );
            }
            Err(e) => {
                println!("Failed to read entry : {e}");
            }
        };
    }
    Ok(entries)
}

fn expand_tilde(path: &Path) -> PathBuf {
    if path.starts_with("~") {
        let str = path.to_str().expect("Failed to convert path");
        let home_path = env::home_dir().expect("Failed to get home directory");
        let rel_path = &str[1..];
        return home_path.join(rel_path);
    }
    path.to_path_buf()
}
