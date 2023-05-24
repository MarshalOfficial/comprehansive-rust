fn main() {
    let array: [i32; 3] = [10,20,30];
    print!("Iterating over array:");
    for n  in array {
        print!(" {n}");
    }
    println!();

    print!("Iterating over array via foreach:");
    array.into_iter().for_each(|n| {
        print!(" {n}");
    });
    println!();

    print!("Iterating over range:");
    for i in 0..3{
        print!(" {}", array[i]);
    }
    println!();

}
