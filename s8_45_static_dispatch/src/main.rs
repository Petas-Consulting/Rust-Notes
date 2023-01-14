trait Printable
{
    fn format(&self) -> String;
}

impl Printable for i32
{
    fn format(&self) -> String
    {
        format!("i32: {}", *self)  // format for the output
    }
}

impl Printable for String
{
    fn format(&self) -> String
    {
        format!("string: {}", *self)  // format for the output
    }
}

// This gets monomorphized: we get type specific
// implementation.
fn print_it<T: Printable>(z: T)
{
    println!("{}", z.format());
}

fn main() {
    let a = 123;
    let b = "hello".to_string();

    println!("{}", a.format()); // gives the output i32: 123
    println!("{}", b.format()); // gives the output string: hello

    // This uses static dispatch. Compiler calls the
    // type specific / monomorphized implementation.
    print_it(a);   // gives the output i32: 123
    print_it(b);    // gives the output string: hello
}
