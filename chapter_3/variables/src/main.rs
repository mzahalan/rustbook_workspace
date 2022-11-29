
// Constants can be defined in the global scope.
// Also Constants must always explicitely declare the type.
// Also Constants cannot be assigned the result of something evaluated at runtime.
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

// ERROR -  you can't have variables defined in the global scope.
//let x = 6;

fn main() {
    

    //---------------------- Mutability ---------------------------------------
    // If we don't make this mutable - the assignment below will fail.
    // let x = 5;

    // Instead we use the mut keyword, and the assignment will pass.
    let mut x = 5;

    println!("The value of X is {x}");
    
    // the following line errors if we don't define the variable as mutable.
    x = 6;
    println!("The value of X is {x}");

    //---------------------- Shadowing ---------------------------------------
    {
        // This new definition of X shadows the previous definition.
        // it will be what the compiler uses for the remainder of this scope.
        let x = "test";
        println!("The value of x is {x}");
    }
    println!("The value of X is {x}");

    // This is kinda neat...
    let spaces = "   ";
    let spaces = spaces.len();
    // spaces was an unmutable str&, but we redefine it as an int.
}
