use std::{
    env,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        eprint!("Usage : {} filename", args[0]);
        return;
    }

    let filename = &args[1];
    let path = Path::new(filename);
    if !path.exists() {
        eprintln!("Path {} does not exists !", path.to_string_lossy());
        return;
    }

    if !path.is_file() {
        eprintln!("{:?} is not a valid file !", path.file_name());
        return;
    }

    let file = match File::open(&path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to open {filename} : {e}");
            return;
        }
    };

    let mut char_count = 0;
    let mut word_count = 0;
    let mut line_count = 0;

    for line in io::BufReader::new(file).lines() {
        match line {
            Ok(elem) => {
                line_count += 1;
                word_count += elem.split_whitespace().count();
                char_count += elem.chars().count();
            }
            Err(e) => {
                eprintln!("Error while counting : {e}");
                return;
            }
        }
    }

    println!("Lines : {line_count}");
    println!("Words = {word_count}");
    println!("Chars : {char_count}");
}
