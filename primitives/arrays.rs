use std::mem;

fn analize_slice(slice: &[i32]) {
    println!("first item: {}", slice[0]);
    println!("len: {}", slice.len());
}

fn main() {
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    let ys: [i32; 500] = [0; 500];

    println!("Array occupies {} bytes", mem::size_of_val(&xs));
    analize_slice(&xs);

    analize_slice(&ys[1 .. 4]);

    let empty_array: [i32; 0] = [];
    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]);

    for i in 0..xs.len() + 1 {
        match xs.get(i) {
            Some(x) => println!("{}: {}", i, x),
            None => println!("Slow down! {} is too far!", i),
        }
    }
}
