
use std::rc::Rc;
use std::cell::{Ref,RefCell};

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}


impl<T> Node<T> {
    fn new(val: T) -> Rc<RefCell<Self>> {
        Rc::new(
            RefCell::new(
                Node{
                    elem: val,
                    next: None,
                    prev: None
                }
            )
        )
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

impl<T> List<T> {
    fn new() -> Self {
        List {
            head: None,
            tail: None
        }
    }


    fn push_front(&mut self, val: T) {
        
        let new_node = Node::new(val);

        match self.head.take() {
            Some(node) => {
                node.borrow_mut().prev = Some(new_node.clone());
                new_node.borrow_mut().next = Some(node);
                self.head = Some(new_node);

            }
            None => {

                self.head = Some(new_node.clone());
                self.tail = Some(new_node);

            }
        }
    }

    fn push_back(&mut self, val: T) {
        
        let new_node: Rc<RefCell<Node<T>>> = Node::new(val);

        match self.tail.take() {
            Some(tail_node) => {
                tail_node.borrow_mut().next = Some(new_node.clone());
                new_node.borrow_mut().prev = Some(tail_node);
                self.tail = Some(new_node);
            }
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            }

        }
    
    }

    fn pop_front(&mut self) -> Option<T> {
        
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().prev.take();
                    self.head = Some(new_head);
                }
                None => { 
                    self.tail.take();
                }
            }
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem
        })
    }

    fn pop_back(&mut self) -> Option<T> {
        
        self.tail.take().map(|old_tail|{
            match old_tail.borrow_mut().prev.take() {
                Some(new_tail) => {
                    new_tail.borrow_mut().next.take();
                    self.tail = Some(new_tail);
                }
                None => {
                    self.head.take();
                }
            }
            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().elem
        })
        
    }

	fn peek_front(&self) -> Option<Ref<T>> {
		self.head.as_ref().map(|front_node| {
			Ref::map(front_node.borrow(), |node| &node.elem)
		})
	}

    fn peek_back(&self) -> Option<Ref<T>> {
        self.tail.as_ref().map(|tail_node| {
            Ref::map(tail_node.borrow(), |node| &node.elem)
        })
    }


}

#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn check_front_queue_operations() {

        let mut list : List<i32> = List::new();

        list.push_front(2);
        list.push_front(3);
        list.push_front(4);

        assert_eq!(Some(4),list.pop_front());
        assert_eq!(Some(3),list.pop_front());


        list.push_front(5);
        list.push_front(7);
        list.push_front(8);

        
        assert_eq!(Some(8),list.pop_front());
        assert_eq!(Some(7),list.pop_front());
        assert_eq!(Some(5),list.pop_front());


    }

	#[test]
	fn check_peek_front() {
		
		let mut list: List<i32> = List::new();
		
		assert!(list.peek_front().is_none());

		list.push_front(3);
		list.push_front(4);

		assert_eq!(&4,&*list.peek_front().unwrap());

	}

    #[test]
    fn check_peek_back() {
        let mut list: List<i32> = List::new();

        assert!(list.peek_back().is_none());
        list.push_front(3);
        list.push_front(4);

        assert_eq!(&3,&*list.peek_back().unwrap());
    }

	#[test]
	fn check_back_queue_operations(){
        
        let mut list: List<i32> = List::new();

        list.push_back(2);
        list.push_back(3);
        list.push_back(4);

        assert_eq!(Some(4),list.pop_back());
        assert_eq!(Some(3),list.pop_back());


        list.push_back(5);
        list.push_back(7);
        list.push_back(8);


        assert_eq!(Some(8),list.pop_back());
        assert_eq!(Some(7),list.pop_back());
        assert_eq!(Some(5),list.pop_back());


	}

    #[test]
    fn check_deque() {
        let mut list: List<i32> = List::new();

        list.push_back(2);
        list.push_back(3);
        list.push_back(4);
        list.push_back(5);

        assert_eq!(Some(2),list.pop_front());
        assert_eq!(Some(3),list.pop_front());
        assert_eq!(Some(4),list.pop_front());

        list.push_front(6);
        list.push_front(7);
        
        assert_eq!(Some(5),list.pop_back());
        assert_eq!(Some(6),list.pop_back());
        assert_eq!(Some(7),list.pop_back());

    }
}
