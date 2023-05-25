//While move semantics are the default, certain types are copied by default:
//These types implement the Copy trait.
fn main() {
    let x = 42;
    let y = x;
    println!("x: {x}");
    println!("y: {y}");


    //it depends on Point type
    let p1 = Point(3, 4);
    let p2 = p1;
    println!("p1: {p1:?}");
    println!("p2: {p2:?}");
}

//You can opt-in your own types to use copy semantics:
#[derive(Copy, Clone, Debug)]
struct Point(i32, i32);