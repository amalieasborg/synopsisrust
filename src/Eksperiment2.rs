use std::mem;

fn main() {
    let x=42;
    let y=Box::new(42);

    println!("Size of i32 on stack: {} bytes", mem::size_of_val(&x));
    println!("Size of Box<i32> on stack: {} bytes", mem::size_of_val(&y));
}