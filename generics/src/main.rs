// using generics is a conceptually similar abstraction to using functions for code reuse 

fn main() {

    // ex. to find the largest number in an array we can build this loop
    let number_list = vec![34, 50, 25, 100, 65];
    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    println!("The largest number is {}", largest);

    // but this only works for the array named 'number_list'. We want this as an abstract function
    let number_list = vec![24, 47, 35, 345, 365];
    println!("The largest number is {}", largest_i32(&number_list));

    // now lets do the same thing, but for chars
    let char_list = vec!['a', 'e', 'i', 'o', 'u', 'y'];
    println!("The largest char is {}", largest_char(&char_list));

    // generics work for structs too
    let integer_point = Point{x:3, y:5};
    let float_point = Point{x:1.1, y:-2.3};
    let char_point = Point{x:'c', y:'F'}; // this works, but probably isn't what was intended
    
    // x and y are different types here, the struct definition says they bost must be the same <T>
    //let mixed_point = Point{x:3, y:3.0}; // this won't compile yet

    // with multiple generic types in the struct definition
    let mixed_point = Point2{x:3, y:3.0};

    // let's call some methods we implemented on the Point struct via generics
    println!("my integer_point.x() is: {}", integer_point.x());
    println!("my float_point.x() is: {}", float_point.x());
    
    // try calling point.distance_from_origin() on our structs. it won't compile for non f32 types
    //println!("my float_point.x() is: {}", integer_point.distance_from_origin()); won't work on i32
    println!("my float_point.distance_from_origin() is: {}", float_point.distance_from_origin());
}



// this function abstracts the hardcoded loop in main()
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &number in list{
        if number > largest {
            largest = number;
        }
    }
    return largest;
}

// same function, but for different data type
fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];
    for &character in list {
        if character > largest {
            largest = character;
        }
    }
    return largest;
}

// rather than have a dozen functions, one for each data type, lets use a generic data type T
// note: this won't compile yet as we haven't guaranteed that all types we pass it will be orderable
/*
fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    return largest;
}
*/

// similarly to function definitions with generic types, structs can also have generic types
struct Point<T> {
    x: T,
    y: T,
}

// we can have multiple different generic types in a struct or function
struct Point2<T, U> {
    x: T,
    y: U,
}

// struct methods can be implemented with or without generics
// here's a fully generic getter method for any Point struct
impl<T> Point<T> {
    fn x(&self) ->&T {
        return &self.x;
    }
}

// this will only be implemented for Points that have float32 data type
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        return ( self.x.powi(2) + self.y.powi(2) ).sqrt();
    }
}

// where to put which generic definition depends on where it is needed
// read this carefully to see where each type T,U,V,W is used
impl <T,U> Point2<T,U> {
    fn swap_stuff<V,W>(self, other: Point2<V,W>) -> Point2<T,W> {
        let new_point = Point2 {x:self.x, y:other.y};
        return new_point;
    }
}

// TRAITS
// traits are similar to abstract classes in c++, or interfaces in c#. They specify a set of methods
// that a struct with the corresponding trait must implement

// here's an example that text-filled structs, like articles and tweets may have

pub trait Summary {
    fn summarize(&self) -> String; // here we just declare the method signature, not implementation

    fn get_author(&self) -> String { // here we define a default method, which can be over written
        return String::from("justin cechmanek");
    }
}
// this states that any struct with the Summary trait must implement and define the summarize method

// now apply it to some structs
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        return format!("{}, by {} ({})", self.headline, self.author, self.location);
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        return format!("{}: {}", self.username, self.content);
    }
}

/// Combining traits with functions
// we saw how to use generics to get better code reuse with functions, 
// now we can do this with traits too 
fn notify(item: impl Summary) { // now this function accepts any type that has the Summary trait
    println!("Breaking news! {}", item.summarize()); 
}

// the above notify(..) function is actually syntactic sugar for what's call a 'trait bound'
// this states that generic types that implement Summary trait are accepted
fn full_notify<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}

use std::fmt::Display; // import some random traits
use std::fmt::Debug;

// when there are many trait bounds things can get hard to read, so Rust has a 'where' keyword
fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
    return 1;
}
// this is equivalent to:
fn some_other_function<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
    return 1;
}

// redoing our generic largest<T>(list :&[T]) -> T function
// now by specifying the needed traits on the input type T this will compile
fn largest<T : PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    return largest;
}

