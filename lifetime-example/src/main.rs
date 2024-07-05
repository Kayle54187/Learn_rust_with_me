#[derive(Debug)]
struct SomeStruct {
    value: i32,
}

fn greatest<'a>(a: &'a SomeStruct, b: &'a SomeStruct) -> &'a SomeStruct {
    if a.value > b.value {
        a
    } else {
        b
    }
}

fn main() {
    let a = SomeStruct { value: 10 };
    let result: &SomeStruct;

    {
        let b = SomeStruct { value: 20 };
        result = greatest(&a, &b); // This returns an error
    }

    println!("Result {:?}", result);
}
