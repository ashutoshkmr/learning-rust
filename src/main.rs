extern crate rand;

fn main() {
    println!("Guess the number!");

    let tup: (i32, i32) = (4, 8);

    let (_x, _y) = tup;

    println!("x = {}, y= {}", tup.0, tup.1)
}
