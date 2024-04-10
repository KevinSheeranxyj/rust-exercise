fn main() {


    let number = false;
    if number {
        println!("number was there");
    }
    // Integer type
    let guess:u32 = "42".parse().expect("Not a number");

    // Scalar types

    let maximum_value:isize = 133;

    println!("Maximum value is {maximum_value}");


    println!("guess value: {guess}");


    // Floating-pointer type
    let x = 3.0;

    // Boolean type
    let t = true;
    let f: bool  = false;

    println!("Boolean type is {t}, {f}");

    // Character type

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    println!("heart_eyed_cat: {heart_eyed_cat}");

    let y:f32 = 3.0;
    println!("x value is {x}, y value is {y}");
}
