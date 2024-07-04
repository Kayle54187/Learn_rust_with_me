fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!(
        "{:?}",
        match six {
            Some(i) => i,
            None => 0,
        }
    );
    println!("{:?}", none);

    // using if let

    if let None = six {
        println!("None");
    } else {
        println!("{}", six.unwrap());
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

// or

// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         Some(i) => Some(i + 1),
//         _ => None,
//     }
// }
