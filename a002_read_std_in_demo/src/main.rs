use std::io::stdin;

fn main() {

    // variables are immutable (unchangeable) by default. Therefore to make it mutable, you have to use the "mut" keyword. 
    let mut message: String = String::new();
    
    // The "!" means the "println" is actually a macro, rather than a function. 
    println!("What is your name?");

    stdin().read_line(&mut message).unwrap();

    // the "{}" is used to embed it with the content of the message var. 
    println!("Hello {}", message);
}
