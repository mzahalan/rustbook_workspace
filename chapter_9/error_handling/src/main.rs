use std::fs::{self, File};
use std::io::ErrorKind;
use std::io::{self, Read};

// fn array_out_of_bounds() {
//     let v = vec!(1,2,3,4);
//     v[5];
// }

// Using Result to gracefully handle errors.
fn open_file(filename : &str) {
    let input_file = File::open(filename);

    match input_file {
        Ok(file) => println!("Successfully opened {}. File: {:?}", filename, file),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => println!("File {filename} not found, should we create it?"),
            _other_error => panic!("Problem opening file for some other reason"),
        }
    }
}

fn open_file_with_unwrap(filename : &str) {
    let input_file = File::open(filename).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            println!("File Not Found: {}", filename);
            panic!("I have to panic here because I have nothing to return to input_file");
            //This is where you could create it if it didn't exist
        } else {
            panic!("I have no idea what's going on right now. {:?}", error);
        }
    });
    println!("Input File: {:?}", input_file);
}

fn unwrap_with_auto_panic(filename : &str) {
    let input_file = File::open(filename).unwrap();
    let md = input_file.metadata().unwrap();

    println!("Opened file: {:?}", md);
}

fn open_with_expect(filename : &str) {
    let _input_file = File::open(filename).expect("I need the file to exist, but it doesn't");
}

fn get_contents(filename: &str) -> Result<String, io::Error> {
    let mut input_file = File::open(filename)?; // Panic if there's no file.

    let mut contents = String::new();
    input_file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn get_contents_chain(filename: &str) -> Result<String, io::Error> {
    let mut contents = String::new();
    File::open(filename)?.read_to_string(&mut contents)?;

    Ok(contents)
}

fn get_contents_shortcut(filename : &str) -> Result<String, io::Error> {
    fs::read_to_string(filename)
}

fn main() {
    // The commented out code demonstrates panic
    //panic!("Never going to print Hello World");
    //array_out_of_bounds();
    println!("Hello, world!");

    println!("\n----------- Using Result instead of dying -----------");
    open_file("Test.txt");
    open_file("Cargo.toml");

    println!("\n----------- Using unwrap -----------");
    //open_file_with_unwrap("test.txt");
    open_file_with_unwrap("Cargo.toml");
    unwrap_with_auto_panic("Cargo.toml");


    println!("\n----------- Using expect -----------");
    open_with_expect("Cargo.toml");


    println!("\n----------- Using question mark -----------");
    //println!("Contents: {}", get_contents("test.txt").unwrap());
    println!("Contents: {}", get_contents_chain("Cargo.toml").unwrap());

    println!("\n----------- Using the fancy shortcut read_to_string method ----------");
    println!("{}", get_contents_shortcut("Cargo.toml").unwrap());

}
