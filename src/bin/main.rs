// fn main() {
//     let ans = is_even(4534);
//     println!("{}", ans);
// }

// // i32 = - +, u32 = +

// fn is_even(n: i32) -> bool {
//     if n % 2 == 0 {
//         return true;
//     } else {
//         return false;
//     }
// }

fn get_string_length(s: &str) -> usize {
    s.chars().count()
}

fn main() {
    let my_string = String::from("Vishakha");
    let length = get_string_length(&my_string);
    println!("The number of characters in this string is: {}", length)
}
