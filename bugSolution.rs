fn main() {
    let mut x = 5;
    { // Create a new scope for the mutable borrow
        let y = &mut x;
        *y += 1;  // Modify x through y
    }
    let z = &mut x;
    *z += 1;
    println!("x = {}", x); // x will be 7
}