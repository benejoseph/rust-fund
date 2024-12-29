
pub struct List {
  head: Link
}

type Link = Option<Box<Node>>;

struct Node {
  elem: i32,
  next: Link,
}

impl List {
  pub fn new() -> Self  {
    List{head: None }
  }

  pub fn push(&mut self, elem: i32) {
    let new_node = Node {
      elem: elem,
      next: self.head.take(),
    };
    self.head = Some(Box::new(new_node));
  }

  // pops the head
  pub fn pop(&mut self) -> Option<i32> {
    // if empty do nothing
    // otherwise return the head reference and set head to the next
    self.head.take().map(|node| {
      self.head = node.next;
      node.elem
    })
  }

}

mod test {
    use crate::second::List;
    #[test]
    fn basics() {
      let mut list = List::new();
      assert_eq!(list.pop(), None);

      list.push(1);
      list.push(2);
      list.push(3);

      assert_eq!(list.pop(), Some(3));
      assert_eq!(list.pop(), Some(2));

      list.push(4);
      list.push(5);

      assert_eq!(list.pop(), Some(5));
      assert_eq!(list.pop(), Some(4));

      // Check exhaustion
      assert_eq!(list.pop(), Some(1));
      assert_eq!(list.pop(), None);
    }
}
