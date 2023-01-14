fn scope_and_shadowing()
{
    let a = 123;
    {
        let b = 456;
        println!("inside, b = {}", b);
        let a = 777;
        println!("inside, a = {}", a);
    }

    println!("outside, a = {}", a);  // value for a will be 123 because a is outside the inside closure
    // println!("outside, b = {}", b);

}


fn main() {
    scope_and_shadowing();
}
