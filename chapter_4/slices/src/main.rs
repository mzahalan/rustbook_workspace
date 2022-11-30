
// Here’s a small programming problem: write a function that takes a string 
// of words separated by spaces and returns the first word it finds in that 
// string. If the function doesn’t find a space in the string, the whole 
// string must be one word, so the entire string should be returned.

fn find_first_word(x:&str) -> &str{

    for (i, c) in x.chars().enumerate() {
        println!("{i} : {c}");
        if c == ' ' {
            println!("Found the first break");
            return &x[..i];
        }
    }

    &x[..] // Returns the whole thing
}

fn get_slice(x:&[i32]) -> &[i32] {
    &x[1..3]
}

fn array_slice_example() {
    let x = [1,2,3,4,5,6];
    let y = get_slice(&x);
    for (i,z) in y.iter().enumerate() {
        println!("{i}: {z}");
    }

    let bomb = get_slice(&[0,1]); // BOOM

}

fn main() {
    let x = String::from("Hello World");
    let y = find_first_word(&x);
    println!("Y: {}", y);

    array_slice_example();
}
