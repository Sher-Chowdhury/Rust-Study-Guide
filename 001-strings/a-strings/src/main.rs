use a_strings::get_alias;

fn main() {

/*
    The String data type in Rust can be classified into the following −

        - String Literal(&str)
        - String Object(String)
*/ 


    // String Literals demo:

    /*
    String literals are static by default. This means that string literals are guaranteed to be valid for the duration of the entire 
    program. We can also explicitly specify the variable as static as shown below
    */


    let company: &str="TutorialsPoint";
    let location: &str = "Hyderabad";
    println!("company is : {} location :{}",company,location);


    // String Objects demo:

    /*
    The String object type is provided in Standard Library. 
    It is defined as public structure in standard library pub struct String. 
    String is a growable collection. It is mutable and UTF-8 encoded type. 
    The String object type can be used to represent string values that are provided at runtime, e.g. if a function returns a string,
    then you can capture that into a String object. 

    String objects allocated in the heap (as apposed to use ). Hence, it's not as good as String Literals when it comes to performance. 


    
    */

    let mut empty_string: String = String::new();
    println!("length is {}",empty_string.len());
    
    // https://users.rust-lang.org/t/what-is-the-idiomatic-way-to-convert-str-to-string/12160 - lets of ways to do this
    empty_string = "test".to_owned();
    println!("length is {}",empty_string.len());

 
    let content_string: String = String::from("TutorialsPoint");
    println!("length is {}",content_string.len());


    let alias_name: String = get_alias("Superman");
    println!("alias name is {}",alias_name);


    let alias_name1: String = get_alias("Batman");
    println!("alias name is {}",alias_name1);


    let alias_name1: String = get_alias("Spiderman");
    println!("alias name is {}",alias_name1);


}
