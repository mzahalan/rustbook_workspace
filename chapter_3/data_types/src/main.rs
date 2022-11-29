fn main() {
    println!("Hello, world!");


    let a : u8 = 255;
    let b : i8 = -127;
    let c : u16 = 16+1;
    let x :isize = 232323;
    let y = 15u8;
    println!("A: {a}, B: {b}, C: {c}, X: {x}, Y: {y}");

    // Floats
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    println!("X: {x}, Y: {y}");

    // Boolean
    let mut x = true;
    println!("X: {x}");
    x = !x;
    println!("X: {x}");

    // Chars
    let c = 'z';  // Note teh single quotes for char!
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("c: {c}, z: {z}, hec {heart_eyed_cat}");

    // Tuples
    let x: (i32, char, u8) = (1234, 'a', 255);
    //println!("Tuple ? {x}"); // Error Here - Tuple can't be formated with default formatter.
    println!("Tuple ( {} , {} , {} )", x.0, x.1, x.2);
    let (a,b,c) = x;
    println!("a: {a}, b: {b}, c: {c}");


    // Arrays
    println!("------- Arrays ------");
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    println!("Month 4: {}", months[3]);

    let a: [i32; 5] = [1, 2, 3, 4, 5];  // This array is defined to have 5 elements.
    let a = [3; 5]; // equivalent to [3,3,3,3,3]
    //println!("{a}");  // Error - arrays can't print with default formatter!
    println!("[{} {} {} {} {}]", a[0], a[1], a[2], a[3], a[4]);
    //println!("Beyond the array: {}", a[6]); // Compile time error saying it'll panic at runtime.


}
