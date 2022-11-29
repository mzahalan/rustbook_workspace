fn while_example() {
    println!( "//------- Example While Loop -------");
    let mut number = 10;
    while number != 0 {
        println!("number: {number}");
        number -= 1;
    }
}

fn for_example() {
    println!( "//------- Example For Loop -------");
    let a = [10, 20, 30, 40, 50, 60];

    for element in a {
        println!("The value is: {element}");
    }
}

fn for_range() {
    println!( "//------- Example Range Loop -------");
    
    // Note some new things here... 1..4 and .rev (reverse method)
    for element in (1..4).rev() {
        println!("The value is: {element}");
    }
}

fn main() {
    println!("Hello, world!");
    while_example();
    for_example();
    for_range();
    /* 
    // IF Statement
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // Error - number is not a boolean
    /*
    if number {
        println!("number was three");
    }
    */


    // IF with ELSE
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // IF in an assignment (like ternary)
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");


    // Using Loop with break and continue.
    // and a return value
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 4 {
            continue;
        }

        println!("Counter: {counter}");

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");

    // Labeling a loop so you can reference it from an inner loop.
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End Count: {count}");
*/
}
