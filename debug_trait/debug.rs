#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

#[allow(dead_code)]
#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

fn main() {
    println!("{:?} months in a year", 12);
    println!("{:?} is the answer to {universe:?}", 42, universe="universe");

    println!("Structure {:?} is printable!", Structure(42));
    println!("But there is no control over formatting: {:?}", Deep(Structure(42)));

    let name = "Max";
    let age = 42;
    let me = Person { name, age };
    println!("Pretty person: {:#?}", me);
}
