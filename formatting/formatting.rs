fn main() {
    println!("{} days", 42);
    println!("Positional {0} are {2} {1}.", "arguments", "important", "very");
    println!("And {named} {arguments} {too}", too="too", arguments="arguments", named="named");
    println!("Base 10: {}", 42);
    println!("Base 16: {:x}", 42);
    println!("And padded too: {x:0>width$}", x=42, width=10);

    let number: f64 = 42.42;
    let width: usize = 5;
    println!("Add capture also works: {number:0>width$}");

    // And a real exercise now!
    let pi: f32 = 3.141592;
    println!("Pi is roughly: {pi:.3}");
}
