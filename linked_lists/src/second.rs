pub struct List<T> {
  head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
  elem: T,
  next: Link<T>,
}

impl<T> List<T> {
  pub fn new() -> Self {
    return List { head: None };
  }

  pub fn push(&mut self, elem: T) {
    let new_node = Box::new(Node {
      elem: elem,
      next: self.head.take(),
    });

    self.head = Some(new_node);
  }

  pub fn pop(&mut self) -> Option<T> {
  
    return self.head.take().map(|node| {
      self.head = node.next;
      return node.elem;
    });
  }

  pub fn peek(&self) -> Option<&T> {
    return self.head.as_ref().map(|node| { return &node.elem;});
  }
  
  pub fn peek_mut(&mut self) -> Option<&mut T> {
    return self.head.as_mut().map(|node| { return &mut node.elem});
  }

  pub fn into_iter(self) -> IntoIter<T> {
    // rather than implement this method like an intelligent human would, the author defines a 
    // a new tuple struct, instantiates and returns this IntoIter struct, and has the Iterator
    // Method defined on IntoIter instead. Pure trash!
    return IntoIter(self);
  }

  pub fn iter<'a>(&'a self) -> Iter<'a,T> {
    return Iter {next :self.head.as_ref().map(|node|  &**node)};
    // &** is by far the ugliest thing I have ever seen in my entire software development life
  }

  pub fn iter_mut(&mut self) -> IterMut<'_, T> {
    return IterMut { next: self.head.as_mut().map(|node| &mut **node)};
  }
}

impl<T> Drop for List<T> {
  fn drop(&mut self) {
    let mut current_link = self.head.take();

    // "while let" == "do this thing until this pattern doesn't match"
    while let Some(mut boxed_node) = current_link {
      current_link = boxed_node.next.take();
      // boxed_node goes out of scope and gets dropped here;
      // but its Node's "next" field has been set to Link::Empty
      // so no unbounded recursion occurs
    }
  }
}

// IntoIter wrapper for list into_iter method 
pub struct IntoIter<T> (List<T>); // tuple struct, AKA disgraceful fucking garbage

impl<T> Iterator for IntoIter<T> {
  type Item = T;
  fn next(&mut self) -> Option<Self::Item> {
    return self.0.pop();
  }
}

// Iter wrapper for list iter method 
pub struct Iter<'a,T> { // yet another wrapper struct for our List
  next: Option<&'a Node<T>>,
}

impl<'a,T> Iterator for Iter<'a,T> {
  type Item = &'a T;

  fn next(&mut self) ->Option<Self::Item> {
    return self.next.map(|node| {
      self.next = node.next.as_ref().map(|node| &**node);
      // &** is by far the ugliest thing I have ever seen in my entire software development life
      return &node.elem;
  });
  }
}

// IterMut wrapper for list iter_mut method
pub struct IterMut<'a, T> {
  next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
  type Item = &'a mut T;

  fn next(&mut self) -> Option<Self::Item> {
    return self.next.take().map(|node| {
      self.next = node.next.as_mut().map(|node| &mut **node);
      return &mut node.elem;
    });
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

  #[test]
  fn peek() {
    let mut list = List::new();

    assert_eq!(list.peek(), None);
    assert_eq!(list.peek_mut(), None);

    list.push(1);
    list.push(2);
    list.push(3);

    assert_eq!(list.peek(), Some(&3));
    assert_eq!(list.peek_mut(), Some(&mut 3));

    list.peek_mut().map(|value| {
      *value = 42;
    });
  }

  #[test]
  fn into_iter() {
    let mut list = List::new();
    list.push(1);
    list.push(2);
    list.push(3);

    let mut iter = list.into_iter();
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), None);
  }

  #[test]
  fn iter() {
    let mut list = List::new();
    list.push(1); list.push(2); list.push(3);
      
    let mut iter = list.iter();
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&1));
  }

  #[test]
  fn iter_mut() {
    let mut list = List::new();
    list.push(1); list.push(2); list.push(3);

    let mut iter = list.iter_mut();
    assert_eq!(iter.next(), Some(&mut 3));
    assert_eq!(iter.next(), Some(&mut 2));
    assert_eq!(iter.next(), Some(&mut 1));
  }
}