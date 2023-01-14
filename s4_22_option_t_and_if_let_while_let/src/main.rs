fn main() {
    let x = 3.0;
    let y = 0.0;

    let result =
        if y != 0.0 { Some( x / y )} else { None };
    // match allows you to compare a value against a series of patterns and then execute code based on which pattern matches.
    match result {    
        // None and Some are the variants of the enum, that is, a value with type Option<T> can either be a None, or it can be a Some containing a value of type T. 
        //You can create the Option enum using the variants as well
        Some(z) => println!("{}/{}={}", x, y, z),
        None => println!("cannot divide by zero")
    }

    if let Some(z) = result {
        println!("result={}", z);
    }

    let mut opt_v = Some(5);

    while let Some(v) = opt_v {
        println!("opt_v = {}", v);  // giving the output of opt_v
        opt_v = None;
    }
}
