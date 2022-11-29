fn new_function() {
    println!("Hello from inside new_function");
}

fn main() {
    println!("Hello, world!");
    new_function();
    another_function(5); // Example passing a parameter.
    
    multiple_params(1234, "ASDF".to_string());

    println!("ugly_return: {}", ugly_return(10));
}

fn another_function(x : u32) {
    println!("X: {x}");
}

fn multiple_params(x:u32, y:String) {
    println!("x: {x}, y:{y}");
}

fn ugly_return(x:u32) -> u32 {
    x-5 //Expressions don't have a semi-colon. Also no need for "return"
}

// This function doesn't compile. Can't assign to X.
//fn bad_function(x : u32) {
//    x = 15;
//    println!("X: {x}");
//}