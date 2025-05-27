fn main() {
    let mut s  = String::from("Eksempel");

    let r1 = &s;

    let r2 = &s;

    let r3 = &mut s;

    println!("{}", r1);
}