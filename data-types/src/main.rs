fn main() {
    let tup = ("Kayle ", 10);
    let (name, age) = tup;
    // let arr = [200, 404, 500];
    let arr = [0; 10];

    println!("Array: {:?}", arr);

    // println!("Hello, world!");

    println!("Name: {}, age: {}", name, age);
    println!("Name: {}, age: {}", tup.0, tup.1);
}
