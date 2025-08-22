use std::{io, path::Path};
mod list_dir;

fn main() {
    println!("Please enter the dir path for lookup");

    let mut dir = String::new();

    io::stdin()
        .read_line(&mut dir)
        .expect("Failed to read input");

    dir = dir.trim().to_string();

    let path = Path::new(&dir);
    match list_dir::list(path) {
        Ok(entries) => {
            for e in entries {
                println!("{e}")
            }
        }
        Err(e) => println!("Error : {e}"),
    };
}
