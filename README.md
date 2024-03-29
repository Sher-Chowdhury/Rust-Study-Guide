# Rust-Study-Guide


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

`i32` means it's the variable data type is an integer. Like golang, rust can work out the data type for you. Which means you can simply the above to:

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

