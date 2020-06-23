// example of implementing traits to use polymorphism.
// first, let's pretend we're building a GUI with buttons and drawing tools on a screen
pub trait Draw {
  fn draw(&self);
}

pub struct Screen {
  pub components: Vec<Box<dyn Draw>>, // can hold any struct that implements Draw trait
}

impl Screen {
  pub fn run(&self) {
    for component in self.components.iter() {
      component.draw(); // compiler knows it's safe because it knows component implements Draw trait
    }
  }
}

// create a struct that implements Draw trait
pub struct Button {
  pub width: u32,
  pub height: u32,
  pub label: String,
}

impl Draw for Button {
  fn draw(&self) {
    // add some code here
  }
}

// create another struct that also implements Draw trait
pub struct SelectBox {
  width: u32,
  height: u32,
  options: Vec<String>,
}

impl Draw for SelectBox {
  fn draw(&self) {
    // some code here
  }
}

// second, let's pretend we are creating blog posts that must be reviewed before publishing
use object_oriented::Post; // our local crate with Post defined in src/lib.rs

fn main() {

  // we can create a Screen instance that holds Buttons and/or SelectBoxes in its components vec
  let my_screen = Screen {
                          components: vec![
                                          Box::new(Button {
                                            width: 3,
                                            height: 3,
                                            label: String::from("button label"),
                                          }),
                                          Box::new(SelectBox {
                                            width: 25,
                                            height:50,
                                            options: vec![String::from("opt1"), String::from("opt2")]
                                          })
                                          ]
                         };


}
