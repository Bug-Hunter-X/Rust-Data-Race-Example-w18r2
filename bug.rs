fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &mut x; // z is ALSO a mutable reference to x

    *y = 10; // Modifying x through y
    *z = 20; // Modifying x through z (data race!)

    println!("x = {}", x);
}