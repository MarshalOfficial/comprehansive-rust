fn main() {
    let a = 10;
    println!("before: {a}");

    {
        let a = "hello";
        println!("inner scope: {a}");

        let a = true;
        println!("shadowed in inner scope: {a}");
    }

    println!("after: {a}");


    let z = 1;
    let g = &z;
    let z = z + 1;
    println!("{z} {g}");
}