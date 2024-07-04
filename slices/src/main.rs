fn main() {
    let s = String::from("hello world");

    // let hello_slice = &s[0..4];
    // let world_slice = &s[6..];

    let word = get_first_word(&s);
    println!("The first word is: {}", word);
}

fn get_first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
