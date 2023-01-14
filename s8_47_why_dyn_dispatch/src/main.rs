struct Circle { radius: f64 }
struct Square { side: f64 }

trait Shape {
    fn area(&self) -> f64;
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }   // setting fomula to solve for area of a square
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        self.radius * self.radius * std::f64::consts::PI
    }  //setting fomula to solve for area of a circle
}


fn main() {
    let shapes:[&dyn Shape; 4] = [
        &Circle{radius: 1.0},
        &Square{side: 3.0},
        &Circle{radius: 2.0},
        &Square{side: 4.0}
    ];   // circle uses radius and square uses side 

    for (i, shape) in shapes.iter().enumerate()  // iterating over the ssides and radius
    {
        println!("Shape #{} has area {}", i, shape.area());
    }
}
