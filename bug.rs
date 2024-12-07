fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    *y += 1;     // Modifies x through y
    let z = &mut x; // this will cause a compile time error
    *z += 1;
    println!("{}", x);
}