fn if_statement()
{
    let temp = 35;
    if temp > 30
    {
        println!("really hot outside");
    }
    else if temp < 10
    {
        println!("really cold!");
    }
    else
    {
        println!("temparature is OK");
    }

    let day = if temp > 20 {"sunny"} else {"cloudy"};
    println!("today is {}", day);   // result will be sunny because temp is greater than 20

    println!("is it {}",
        if temp > 20 {"hot"} else if temp < 10 {"cold"} else {"OK"});  // result will be hot because temp is greater than 20

    println!("it is {}",
        if temp > 20 {
            if temp > 30 {"very hot"} else {"hot"}
        } else if temp < 10 {"cold"} else {"OK"})  // result will be very hot because temp is greater than 20 and 30
}

fn main() {
    if_statement();
}
