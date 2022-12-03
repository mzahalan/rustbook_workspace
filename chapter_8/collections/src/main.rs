use std::{collections::HashMap};




fn first_vector() {
    println!("\n\n\n------- First Vector -------");
    let mut v: Vec<i32> = Vec::new();
    v.push(10);
    if let Some(x) = v.pop() {
        println!("Popped: {x}");
    }
    println!("Vector Size: {}", v.len());
}

fn vector_from_macro() {
    println!("\n\n\n------- From Macro -------");
    let v = vec![1, 2, 3];
    for (i,x) in v.iter().enumerate() {
        println!("Vector[{i}] : {x}");
    }
}

fn using_get() {
    println!("\n\n\n------- Using Get -------");
    let v = vec![1,2,3,4,5,6];
    // The following code compiles but at runtime it panics... Accessing array out of bound is a runtime error.
    // let tenth = &v[9];
    // println!("The 10th element is: {}", tenth);

    // Using get is safer, as it returns an optional.
    if let Some(tenth) = v.get(9) {
        println!("The 10th value is: {tenth}");
    } else {
        println!("Error: There is no 10th value");
    }
}

fn basic_iterating() {
    println!("\n\n\n------- Basic Iterating -------");
    let mut v = vec![1,2,3,4,5,6];

    // The difference between &v and v is subtle.
    // I believe using &v gives you a reference to the element in the vector.
    // That's better than doing a shallow copy for each element.
    for i in &v {
        println!("{i}");
    }

    // This next bit is pretty tricky. We've got a mutable reference to an element in a vector.
    // In order to change the value at that address, we have to dereference using the dereference operator.
    for i in &mut v {
        *i += 50;
    }

    for i in &v {
        println!("{i}");
    }
    
}


#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

// This example is a little crazy... The vector is of type enum
// But the enum elements are of different types. So this is kinda like a 
// version of polymorphism.
// It looks like when an enum has different item-types, it finds the largest one, and
// that's how big each of them are. That's the only way this works.
// Because you can define methods for enum, this is how you could have a collection of "renderables"
// or other "ables" that all implement the same interface.
fn vec_with_enum() {
    println!("\n\n\n------- Vec of Enums -------");
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for x in &row {
        println!("Cell: {:?}", x);
    }
}


fn string_example() {
    println!("\n\n\n------- Strings -------");

    let mut s = String::from("foo");
    let mut y = "bar".to_string();

    s.push_str(&y);  // Important to note here that push_str takes a slice reference &str
                            // this means we have to give it borrowed reference. IE: push_str doesn't
                            // take ownership of y, it gives it back when the method is done.

    y.push('1');
    println!("String s: {s}");
    println!("String y: {y}");
}

fn combining_strings() {
    println!("\n\n\n------- Combining Strings -------");
    let s1 = String::from("foo");    
    println!("String 1: {s1}");
    let s2 = String::from(" bar");
    let s3 = s1 + &s2;              // Note S1 is moved to S1 (it's not a reference)
                                            // so you can't try to read from it again.

    println!("String 2: {s2}");             // S2 was only borrowed for the calculation. We can access
                                            // it here. (unlike S1)
    println!("String 3: {s3}");


    println!("\n\n\n------- Handy format macro -------");
    let s1 = String::from("Tic");
    let s2 = String::from("Tac");
    let s3 = String::from("Toe");
    let s = format!("{}-{}-{}", s1,s2,s3);
    println!("{s}");


    // Note - you can't index into a string because it's not gauranteed that 
    // each character in the string is 1 byte, yet the string is stored as a byte array.
    // so [] doesn't work.

    // Slicing [..] does work though. However it'll panic if you index into a non-char boundary.
    // for example if a char is 2 bytes, and you do something like [1..3], it'll bomb.
}

fn accessing_strings() {
    // Note - you can't index into a string because it's not gauranteed that 
    // each character in the string is 1 byte, yet the string is stored as a byte array.
    // so [] doesn't work.

    // Slicing [..] does work though. However it'll panic if you index into a non-char boundary.
    // for example if a char is 2 bytes, and you do something like [1..3], it'll bomb.

    println!("\n\n\n------- String Slicing -------");
    let hw = String::from("Hello World");
    let s = &hw[0..=4];                         // using the = makes it inclusive.
    println!("{}[0..=4] => {}", hw, s);
    println!("{}[0..4] => {}", hw, &hw[0..4]);        // default is non inclusive end boundary.


    println!("\n\n\n------- Iterating over Chars -------");
    for c in "Зд".chars() {
        println!("{}", c);
    }

    // Note how this string has 2 chars, but 4 bytes.
    println!("\n\n\n------- Iterating over Bytes -------");
    for c in "Зд".bytes() {
        println!("{}", c);
    }
}

fn hash_maps() {
    println!("\n\n\n------- Hashmaps -------");
    let mut scores = HashMap::new(); // Key and Value types are inferred from below.

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 40);
    scores.insert(String::from("Yellow"), 5);

    let score = scores.get("Blue").copied().unwrap_or(0);

    // Some things to explain up there ^^ .copied() gets a copy of the item instead of a reference to it.
    // This is helpful here because Option<i32> is easier to unwrap than Option<&i32>.
    // unwrap_or(0)... This is fancy - scores will either == an i32 value, or fall back to 0 if the key isn't
    // found.
    println!("Blue Team Score: {:?}", score);

    // Iterate over a borrowed reference - not a copy.
    // Order isn't gauranteed for hashmaps.
    for (key, value) in &scores {
        println!("Key: {key} Value: {value}");
    }

    // By Default - inserting a value into a hashmap is done by value - IFF that type implements Copy.
    // If it doesn't implement copy, then it becomes an owned value, and the original variable is unusable.
}

fn update_map() {
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
fn main() {
    first_vector();
    vector_from_macro();
    using_get();
    basic_iterating();
    vec_with_enum();
    string_example();
    combining_strings();
    accessing_strings();
    hash_maps();
    update_map();
}
