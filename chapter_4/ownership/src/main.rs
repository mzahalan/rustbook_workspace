
fn print_string(foo:String) -> String {
    println!("{foo}");
    foo
}
fn pass_back_example() {
    println!("//--------  Pass by Value, and return value  -------");
    let mut x = String::from("foo");
    x = print_string(x);
    println!("{x}");
}


fn get_length(foo:&String) -> usize {
    foo.len()
}
fn example_pass_by_reference() {
    println!("//--------  Pass By Reference  -------");
    let mut x = String::from("foo");
    get_length(&x);
    println!("{x}");
}

fn add_bar(bar : &mut String) {
    bar.push_str(" bar");
}

fn example_mutable_reference() {
    println!("//--------  Mutable Reference  -------");
    let mut x = String::from("foo");

    add_bar(&mut x);
    println!("{x}");

    //Note - you can only have 1 mutable reference to something.
    // But you can have many immutable references.
    // You also can't have a mutable and immutable reference in scope
    // at the same time.
}

fn main() {
    pass_back_example();
    example_pass_by_reference();
    example_mutable_reference();
/*
    // This is a string literal (not a String type)
    // It's hardcoded, so it can literally exist on the stack.
    // Even if you declare it mutable, you can't mutate it.
    let s = "hello";
    println!("{s}, world!");

    // Mutable Heap String! (also shadowing ftw)
    let s = String::from("Hello");
    let y = s;

    //println!("{s}");  // This would be a problem, because s isn't an owner anymore.
    println!("{y}");

    // Here's a deep copy using clone
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);


    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

    //println!("{s}");    // ERROR.
    println!("{x}");    // works.
*/

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.