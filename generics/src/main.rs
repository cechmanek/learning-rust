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
    println!("my float_point.x() is: {}", float_point.distance_from_origin());
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