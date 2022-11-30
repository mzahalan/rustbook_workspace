
// Structs apparently use Upper Case by convention.
// Also notice that username is a String... that means that by default
// the "shallow copy" will invalidate the previous reference (borrowing)

#[derive(Debug)]        // We get to this later, but it's to make printing easier.
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// We don't actually assign it, we'll just return it.
fn create_user() -> User {
    println!("//------- Creating a \"User\" -------");
    User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    }
}

fn build_user(email: String, username: String) -> User {
    println!("//------- Build a \"User\" -------");
    User {
        email,              // Automatically assigned based on name match with an inscope variable of the same type.
        username,           // "This uses the "Field Init" shorthand. Section 5.1
        active: true,
        sign_in_count: 1,
    }
}

// When copying - I feel like you want a const reference... not a borrow. But that doesn't work
// Because we need an owner of the shared string.
fn from_user(user1:User) -> User {
    println!("//------- From \"User\" -------");
    User {
        email: String::from("user2@example.com"),
        username: String::from("user2"),
        ..user1
    }
}
// Look at these fancy structs without any names in them...
#[derive(Debug)]   
struct Color(i32, i32, i32);
fn tuple_struct() {
    println!("\n\n\n//------- Tuple Struct -------");
    let black = Color(125, 135, 255);
    println!("{:#?}", black);
}

// I guess this might be useful? *shrug*
#[derive(Debug)]
struct AlwaysEqual;
fn unit_struct() {
    println!("\n\n\n//------- Always Equal -------");
    let subject = AlwaysEqual;
    println!("{:#?}", subject);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    // This is now a method of a rectangle instance (note it takes in a reference to itself)
    fn area(&self) -> u32 {
        self.height * self.width
    }

    // This is cool - it's like a constructor... Static method that returns an instance of Rectangle.
    // Note it doesn't take in &self.
    fn new(height:u32, width:u32) -> Self {
        Rectangle {
            height,
            width
        }
    }

    fn can_hold(&self, other:&Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn new_rectangle(height:u32, width:u32) -> Rectangle {
    Rectangle {
        width,
        height
    }
}

fn example_program() {
    println!("\n\n\n//------- Example Program -------");
    let rect = new_rectangle(20,30);
    let rect2 = Rectangle::new(25,35);

    println!("The area of the rectangle1 is {} square pixels", rect.area());
    println!("The area of the rectangle2 is {} square pixels", rect2.area());
    if rect2.can_hold(&rect) {
        println!("Rectangle 1 fits inside Rectange 2");
    } else {
        println!("Rectangle 1 does not fit inside Rectangle 2");
    }
}

fn main() {
    let new_user = create_user();
    println!("{:#?}", new_user);
    println!("User Email: {}", new_user.email);

    let new_user = build_user(String::from("test@example.com"), String::from("foo bar"));
    println!("{:#?}", new_user);

    let new_user2 = from_user(new_user);
    println!("{:#?}", new_user2);

    tuple_struct();
    unit_struct();
    example_program();
}
