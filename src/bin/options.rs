fn main() {
    let s = String::from("Vihaksha");
    let index = find_first_s(s);

    match index {
        Some(value) => println!("Found s at index {}", value),
        None => println!("s not found"),
    }
}

fn find_first_s(s: String) -> Option<i32> {
    if s.len() == 0 {
        return None;
    }
    for (index, char) in s.chars().enumerate() {
        if char == 's' {
            return Some(index as i32);
        }
    }
    return None;
}
