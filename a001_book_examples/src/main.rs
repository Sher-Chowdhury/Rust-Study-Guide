// fn main() {
//     println!("Hello, world!");
// }

// Page 36
// fn main() {
//     let s1: &str = "hello";
//     let s2: &str = s1; 
//     println!("{s1}");
//     println!("{s2}");


//     // this is a failing example
//     // "String" based data are more complex when compared to &str, and therefore can't be copied. 
//     // instead, it ends up getting moved. 

//     let s1: String = String::from("hello");
//     let s2: String = s1; 
//     println!("{s1}");  // this line won't work because, s1 doesn't exist before. becuase s2 now owns s1
//     println!("{s2}"); 
// }




// // Page 36
// fn main() {
//     let s :u8 = 255;
//     println!("{s}");
// }


// Page 62
// "s" is a string literal. I.e. the value is calculated and hardcoded during compile time. It can be mutable, in which case the last value it gets 
// set to ends up becoming the hardcoded inside the value. It's also why it contains the "&" in "&str" which indicates that it points to a pacific location 
// inside the binary. The size, or content of this string variable will not change during the binary's runtime. 
// fn main() {
//     let s :&str = "hello";
//     println!("{s}");
// }


// // Page 63
// fn main() {
//     let s :String = String::from("hello 63");
//     println!("{s}");

//     let s :&str = &String::from("hello 63");
//     println!("{s}");
// }


// // Page 64

// fn change(mut s: String){
//     s.push_str(" world");
//     println!("{s}");
// }
// fn main() {
//     let s :String = String::from("hello");
//     println!("{s}");

//     change(s.clone());
//     println!("{s}");
// }


// // Page 69
// fn main() {
//     let s :String = String::from("hello");
//     println!("{s}");

//     change(s);  // this has moved ownership of s. 

//     // println!("{s}");  // this will fail because "s" is now out of this scope. 
// }

// fn change(mut s: String){
//     s.push_str(" world");
//     println!("{s}");
// }



// // Page 71
// fn change(s: &mut String){
//     s.push_str(" world");
//     println!("{s}");
// }

// fn main() {
//     let mut s :String = String::from("hello");
//     println!("{s}");

//     change(&mut s);
//     println!("{s}");  // this variable is now updated
// }


// // Page 79
// fn main() {
//     let s :String = String::from("hello world");

//     let h: &str = &s[0..5];
//     let w: &str = &s[6..];
//     println!("{h}");
//     println!("{w}");
// }


// // Page 81 - failing example
// fn main() {
//     let mut s :String = String::from("hello world");

//     let word: &str = first_word(&s);
//     // s.clear();
//     println!("{word}"); // "s" needs to exist at this point, becuase "word" actually is set to &s[0..5]. I.e. it's pointing to memory localation where s lives. 
// }

// fn first_word(s: &String) -> &str {
//     println!("entered first_word");
//     &s[0..5]
// }

// // Page 81
// fn main() {
//     let mut s :&str= "hello"; // this can be mutable. but ultimately this will get set to "world" inside the compiled binary  
//     s = "world";

//     println!("{s}");
// }


// // Page 86
// struct User {
//     employed: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64
// }

// fn main() {
//     let user1: User = User { 
//         employed: true, 
//         username: String::from("Neo"), 
//         email: String::from("neo@matrix.com"),  
//         sign_in_count: 5 
//     };

//     println!("{} {} {} {}", {user1.employed},{user1.username},{user1.email},{user1.sign_in_count})
// }

// // Page 86
// struct User {
//     employed: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64
// }

// fn main() {
//     let user1: User = User { 
//         employed: true, 
//         username: String::from("Neo"), 
//         email: String::from("neo@matrix.com"),  
//         sign_in_count: 5 
//     };

//     // user1.username = String::from("Thomas") // this won't work, unless you make the entire struct variable mutable

//     println!("{} {} {} {}", {user1.employed},{user1.username},{user1.email},{user1.sign_in_count})
// }


// // Page 86
// struct User {
//     employed: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64
// }

// fn main() {
//     let mut user1: User = User { 
//         employed: true, 
//         username: String::from("Neo"), 
//         email: String::from("neo@matrix.com"),  
//         sign_in_count: 52 
//     };

//     user1.username = String::from("Thomas"); // this won't work, unless you make the entire struct variable mutable

//     println!("{} {} {} {}", {user1.employed},{user1.username},{user1.email},{user1.sign_in_count})
// }


// // Page 94
#[derive(Debug)]
struct User {
    employed: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

fn main() {
    let mut user1: User = User { 
        employed: true, 
        username: String::from("Neo"), 
        email: String::from("neo@matrix.com"),  
        sign_in_count: 52 
    };

    user1.username = String::from("Thomas"); // this won't work, unless you make the entire struct variable mutable

    println!("{:#?}", user1)
}