use std::{cell::RefCell, rc::Rc};


struct ListNode<T> {
    item: T,
    next: Link<T>,
    prev: Link<T>,
    token: Token,
}

impl<T> ListNode<T> {
    fn new(item: T) -> Self {
        Self {
            item,
            next: None,
            prev: None,
            token: Token::new(),
        }
    }
}

type Link<T> = Option<Rc<RefCell<ListNode<T>>>>;

#[derive(Default)]
pub struct LinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
    size: usize,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            size: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn push(&mut self, item: T) {
        let node = Rc::new(RefCell::new(ListNode::new(item)));
        if let Some(prev_tail) = self.tail.take() {
            prev_tail.borrow_mut().next = Some(Rc::clone(&node));
            node.borrow_mut().prev = Some(prev_tail);
            self.tail = Some(node);
            self.size += 1;
        } else {
            self.head = Some(Rc::clone(&node));
            self.tail = Some(node);
            self.size = 1;
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        self.tail.take().map(|prev_tail| {
            self.size -= 1;
            match prev_tail.borrow_mut().prev.take() {
                Some(node) => {
                    node.borrow_mut().next = None;
                    self.tail = Some(node);
                }
                None => {
                    self.head.take();
                }
            }
            Rc::try_unwrap(prev_tail).ok().unwrap().into_inner().item
        })
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while let Some(node) = self.head.take() {
            let _ = node.borrow_mut().prev.take();
            self.head = node.borrow_mut().next.take();
        }
        self.tail.take();
    }
}

struct Token {}

impl Token {
    fn new() -> Self {
        println!("CREATED");
        Self {}
    }
}

impl Drop for Token {
    fn drop(&mut self) {
        println!("DESTROYED");
    }
}

impl<T> IntoIterator for LinkedList<T> {
    type Item = <ListIterator<T> as Iterator>::Item;

    type IntoIter = ListIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self)
    }
}

pub struct ListIterator<T> {
    list: LinkedList<T>,
}

impl<T> ListIterator<T> {
    fn new(list: LinkedList<T>) -> Self {
        Self { list }
    }
}

impl<T> Iterator for ListIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn it_works() {
        let mut list: LinkedList<i8> = LinkedList::new();

        assert_eq!(list.pop(), None);
        assert_eq!(list.len(), 0);
        assert!(list.is_empty());
    }

    #[test]
    fn can_push() {
        let mut list = LinkedList::new();
        assert_eq!(list.pop(), None);

        list.push(3);
        list.push(4);
        list.push(5);
        assert_eq!(list.pop(), Some(5));

        list.push(6);
        list.push(7);
        assert_eq!(list.pop(), Some(7));
        assert_eq!(list.pop(), Some(6));
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(3));

        list.push(2);
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), None);
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn it_drops_correctly() {
        let mut list = LinkedList::new();
        for i in 0..3 {
            list.push(i);
        }

        drop(list);
    }

    #[test]
    fn can_iterate_forward() {
        let mut list = LinkedList::new();
        for i in 0..10 {
            list.push(i);
        }

        assert!(Iterator::eq(list.into_iter(), (0..10).rev()));
    }
}

fn main() {
    println!("Hello, world!");
}