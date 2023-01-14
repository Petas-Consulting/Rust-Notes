struct Point
{
    x: f64,
    y: f64
}

struct Line
{
    start: Point,
    end: Point
}

fn structures() {
    let p = Point { x: 3.0, y: 4.0 };  // declaring p.x = 3.0 and p.y = 4.0
    println!("point p is at ({},{})", p.x, p.y);   

    let p2 = Point { x: 5.0, y: 10.0};    // declaring p.x = 3.0 and p.y = 4.0
    let myline = Line { start: p, end: p2 };   
    println!(
        "myline.start.y = {}, myline.end.x = {}",  // y will be 4 and x will be 5
        myline.start.y, myline.end.x);
}

fn main() {
    structures();
}
