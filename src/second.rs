pub struct List<T> {
    head: Link<T>
}

type Link<T> = Option<Box<Node<T>>>;

pub struct IntoIter<T>(List<T>);

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    /// origin link is : 1 -> 2 -> 3
    /// The element is 4
    /// new link is : 4 -> 1 -> 2 -> 3
    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    /// origin link is : 1 -> 2 -> 3
    /// After pop, the link is : 2 -> 3
    /// And return 1
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn pop_node(&mut self) -> Link<T> {
        match self.head.take() {
            None => None,
            Some(mut node) => {
                let node_next = node.next.take();
                self.head = node_next;
                Some(node)
            },
        }

    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.elem
        }) 
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.elem
        }) 
    }
}

impl<T> List<T> {

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<T> {
        Iter { next: self.head.as_deref()}
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut { next: self.head.as_deref_mut() }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        /* 
            self.next 的类型为 Option<&'a mut Node<T>>，我们不能在可变引用中中移动数据
            那为什么在Iter的next方法中就可以使用呢，因为Iter的next返回的对象不要求可修改
            其返回类型为 Option<&'a Node<T>>，由于`&`是不可变引用，是实现了Copy的，
            所以在map的回调中传入的指为原始指针的拷贝，不涉及移动操作，而&mut 是可变引用，
            在map的回调中传入的指针是原始指针的可变引用，涉及到移动操作，所以需要使用take方法
         */
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = std::mem::replace(&mut boxed_node.next, None);
        }
    }
}

struct Node<T> {
    elem: T,
    next: Link<T>
}


#[cfg(test)]
mod test {
    use crate::second::List;

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
    fn _drop() {
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
            None => assert!(false, "Link should not be empty"),
            Some(node) => {
                assert!(node.elem == 4, "Node elem should be 4");
                let next_link = node.next;
                match next_link {
                    None => assert!(true, ""),
                    Some(_) => {
                        assert!(false, "Next link should be empty");
                    },
                }
            },
        }
    }

    #[test]
    fn peek() {
       let mut list = List::new();
       assert_eq!(list.peek(), None);
       assert_eq!(list.peek_mut(), None);
       list.push(1);
       list.push(2);
       list.push(3);
       list.push(4);
       assert_eq!(list.peek(), Some(&4));
       assert_eq!(list.peek_mut(), Some(&mut 4));

       list.peek_mut().map(|value| {
           *value = 5;
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
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter();

        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter_mut();

        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));

        // Modify the elements
        if let Some(value) = iter.next() {
            *value += 10;
        }

        // Check the modified value
        list.pop();
        list.pop();
        assert_eq!(list.pop(), Some(11));
    }
}