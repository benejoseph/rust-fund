use std::mem;

pub struct List {
  head: Link
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
  pub fn new() -> Self  {
    List{head: Link::Empty }
  }

  pub fn push(&mut self, elem: i32) {
    let new_node = Node {
      elem: elem,
      next: mem::replace(&mut self.head, Link::Empty),
    };
    self.head = Link::More(Box::new(new_node));
  }

  // pops the head
  pub fn pop(&mut self) -> Option<i32> {
    // if empty do nothing
    // otherwise return the head reference and set head to the next
    match mem::replace(&mut self.head, Link::Empty) {
      Link::Empty => None,
      Link::More(node) => {
        self.head = node.next;
        Some(node.elem) 
      }
    }
  }
}

mod test {
    #[test]
    fn basics() {
        // TODO
    }
}
