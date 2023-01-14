
fn while_and_loop()
{
    let mut x = 1;

    while x < 1000
    {
        x *= 2;   // x = x * 2

        if x == 64 { continue; }  // this will stop at 64, skip it and continue with the next number 

        println!("x = {}", x);  
    }

    let mut y = 1 ;
    loop
    {
        y *= 2;    // y = y * 2
        println!("y = {}", y);

        if y >= 1<<10 { break; }
    }
}

fn main() {
    while_and_loop();
}
