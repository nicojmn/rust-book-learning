use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 || args.len() > 2 {
        eprintln!("Usage {} <word>", args[0]);
    }

    let word = args.get(1).expect("Failed to read word").trim();
    let reversed: String = word.chars().rev().collect();

    if word == reversed {
        println!("{word} is a palindrome !")
    } else {
        println!("{word} is not a palindrome !")
    }
}
