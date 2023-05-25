

// You can have one or more &T values at any given time, or
// You can have exactly one &mut T value.


fn main() {
    let mut a: i32 = 10;
    let b: &i32 = &a;
    
    println!("b: {b}");

    {
        let c: &mut i32 = &mut a;
        *c = 20;
    }

    println!("a: {a}");
    //println!("b: {b}");
}