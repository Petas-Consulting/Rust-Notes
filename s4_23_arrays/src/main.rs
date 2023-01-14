use std::{mem, usize};

// use std;

fn arrays()
{
    // Redundant type declaration.
    // let mut a:[i32; 5] = [1, 2, 3, 4, 5];
    let mut a = [1, 2, 3, 4, 5]; 
    println!("a has {} elements, first is {}",
        a.len(), a[0]);    // len() fuction is use to know the number of elements we have in a array
                            // a[0] calls for the first item in the array

    a[0] = 321;   // changing the value of a[0] to 321
    println!("a[0] = {}", a[0]);   

    // Nice debug print notation to print the full array:
    println!("{:?}", a);   // out of array a; [321, 2, 3, 4, 5]

    if a != [1, 2, 3, 4, 5]
    {
        println!("does not match");
    }

    if a != [321, 2, 3, 4, 5]
    {
        println!("match");
    }   

    // Cannot compile with array of different lenght.
    // if a != [1, 2, 3, 4, 5, 6]

    let b = [1; 10];
    for i in 0..b.len()
    {
        println!("b[{}] = {}", i, b[i]);
    }

    println!("b took up {} bytes", mem::size_of_val(&b));

    let mtx:[[f32;3];2] =
    [
        [1.0, 0.0, 0.0],
        [0.0, 2.0, 0.0]
    ];   // creating a matrix using array

    println!("{:?}", mtx);

    for i in 0..mtx.len()
    {
        for j in 0..mtx[i].len()
        {
            println!("mtx[{}][{}] = {}", i, j, mtx[i][j]);
        }
    }   // showing the matrix index and the value
}

fn main() {
    arrays();
}
