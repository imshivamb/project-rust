fn main() {
    let ans = fib(44);
    println!("{}", ans);
}

// i32 = - +, u32 = +

fn fib(num: i32) -> i32 {
    let mut first = 0;
    let mut second = 1;

    if num == 0 {
        return first;
    }
    if num == 1 {
        return second;
    }

    for _ in 1..num - 1 {
        let third = second;
        second = first + second;
        first = third;
    }
    return second;
}
