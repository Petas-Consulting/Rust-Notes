use std::fmt::Debug;
// Area of a Circle
#[derive(Debug)]
struct Circle {
    radius: f64,
}

#[derive(Debug)]
struct Square {
    side: f64,
}

trait Shape {
    fn area(&self) -> f64;
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        self.radius * self.radius * std::f64::consts::PI
    }// std::f64::consts::PI is getting the value of PI
}// formula for area of Circle is radius**2 * PI

// 3 ways of specifying trait constraints:
// fn print_info(shape: impl Shape + Debug)
// fn print_info<T: Shape + Debug>(shape: T)
fn print_info<T>(shape: T)
  where T: Shape + Debug
{
    println!("{:?}", shape);
    println!("The area is {}", shape.area());
}

fn main() {
    let c = Circle{ radius: 2.0 };
    print_info(c);
}
