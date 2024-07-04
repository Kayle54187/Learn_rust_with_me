#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let user1 = User {
        email: String::from("kayle@gmail.com"),
        username: String::from("kayle"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = build_user(String::from("kingvader@gmail.com"), String::from("vader"));

    let username1 = user1.username;

    let user3 = User { ..user2 };

    println!("username1: {}", username1);
    println!("username2 active: {}", user2.active);
    println!("User 3 object: {:#?}", user3);

    rectangle_main();
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn rectangle_main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let square = Rectangle::square(10);

    println!("rect1 is {:#?}", rect1);
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold square? {}", rect1.can_hold(&square));
}
