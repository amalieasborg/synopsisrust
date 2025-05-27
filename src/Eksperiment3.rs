fn main() {
    let s1 = String::from("Eksempel");
    let s2 = s1; // s1 bliver flyttet til s2
    println!("{}", s2); // virker
    //println!("{}", s1); // fejl! s1 ejer ikke længere data
}