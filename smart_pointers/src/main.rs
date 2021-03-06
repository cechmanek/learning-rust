
// we need pointers so that we can define data with unknown sizes at compile time,
// such as a linked list
enum LinkedList {
    Node(i32, Box<LinkedList>), // node with int value and smart pointer to next LinkedList Node 
    Nil, // terminal node
}

use crate::LinkedList::{Node, Nil}; // shorthand so we can say Node(), instead of LinkedList::Node

// we will define our own version of the Box<> smart pointer
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        return MyBox(x);
    }
}

// implement the dereferencing trait (so we can use the * deref operator)
use std::ops::Deref;
impl<T> Deref for MyBox<T> {
    type Target = T; // slightly different way to do template type. name must be 'Target'
    fn deref(&self) -> &T {
        return &self.0; // return a reference to the struct's inner data
    }
}

// since pointers don't hold data themselves there needs to be clean up when they go out of scope
// we can specify custom clean up steps when a pointer drops out of scope
// this is comparable to c++ class destructors, and is done via the Drop trait

struct CustomSmartPointer {
  data: String,
}

impl Drop for CustomSmartPointer {
  fn drop(&mut self) {
      println!("Dropping this custom smart pointer with data: {}", self.data);
  }
}
// if we didn't write custom drop() method the struct would still be cleaned up, just silently


// above are the simplest types of smart pointers. We can also use reference counted pointers, Rc<>
use std::rc::Rc;
enum RcList {
    RcNode(i32, Rc<RcList>),
    RcNil, // RcNil instead of Nil, because Nil belongs to List enum above
}
use crate::RcList::{RcNode, RcNil};


fn main() {
  let b = Box::new(5); // smart pointer that is allocated on the heap
  println!("b = {}", b);

  let my_list = Node(1, Box::new(Node(2, Box::new(Node(3, Box::new(Nil))))));

  // with the Deref trait implemented, and the deref() method defined we can use the deref operator
  let x = 5;
  let my_box = MyBox::new(x);

  assert_eq!(x, 5);
  //assert_eq!(my_box, 5); // won't work as my_box is struct, not a number 
  assert_eq!(*my_box, 5); // *my_box === *(my_box.deref()) which is a normal dereference operation

  // see what happens when we create and destroy a CustomSmartPointer with the Drop trait
  let custom = CustomSmartPointer {data: String::from("message that is printed when i drop.")};
  // when main() ends custom goes out of scope and custom.drop() is called

  // we can't call custom.drop() manually as Rust does it implicitly always
  // this would be a double-free error if we did it too
  
  // we can call the std::mem::drop() function on a variable to clean it early
  drop(custom); // this works just fine
  println!("this will print after 'custom' is deleted");

  // with reference counted smart pointers we can create graphs with nodes pointing to each other
  let a = Rc::new(RcNode(5, Rc::new(RcNode(10, Rc::new(RcNil)))));
  let b = RcNode(3, Rc::clone(&a)); // clone() creates a new ref counted pointer, doesn't copy data
  let c = RcNode(4, Rc::clone(&a));
  /* graph looks like:
   b[3] -> a[5] -> [10] -> [RcNil]
   c[4] ---^
  */
  
  // to see how many ref counted pointers exist on an item call:
  let num_references = Rc::strong_count(&a);
  println!("there are {} references on the data pointed to by 'a'",num_references);
  // the references to &a are : [a, b, c]. Don't forget 'a' points to '&a' so is a reference

}