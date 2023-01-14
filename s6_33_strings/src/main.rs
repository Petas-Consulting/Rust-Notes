fn strings()
{
    // Rust has 2 different string types.

    //
    // A string slice. It is statically allocated.
    //
    let s: &'static str = "hello there!";
    // Cannot do as follow:
    // s = "abc";
    // let h = s[0];

    for c in s.chars()
    {
        println!("{}", c);
    }  // looping the string s

    for c in s.chars().rev()
    {
        println!("{}", c);
    } // looping the string s in a reverse order

    if let Some(first_char) = s.chars().nth(0) // calling for the first letter in the string
    {
        println!("first letter is {}", first_char);
    }

    //
    // A `String`. Heap allocated.
    // Can be modified.
    let mut letters = String::new();
    let mut a = 'a' as u8;
    while  a <= ('z' as u8)
    {
        letters.push(a as char);
        letters.push_str(",");
        a += 1;
    }
    println!("{}", letters);  // looping through letters and a to giving out all english alphabets

    // &str <> String
    // let u:&str = &letters;

    // concatenation
    // String = str
    // let z = letters + "abc";
    // let z = letters + &letters;

    let mut abc = "hello world".to_string();
    abc.remove(0);
    abc.push_str("!!!");  // add !!! to value of abc
    println!("{}", abc.replace("ello", "goodbye")); // changing ello in value to goodbye
}

fn main() {
    strings();
}
