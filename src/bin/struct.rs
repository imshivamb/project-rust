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
    println!(
        "User 1 details: email: {}, username: {}, sign_in_count: {}, active: {}",
        user1.email, user1.username, user1.sign_in_count, user1.active
    );
    println!("User1: {:#?}", user1)
}
