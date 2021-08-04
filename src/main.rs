extern crate communicator;

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32);

struct Point(i32, i32, i32);

fn main() {
    let mut user1 = build_user(
        String::from("ashutoshkmr40@gmail.com"),
        String::from("Ashutosh Kumar"),
    );

    user1.email = String::from("someemail@example.com");

    // Struct update syntax
    let user2 = User {
        username: String::from("Some username"),
        email: String::from("anotheremail@example.com"),
        ..user1
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let rectangle = Rectangle {
        width: 20,
        height: 25,
    };

    println!("The area of rectangle is {}", rectangle.get_area());
    println!("The rectangle is {:#?}", &rectangle);

    let rect1 = Rectangle {
        width: 30,
        height: 55,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2 ? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect2 ? {}", rect1.can_hold(&rect3));

    let amount = Coin::Dime;

    println!("Value in cents for amount is {}", amount.value_incents());

    communicator::client::connect();
}

fn build_user(email: String, username: String) -> User {
    User {
        username,
        email,
        sign_in_count: 1,
        active: true,
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn get_area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rectangle: &Rectangle) -> bool {
        rectangle.width < self.width && rectangle.height < self.height
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quater,
}

impl Coin {
    fn value_incents(&self) -> u32 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quater => 25,
        }
    }
}
