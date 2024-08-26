use std::io::stdin;

fn main() {
    let mut message: String = String::new();
    println!("What is your name?");

    stdin().read_line(&mut message).unwrap();

    println!("Hello {}", message);
}
