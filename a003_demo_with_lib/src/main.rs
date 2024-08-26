
fn main() {

    let company:&str="CodingBee";
    let location:&str = "UK";
    println!("company is : {} location :{}",company,location);

    // Create a new string variable. 
    let mut number_str: String = String::from("23");
    println!("{}", number_str);
    number_str = "24".to_string(); // There's lots of ways to do this - https://users.rust-lang.org/t/what-is-the-idiomatic-way-to-convert-str-to-string/12160

    println!("{}", number_str);
}
