use std::mem;

pub struct List {
  head: Link,
}

enum Link {
  Empty,
  More(Box<Node>),
}

struct Node {
  elem: i32,
  next: Link,
}

impl List {
  pub fn new() -> Self {
    return List { head: Link::Empty };
  }

  pub fn push(&mut self, elem: i32) {
    let new_node = Box::new(Node {
      elem: elem,
      next: mem::replace(&mut self.head, Link::Empty),
    });

    self.head = Link::More(new_node);
  }

  pub fn pop(&mut self) -> Option<i32> {
   
   return match mem::replace(&mut self.head, Link::Empty) {
      Link::Empty => { None } 
      Link::More(node) => { self.head = node.next;
                            Some(node.elem) }
    };
  }

}

impl Drop for List {
  fn drop(&mut self) {
    let mut current_link = mem::replace(&mut self.head, Link::Empty);

    // "while let" == "do this thing until this pattern doesn't match"
    while let Link::More(mut boxed_node) = current_link {
      current_link = mem::replace(&mut boxed_node.next, Link::Empty);
      // boxed_node goes out of scope and gets dropped here;
      // but its Node's "next" field has been set to Link::Empty
      // so no unbounded recursion occurs
    }
  }
}

#[cfg(test)] // only build and run this module when in test mode
mod test {
  use super::List;

    #[test]
    fn basics ()  {
      let mut list = List::new();

      // check empty list behaves right
      assert_eq!(list.pop(), None);

      // populate list
      list.push(1);
      list.push(2);
      list.push(3);

      //check normal removal
      assert_eq!(list.pop(), Some(3));
      assert_eq!(list.pop(), Some(2));

      // push some more to make sure nothing is broken
      list.push(4);
      list.push(5);

      // check normal removal
      assert_eq!(list.pop(), Some(5));
      assert_eq!(list.pop(), Some(4));

      // check exhaustion
      assert_eq!(list.pop(), Some(1));
      assert_eq!(list.pop(), None);
    }
}