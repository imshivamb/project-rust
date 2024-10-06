// struct Rect {
//     width: u32,
//     height: u32,
// }

// struct Circle {
//     radius: u32,
// }

// impl Circle {
//     fn circumference(&self) -> f64 {
//         return 2.0 * self.radius as f64 * std::f64::consts::PI;
//     }
// }

// impl Rect {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }

// fn main() {
//     let rect = Rect {
//         width: 25,
//         height: 44,
//     };
//     let circle = Circle { radius: 10 };

//     println!("the Area of rectangle is: {}", rect.area());
//     println!("The Circumference of circle is: {}", circle.circumference());
// }

enum Shape {
    Square(f32),
    Rectangle(f32, f32),
    Circle(f32),
}

fn main() {
    let rect = Shape::Rectangle(10.0, 20.3);
    let sq = Shape::Square(4.5);
    let circle = Shape::Circle(4.0);
    println!("Area of rectangle is: {}", find_area(rect));
    println!("Area of square is: {}", find_area(sq));
    println!("Area of circle is: {}", find_area(circle));
}

fn find_area(shape: Shape) -> f32 {
    let area = match shape {
        Shape::Circle(r) => std::f32::consts::PI * r.powi(2),
        Shape::Rectangle(w, h) => w * h,
        Shape::Square(s) => s * s,
    };
    return area;
}
