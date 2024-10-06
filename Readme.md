

Here is the README.md file for your Rust project:

# Rust Learning Project

This project is a collection of Rust code examples and exercises used to learn the Rust programming language.

## Project Structure

* `src/bin`: This directory contains executable Rust files that can be run directly.
	+ `shapes.rs`: A file that demonstrates the use of enums and pattern matching to calculate the area of different shapes.
	+ `struct.rs`: A file that defines a `User` struct and demonstrates its usage.
	+ `main.rs`: A file that contains a `main` function and demonstrates the use of functions and variables.
	+ `fibonacci.rs`: A file that calculates the nth Fibonacci number using recursion.
* `Cargo.toml`: The configuration file for the Rust project.

## Code Examples

### Shapes

The `shapes.rs` file defines an enum `Shape` with three variants: `Square`, `Rectangle`, and `Circle`. It also defines a function `find_area` that calculates the area of a given shape using pattern matching.

```rust
enum Shape {
    Square(f32),
    Rectangle(f32, f32),
    Circle(f32),
}

fn find_area(shape: Shape) -> f32 {
    match shape {
        Shape::Square(side) => side * side,
        Shape::Rectangle(width, height) => width * height,
        Shape::Circle(radius) => std::f32::consts::PI * radius * radius,
    }
}
```

### Struct

The `struct.rs` file defines a `User` struct with four fields: `username`, `email`, `sign_in_count`, and `active`. It also demonstrates how to create a new `User` instance and print its fields.

```rust
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        username: String::from("vishakha"),
        email: String::from("vishu@gmail.com"),
        sign_in_count: 1,
        active: true,
    };
    println!("User 1 details: email: {}, username: {}, sign_in_count: {}, active: {}", user1.email, user1.username, user1.sign_in_count, user1.active);
}
```

### Main

The `main.rs` file contains a `main` function that demonstrates the use of functions and variables.

```rust
fn get_string_length(s: &str) -> usize {
    s.chars().count()
}

fn main() {
    let my_string = String::from("Vishakha");
    let length = get_string_length(&my_string);
    println!("The number of characters in this string is: {}", length)
}
```

### Fibonacci

The `fibonacci.rs` file calculates the nth Fibonacci number using recursion.

```rust
fn fib(num: i32) -> i32 {
    if num == 0 {
        0
    } else if num == 1 {
        1
    } else {
        fib(num - 1) + fib(num - 2)
    }
}

fn main() {
    let ans = fib(44);
    println!("{}", ans);
}
```

## Build and Run

To build and run the project, navigate to the project directory and run the following commands:

```bash
cargo build
cargo run
```

This will build the project and run the `main` function in the `main.rs` file.

## Learn More

* [Rust Programming Language](https://www.rust-lang.org/)
* [Rust Book](https://doc.rust-lang.org/book/)
* [Rust Documentation](https://doc.rust-lang.org/std/)