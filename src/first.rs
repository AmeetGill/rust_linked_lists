use std::mem;

pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node{
    elem: i32,
    next: Link
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, val: i32) {
        let new_node = Box::new(Node {
            elem : val,
            next : mem::replace(&mut self.head, Link::Empty)
        });

        self.head = Link::More(new_node);
    }

    pub fn drop(&mut self) {
        let mut curr_node = mem::replace(&mut self.head,Link::Empty);

        while let Link::More(mut node) = curr_node {
            curr_node = mem::replace(&mut node.next, Link::Empty);
        }

    }

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

mod test {
    #[test]
    fn check_empty_list() {

        let mut list = super::List::new();

        assert_eq!(list.pop(),None);

    }
   
    #[test]
    fn check_push_and_pop() {
        let mut list = super::List::new();

        list.push(2);
        list.push(3);

        assert_eq!(Some(3),list.pop());
        assert_eq!(Some(2),list.pop());

    }    
}
