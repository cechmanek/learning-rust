// examples of pattern matching such as 'if let', 'match' and 'while let'

fn main() {
  // example of mixing 'if let' and regular 'if' statements to do complex logic
  let favorite_color: Option<&str> = None;
  let is_tuesday = false;
  let age: Result<u8, _> = "34".parse();

  if let Some(color) = favorite_color {
    println!("Using your favorite color, {}, as the background", color);
  }
  else if is_tuesday {
    println!("Using green as it is Tuesday");
  }
  else if let Ok(age) = age { // shadowing Result age with u8 age
    if age > 30 {
      println!("Using purple as the background color");
    }
    else {
      println!("Using orange as the background color");
    }
  }
  else {
    println!("Using blue as the background color");
  }

  // example of 'while let' conditional loop
  let mut stack = Vec::new();
  stack.push(1);
  stack.push(2);
  stack.push(3);

  while let Some(top) = stack.pop() { // stack.pop() returns an Option so we need to match
    println!("{}", top);
  }

  // example of for loop matching
  let my_vec = vec!['a','b','c'];

  for (index, value) in my_vec.iter().enumerate() { // the (index, value) is the matched pattern
    println!("{} is at index {}", value, index);
  }

  // pattern matching is a core Rust concept. Even basic assignment is technically pattern matching
  // let PATTERN = EXPRESSION
  let x = 5; // the pattern 'x' matches whatever is on the right of the '=' sign

  // this concept exists in destructuring
  let (x,y,z) = (1,2,3);

  // pattern matching syntax
  let x = 10; // let x = 'a' won't work with below match block as types aren't identical
  
  match x { // match requires that x match with at least one branch
    1 => println!("x is one"),
    2 => println!("x is two"),
    3 => println!("x is three"), 
    4 | 5 => println!("x is either four or five"), // can have conditional matches
    6..=10 => println!("x between 6 and 10"), // can match to inclusive ranges. 6 and 10 match here
    _ => println!("x is not between 1 and 10"), // _ is a catch all pattern
  }

  // we need to be careful with shadowing in match blocks
  let x = Some(5);
  let y = 10;

  match x { 
    Some(50) => println!("Got 50"), // 50 doesn't match x. let 50=x fails
    Some(y) => println!("Matched, y = {:?}", y), // matches any Some(). local y variable shadows x
    _ => println!("default, x = {:?}", x), // this would also match, and print x=Some(5)
  }

  println!("at the end: x = {:?}, y = {:?}", x, y);

  // we can destructure and match values simultaneously
  struct Point {
    x: i32,
    y: i32,
  }

  let p = Point { x: 0, y: 7 };

  match p {
    Point { x, y: 0 } => println!("On the x axis at {}", x),
    Point { x: 0, y } => println!("On the y axis at {}", y),
    Point { x, y } => println!("On neither axis: ({}, {})", x, y)
  }

  // for larger structs we may want a short hand that matches on some vals, without listing all
  struct Point3 {
    x: i32,
    y: i32,
    z: i32,
    }

    let origin = Point3 { x: 0, y: 0, z: 0 };

    match origin {
      Point3 { x, .. } => println!("x is {}", x), // only care about x, .. matches all other params
    }

  // destructuring enums
  enum Message {
    Quit, // type of Message with no associated data
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
  }

  let msg = Message::ChangeColor(0, 160, 255);

  match msg {
    Message::Quit => {
      println!("The Quit variant has no data to destructure.")
    }
    Message::Move { x, y } => { // local x,y shadow Message::Move.x, y
      println!("Move in the x direction {} and in the y direction {}", x, y);
    }
    Message::Write(text) => println!("Text message: {}", text),
    Message::ChangeColor(r, g, b) => println!("Change color to r: {}, g: {}, and b: {}", r, g, b),
  }

  // we can add an extra conditional to a match block. this is called a match guard
  let num = Some(6);

  match num {
    Some(x) if x < 5 => println!("less than five: {}", x), // additional match guard x < 5
    Some(x) => println!("greater than five: {}", x),
    None => (),
  }


}
