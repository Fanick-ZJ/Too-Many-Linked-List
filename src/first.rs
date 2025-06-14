pub struct List {
    head: Link
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    /// origin link is : 1 -> 2 -> 3
    /// The element is 4
    /// new link is : 4 -> 1 -> 2 -> 3
    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem,
            next: std::mem::replace(&mut self.head, Link::Empty),
        });
        self.head = Link::More(new_node);
    }

    /// origin link is : 1 -> 2 -> 3
    /// After pop, the link is : 2 -> 3
    /// And return 1
    pub fn pop(&mut self) -> Option<i32> {
        match std::mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        } 
    }

    pub fn pop_node(&mut self) -> Link {
        match std::mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => Link::Empty,
            Link::More(mut node) => {
                let node_next = std::mem::replace(&mut node.next, Link::Empty);
                self.head = node_next;
                Link::More(node)
            },
        }

    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = std::mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut boxed_node) = cur_link {
            println!("Dropping {}", boxed_node.elem);
            cur_link = std::mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}

enum Link {
    Empty,
    More(Box<Node>)
}

struct Node {
    elem: i32,
    next: Link
}


#[cfg(test)]
mod test {
    use crate::first::List;

    #[test]
    fn basics() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);

        list.push(4);
        list.push(5);
        list.push(6);
        assert_eq!(list.pop(), Some(6));
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        assert_eq!(list.pop(), None);
    }

    #[test]
    fn drop() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        list.push(4);
        list.push(5);
        list.push(6);
    }

    #[test]
    fn pop_node() {
       let mut list = List::new();
       list.push(1);
       list.push(2);
       list.push(3); 
       list.push(4);
       let cur_link = list.pop_node();
        match cur_link {
            super::Link::Empty => assert!(false, "Link should not be empty"),
            super::Link::More(node) => {
                assert!(node.elem == 4, "Node elem should be 4");
                let next_link = node.next;
                match next_link {
                    crate::first::Link::Empty => assert!(true, ""),
                    crate::first::Link::More(_) => {
                        assert!(false, "Next link should be empty");
                    },
                }
            },
        }
    }
}