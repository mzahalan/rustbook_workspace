mod article;
use article::{Tweet, NewsArticle, Summary};

fn largest<T: std::cmp::PartialOrd>(list :&[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_in_a_list() {
    println!("\n\n------- Largest In List -------");
    let number_list = vec![34, 36, 34, 65, 33, 30, 20, 200, 4];
    println!("largest number is {}", largest(&number_list));

    let char_list = vec! ['a', 'b', 'c', 'd', 'e', 'f'];
    println!("Largest char is {}", largest(&char_list));
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

#[derive(Debug)]
struct TwoPoint<T, U> {
    x: T,
    y: U
}

fn generic_point() {
    println!("\n\n------- Generic Point -------");
    let int_point = Point { x: 5, y: 10 };
    println!("Int Point {:#?}", int_point);
    let float_point = Point { x: 1.0, y: 2.5};
    println!("Float Point {:#?}", float_point);
    let weird_point = TwoPoint {x:5, y:4.3};
    println!("Weird Point {:#?}", weird_point);


    println!("\n\n------- Getter for Point -------");
    println!("Using X getter for point {:?} == {}",int_point, int_point.x() );
    println!("Distance From Origin for {:?} is {}", float_point, float_point.distance_from_origin());
}

fn wonky_mixup() {
    println!("\n\n------- Mix and Match -------");
    struct Point<X1, Y1> {
        x: X1,
        y: Y1,
    }
    impl<X1,Y1> Point<X1,Y1> {
        fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
            Point {
                x: self.x,
                y: other.y,
            }
        }
    }

    let p1 = Point {x:5, y:10.4};
    let p2 = Point {x:"Hello", y:'c'};
    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

}

fn using_article() {
    println!("\n\n------- Article -------");
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Headline"),
        location: String::from("Somewhere"),
        author: String::from("Some guy named Jett (with two Ts)"),
        content: String::from("wasn't worth reading anyway")
    };
    println!("1 new article: {}", article.summarize());
}


pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

fn using_notify() {
    println!("\n\n------- Notify -------");

    let item = returns_summarizable();
    notify(&item);
}

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "This is something new.",
        ),
        reply: false,
        retweet: false,
    }
}

fn trait_bound() {
    println!("\n\n ------- Traits for limited types ------- ");
    println!("In this section we have a method cmp_display only for Pairs");
    println!("in which the Type <T> implements the Display and PartialOrd traits");
    use std::fmt::Display;

    struct Pair<T> {
        x: T,
        y: T,
    }

    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }

    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }

    let my_pair = Pair {x:10, y:15};
    let new_pair = Pair::new(15, 25);
    my_pair.cmp_display();
    new_pair.cmp_display();
}

fn trait_on_trait() {
    // You can define an implementation for trait ToString on 
    // any Type that implements trait Display (for example)
    /*
    impl<T: Display> ToString for T {
        //
    }*/
    // This lets you call to_string() on anything that implements Display.

    let s = 3.to_string();
    println!("I just made a string from the number 3 -> {s}");
}

fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        return s1;
    } else {
        return s2;
    }
}

fn longest_string() {
    println!("\n\n------- Longest String -------\n"); 
    let s1 = String::from("abcd");          //String Type.
    let s2 = "xyz";                           // String slice

    let result = longest(s1.as_str(), s2);
    println!("The longest string is {}", result);
}

fn main() {
    largest_in_a_list();
    generic_point();
    wonky_mixup();
    using_article();
    using_notify();
    trait_bound();
    trait_on_trait();
    longest_string();
}
