// use std::vec;

fn vectors()
{
    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);
    // all the value will be pushed into variable a 
    println!("a = {:?}", a);
    
    a.push(44);

    println!("a = {:?}", a);

    let idx: usize = 0;

    a[idx] = 312;
    println!("a[0] = {}", a[idx]);

    // Return an option type when out of bound index.
    match a.get(6)
    {
        Some(x) => println!("a[6] = {}", x),
        None => println!("error, no such element")
    } // there is no such value of 6 in a so the result will be None

    if let Some(x) = a.get(6)
    {
        println!("a[6] = {}", x);
    }

    for x in &a { println!("{}", x); }

    a.push(77);
    println!("{:?}", a);

    let last_elem = a.pop();
    println!("last elem is {:?}, a = {:?}", last_elem, a);

    while let Some(x) = a.pop() //pop() will remove the last item in the list
    {
        println!("{}", x);
    }
}

fn main() {
    vectors();
}
