fn main() {
    let mut x = 5;
    { // create a new scope to ensure the mutable reference is released before creating another 
        let y = &mut x; // y is a mutable reference to x
        *y = 10; // Modifying x through y
    }
    
    { // create a new scope to ensure the mutable reference is released before creating another 
        let z = &mut x; // z is a mutable reference to x
        *z = 20; // Modifying x through z
    }
    
    println!("x = {}", x);
} 