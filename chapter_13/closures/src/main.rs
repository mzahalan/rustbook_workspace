
#[derive(Debug, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preferences: Option<ShirtColor>) -> ShirtColor {
        user_preferences.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for shirt in &self.shirts {
            match shirt {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn inventory_closure() {
    println!("\n\n------- Inventory Closure Example -------");
    let store = Inventory { 
        shirts: vec![ShirtColor::Blue, ShirtColor::Blue, ShirtColor::Red],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let user_pref2 = None;

    let giveaway1 = store.giveaway(user_pref1);
    println!("The user with preference {:?} gets shirt {:?}", user_pref1, giveaway1);
    println!("The user with preference {:?} gets shirt {:?}", user_pref2, store.giveaway(user_pref2));

}

// The closure figured out that we only need an immutable reference to the
// list. The closure decides between: borrowing immutably, borrowing mutably, 
// and taking ownership based on what the body does.

fn capturing_immutable_reference() {
    println!("\n\n-------Capturing immutable references -------");
    let list = vec![1,2,3];
    println!("Before defining closure: {:?}", list);
    let only_borrows = || println!("From Closure: {:?}", list);
    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);
}

fn capturing_mutable_reference() {
    println!("\n\n-------Capturing immutable references -------");
    let mut list = vec![1,2,3];
    println!("Before defining closure: {:?}", list);
    let mut only_borrows = || {
        println!("From Closure: {:?}", list);
        list.push(5);
    };
    
    // It would be a compiler error to try to use list before only_borrows()
    // only_borrows gets a mutable borrow, and only 1 of those are allowed.
    //println!("Before calling closure: {:?}", list);

    only_borrows();
    println!("After calling closure: {:?}", list);
}

use std::thread;

// This example is pretty neat.
// We don't really need to take ownership, but we can use the move keyword to force it.
fn capturing_with_move() {
    println!("\n\n-------Capturing with move -------");
    let list = vec![1,2,3];
    println!("Before defining closure: {:?}", list);
    
    thread::spawn(move || println!("from thread: {:?}", list)).join().unwrap();
}

fn custom_sort() {

    println!("\n\n------ Custom Sort -------");

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let mut list = [
        Rectangle {width: 10, height: 5},
        Rectangle {width: 15, height: 15},
        Rectangle {width: 12, height: 25},
    ];

    list.sort_by_key(|r| r.width);
    println!("Sorted List: {:#?}", list);
}

fn iter_example() {

    println!("\n\n------ Iterators -------");
    let vec = vec![1,2,3,4];

    let iter = vec.iter();

    let total : i32 = vec.iter().sum();
    println!("Sum of {:?} = {}", vec, total);
}

fn map_example() {
    println!("\n\n------ Map -------");
    let vec = vec![1,2,3,4];
    let new_vec : Vec<_> = vec.iter().map(|x| x+1).collect();
    println!("Old Vec: {:?}; New Vec: {:?}", vec, new_vec);
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

fn filter_example() {
    println!("\n\n------ Map, Filter, Collect -------");
    let shoes = vec![
        Shoe { size: 5, style: String::from("Sneaker") },
        Shoe { size: 6, style: String::from("Dress Shoe") },
        Shoe { size: 7, style: String::from("High Top") },
        Shoe { size: 5, style: String::from("Low Top") }
    ];

    let shoes_my_size = shoes_in_size(shoes, 5);
    println!("{:#?}",shoes_my_size);

    //println!("Original list: {:#?}", shoes);
}

fn main() {
    println!("Hello, world!");
    inventory_closure();
    capturing_immutable_reference();
    capturing_mutable_reference();
    capturing_with_move();
    custom_sort();
    iter_example();
    map_example();
    filter_example();
}
