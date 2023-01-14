fn for_loop()
{
    for x in 1..11
    {
        if x == 3 { continue; }  // this will stop at 3, skip it and continue with the next number 


        if x == 8 { break; }     // this will stop at 7 
        println!("x = {}", x);
    }

    for (pos, y) in (30..41).enumerate()   // defining the pos for y where y will be from 30 to 41
    {
        println!("{}: {}", pos, y);
    }
}

fn main() {
    for_loop();
}
