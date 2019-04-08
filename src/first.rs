use std::mem;

/// A bog-standard, naively-implemented singly linked list.
pub struct List {
    head: Link,
}

/// A link between nodes in a `List`. If `Empty`, this list node is just empty. If
/// `More(Box<Node>)`, then the `Node` inside the `Box` is the current node, and contains data (and
/// potentially a `Link` to the next node.)
enum Link {
    Empty,
    More(Box<Node>),
}

/// A node in the `List`. Contains data in `elem` and a `Link` in `next`, which could either be
/// `Empty` or the next node (`More(Box<Node>)`).
struct Node {
    elem: i32,
    next: Link,
}

impl List {
    /// Create a new singly linked list.
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    /// Push an item to the head of the list.
    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });

        self.head = Link::More(new_node);
    }

    /// Pop the value from the head of the list and return it.
    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

// Implementing `Drop` allows the List to be deallocated when it goes out of scope.
impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right.
        assert_eq!(list.pop(), None);

        // Populate list.
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal.
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
