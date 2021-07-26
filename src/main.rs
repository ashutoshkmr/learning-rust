fn main() {
    let tup: (i32, i32) = (4, 8);

    let (_x, _y) = tup;

    println!("x = {}, y = {}", tup.0, tup.1);

    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];

    println!("first = {}, second = {}", first, second);
    foo(second);

    let x = five();
    println!("The value of x is: {}", x);

    if x < 3 {
        println!("Condition is true");
    } else {
        println!("Condition is false");
    }

    let mut number = if true { 4 } else { 5 };

    println!("The value of number is: {}", number);

    while number != 0 {
        println!("The value of number is: {}", number);
        number = number - 1;
    }
}

fn foo(x: i32) {
    let y = {
        let x = 3;
        x + 1
    };

    println!("Foo = {}", x);
    println!("{}", y)
}
fn five() -> i32 {
    5
}
