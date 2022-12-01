use std::net::{Ipv4Addr, Ipv6Addr};
use rand::Rng;

#[derive(Debug)]
enum IpAddrKindBasic {
    V4,
    V6
}


fn route(kind: IpAddrKindBasic) {
    println!("//---- Example using an enum instance in a method");
}

fn basic_enum() {
    let four = IpAddrKindBasic::V4;
    route(four);
    route(IpAddrKindBasic::V6);  // Kinda interesting
}

#[derive(Debug)]
enum IpAddr {
    V4(u8,u8,u8,u8),
    V6(String),
}

fn enum_with_data() {

    println!("\n//---- Putting Data Directly Into an Enum");
    let home = IpAddr::V4(192,168,1,1);
    let loopback = IpAddr::V6(String::from("::1"));
    println!("{:?}", home);
    println!("{:?}", loopback);
}

#[derive(Debug)]
enum IpAddr2 {
    V4(Ipv4Addr),   // Fance IDE added the use statement up top 
    V6(Ipv6Addr),   //
}
fn using_std_ip_def() {
    println!("\n//---- Using Stdlib IP types");
    let home = IpAddr2::V4(Ipv4Addr::new(192, 168, 1, 1));
    println!("{:?}", home);
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

impl Message {
    fn print(&self) {
        println!("{:?}", self);
    }
}


fn mixed_types_example() {
    println!("\n//---- Enum with Mixed Types");
    let mv = Message::Move { x: 5, y: 6 };
    let write = Message::Write(String::from("Hello"));
    mv.print();
    write.print();
}

fn get_int() -> Option<i32> {
    let secret_number = rand::thread_rng().gen_range(1..=2);
    if secret_number == 2 {
        return Some(2)
    }
    None
}

// This is a contrived example. Basically we're calling a method that returns an
// Option<i32>. If that Option has a value we print it. Otherwise we print none.
// There are certainly better ways to handle Option<T> return types.
fn option_example() {
    println!("\n//---- option example");
    let x = get_int();
    if x.is_some() {
        println!("X has a value: {}", x.unwrap());
    } else {
        println!("X has no value");
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}
fn get_value(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => {
            println!("Shiny and new");
            5
        },
        Coin::Dime => 10,
        Coin::Quarter => 25, 
    }
}
fn basic_match() {
    println!("\n//------- basic match example -------");
    println!("Value of a Nickle is {}", get_value(Coin::Nickel));
}

fn match_option() {
    println!("\n//------- Match Option -------");
    match get_int() {
        None => println!("None Returned"),
        Some(i) => println!("Found {i}"),
    };
}

fn get_dice() -> i32 {
    rand::thread_rng().gen_range(1..=6)
}

// 
// This is another contrived example, however
// the point is to illustrate the catchall "other"
// this is similar to default
fn match_dice() {
    println!("\n//------- Match Dice -------");
    match get_dice() {
        1 => println!("1"),
        3 => println!("3"),
        5 => println!("5"),
        other => println!("Even: {}", other),
    };
}

fn if_let() {
    println!("\n//------- If - Let example -------");

    if let Some(thing) = get_int() {
        println!("The integer is: {}", thing);
    } else {
        println!("There is no integer.");
    }
}

fn main() {
    println!("Hello, world!");
    basic_enum();
    enum_with_data();
    using_std_ip_def();
    mixed_types_example();
    option_example();
    basic_match();
    match_option();
    match_dice();
    if_let()
}
