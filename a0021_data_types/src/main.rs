fn main() {

    // String literals (&str) are used when the value of a string is known at compile time. 
    // String literals are a set of characters, which are hardcoded into a variable. 
    // For example, let company="Codingbee". 
    // String literals are found in module std::str. String literals are also known as string slices.
    let company_string: &str = "codingbee";  // string type
    let rating_float = 4.5;             // float type
    let is_growing_boolean = true;     // boolean type
 
    println!("company name is:{}",company_string);
    println!("company rating on 5 is:{}",rating_float);
    println!("company is growing :{}", is_growing_boolean);
    println!("\n\n\n");


    // String literals are static by default. This means that string literals are guaranteed to be valid for the 
    // duration of the entire program. We can also explicitly specify the variable as static as shown below:
    let company1:&'static str = "Titanic";
    let location1:&'static str = "Atlantic";
    println!("company is : {} location :{}",company1,location1);


    // The naming convention for Constants are similar to that of variables. All characters in a constant name are 
    // usually in uppercase. Unlike declaring variables, the let keyword is not used to declare a constant.
    const USER_LIMIT:i32 = 100;    // Declare a integer constant
    const PI:f32 = 3.14;           //Declare a float constant
 
    println!("user limit is {}",USER_LIMIT);  //Display value of the constant
    println!("pi value is {}",PI);            //Display value of the constant


    // A variable declared using the let keyword is by default immutable. 
    // However, you have an option to mutate it using the mut keyword. Constants are immutable.

    // Constants can be set only to a constant expression and not to the result of a 
    // function call or any other value that will be computed at runtime.


    // Constants can be declared in any scope, including the global scope, which makes them useful for 
    // values that many parts of the code need to know about.



    // Shadowing of Variables and Constants - https://www.tutorialspoint.com/rust/rust_constant.htm

    // you can override variables:

    let salary: f64 = 100.00;
    println!("Old salary is: {}",salary);

    let salary: f64 = 1.50; 
    println!("The value of salary is: {}",salary);


    // This approave is useful if you know the first value is no longer needed. e.g.
    let name: &str = "Peter";
    // len() outputs variable of the type "usize" so convert that here. 
    let name: i32 = name.len() as i32;
    println!("name changed to integer : {}",name);


    // An alternative to using overriding is that you can use mutable variables

    let mut salary1: f64 = 1000.00;
    println!("Old salary is: {}",salary1);

    salary1 = 2000.45; 
    println!("The value of salary is: {}",salary1);



    // There are 2 types of string data:
    // String Literal(&str)
    // String Object(String)




}
