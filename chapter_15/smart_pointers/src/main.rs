use crate::List::{Cons,Nil};
use std::rc::Rc;
use std::ops::Deref;

fn basic_box() {
    println!("\n\n------- Basic Box -------");
    let b = Box::new(5);
    println!("b = {}", b);
}

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn cons_list() {
    println!("\n\n------- Basic Box -------");

    // Recursion... 
    // The List enum has an element that points to another list item.
    // That means we can't know the size at compile time.
    let my_list = Box::new(Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil)))))));
    println!("my_list: {:?}", my_list);
}

#[derive(Debug)]
struct MyBox<T>(T);

// We're implementing the Deref trait
// this allows us to do something like *my_trait to get to
// the value.
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

fn new_box() {
    println!("\n\n-------  My Box -------");
    let x = MyBox::new(5);
    let y = MyBox::new(String::from("test"));
    let z = &y;

    println!("x: {:?}, y: {:?}", x,y);
    println!("Reference to y: {:?}", *z);
    println!("Dereferencing y: {:?}", *y);
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn deref_coercion() {
    println!("\n\n-------  deref_coercion -------");
    
    // Passing a &str to hello.
    hello("World");

    // Passing String to &str... this uses deref coersion 
    // String defines a "deref" operation to &str.
    let name = String::from("World");
    hello(&name);

    // Passing a reference to MyBox also works because
    // We defined a Deref to String.
    let my_box = MyBox::new(String::from("World"));
    hello(&my_box);

}

#[derive(Debug)]
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}


fn drop_example() {
    println!("\n\n-------- Drop Trait -------");
    let c = CustomSmartPointer{data:String::from("A")};
    let d = CustomSmartPointer{data:String::from("B")};
    println!("CustomSmartPointers Created... {:?} and {:?}", c, d);
    
    // Explicitely calling std::mem::drop() to free a resource
    // this will cause the destructor to run.
    drop(c);
    println!("Going out of scope...");
}

#[derive(Debug)]
enum RcList {
    Cons(i32, Rc<RcList>),
    Nil,
}

fn rc_pointer() {
    println!("\n\n-------- Reference Counter -------");
    
    let a = Rc::new(RcList::Cons(5, Rc::new(RcList::Cons(10, Rc::new(RcList::Nil)))));
    println!("a list: {:?}", a);
    println!("count after creating a = {}", Rc::strong_count(&a));

    let b = RcList::Cons(3, Rc::clone(&a));
    println!("b list: {:?}", b);
    println!("count after creating b = {}", Rc::strong_count(&a));

    {
        let c = RcList::Cons(4, Rc::clone(&a));
        println!("c list: {:?}", c);
        println!("count after creating c = {}", Rc::strong_count(&a));
    }

    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

fn main() {
    println!("chapter 15");
    basic_box();
    cons_list();
    new_box();
    deref_coercion();
    drop_example();
    rc_pointer();
}
