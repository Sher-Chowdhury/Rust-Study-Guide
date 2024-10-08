# Rust-Study-Guide


## Recommended book

https://doc.rust-lang.org/stable/book/


## Install rust

Run the curl command in - https://www.rust-lang.org/tools/install

After the install, you should end up with to cli commands, they are `cargo` and `rustc`

`cargo` is the main tool you'll be using most often, whereas `rustc` is less used, but when it is used, it's used for compiling. 


## Initialize a new Rust projct. 

Here's how to create a new rust project

```bash
$ cargo new my_rust_app

    Creating binary (application) `my_rust_app` package
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
```

This created the following structure:

```bash
$ tree my_rust_app 
my_rust_app
├── Cargo.toml
└── src
    └── main.rs

```

Note: if you want to use vscode play/debug buttons, then you need to open up vscode in the directory that contains the `Cargo.toml` file 


## Intall vs code extension

- `rust-analyzer` - the official rust extension
- `CodeLLDB` - needed for runnning rust code in debug mode. - https://code.visualstudio.com/docs/languages/rust#_install-debugging-support


## Hello world example
Hello world example:

```rust
use std;

fn main() {
    println!("Hello World!");
}
```

`println` is a standard library feature, the `!` indicates that `println` is actually a "macro" rather than a regular function. 

Macros: compiler-supported coding shortcuts. The compiler substitutes with something more complex behind the scenes. 

Rust uses the std library by default, so we can omit the use std line, and everything still works:

```rust
fn main() {
    println!("Hello World!");
}
```




Here's an example of variables in rust:

```rust
use std;

fn main() {
    let a: i32 = 38;
    let b: i32 = 4;
    let sum: i32 = a + b;
    
    println!("{} + {} = {}", a, b, sum);
}

```

`i32` means it's the variable data type is an integer. Like golang, rust can work out the data type for you. Which means you can simply change the above to:

```rust
use std;

fn main() {
    let a = 38;
    let b = 4;
    let sum = a + b; 
    
    println!("{} + {} = {}", a, b, sum);
}
```

However if you rewrite the above to this, then it will fail:

```rust
use std;

fn main() {
    let a = 38;
    let b = 4;
    let sum = a;    // sets an initial value for sum
    
    sum += b;       // updates the value of sum with a new value
    
    println!("{} + {} = {}", a, b, sum);
}
```

The above fails, becuase in Rust, variables are treated like constants by default, i.e. rust variables are immutable by default.  

To make variables mutable, you have to make use the `mut` keyword:

```rust
use std;

fn main() {
    let a = 38;
    let b = 4;
    let mut sum = a;
    
    sum += b;
    
    println!("{} + {} = {}", a, b, sum);
}
```

   
Here's an example of how to write a function:


```rust
use std;

fn add(n1: i32, n2: i32) -> i32 {
    return (n1 + n2); 
}

fn main() {
    let a = 38;
    let b = 4;
    let sum = add(a, b);
    
    println!("{} + {} = {}", a, b, sum);
}
```

the `->` is just rust syntax which is used to show what gets return, which in this case is one return value of the type integer. 

The return line can be simplified to `return n1 + n2`, or even just `n1 + n2`



## Crates

In a rust project, you can have:

- up to one library create, and
- as many binary crates as you like. 

The `main.rs` file is the rust project's root binary crate. 


## Variables

All variable names must be in snake case format. 

In Rust, variables are immutable by default, i.e. they are unchangeable. Therefore, you have to use the `mut` key word to say that a variable is mutable.

E.g. this is an immutable variable:

```
let my_number = 1;
```

Therefore you can't do:

```rust
my_number = 2;
```

However you can override (aka shadow) a variable:

```rust
let my_number = 2;
```


Whereas here's how to define a mutable variable

```rust
let my_next_number = 2;
```

Here's how do define a constant:

```rust
const my_lucky_number: i32 = 7;
```
