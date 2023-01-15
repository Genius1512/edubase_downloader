use std::io;

pub fn read_line(prompt: &str) -> String {
    println!("{}", prompt);
    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read input");

    number.trim().to_string()
}