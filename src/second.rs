pub struct List<T> {
    head: Link<T>,
}

pub struct IntoIter<T> (List<T>);

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub struct Node<T> {
    elem: T,
    pub next: Link<T>
}

impl<T: std::fmt::Debug> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<'a, T: std::fmt::Debug> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.elem
        })
    }

}

impl<'a, T: std::fmt::Debug> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_mut().map(|node| &mut **node);
            &mut node.elem
        })
    }

}



impl<T> List<T> 
	where T : std::fmt::Debug
{
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, val: T) {
        let new_node = Box::new(Node {
            elem : val,
            next : self.head.take()
        });

        self.head = Some(new_node);
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node|{
            &mut node.elem
        })
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter (self)
    }

    pub fn iter(&self) -> Iter<T> {
        Iter { next: self.head.as_ref().map(|node| &**node) }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut { next: self.head.as_mut().map(|node| &mut **node) }
    }

    pub fn contains_cycle(&self) -> bool {
        if self.head.is_none() {
            return false;
        }

        let mut slow_ptr: Option<&Box<Node<T>>> = self.head.as_ref();
        let mut fast_ptr: Option<&Box<Node<T>>> = self.head.as_ref().unwrap().next.as_ref();

        while let Some(node_ref) = fast_ptr.unwrap().next.as_ref() {
            slow_ptr = slow_ptr.unwrap().next.as_ref();
            if fast_ptr.is_some() {
                fast_ptr = node_ref.next.as_ref();
            } else {
                break;
            }

            if slow_ptr.unwrap() as *const _  == fast_ptr.unwrap() as *const _ {
                return false;
            }
        }

        return false;
    }

	pub fn reverse(&mut self) {
		
		if self.head.is_none() || self.head.as_ref().unwrap().next.is_none() {
			return ();
		}

		let mut prev_node: Option<Box<Node<T>>> = self.head.take();
		let mut curr_node: Option<Box<Node<T>>> = prev_node.as_mut().unwrap().next.take();

		while let Some(next_node) = curr_node.as_mut().unwrap().next.take() {
			println!("curr = {:?} \n prev = {:?} \n next = {:?} ", curr_node, prev_node, next_node);
			let temp_next_node: Option<Box<Node<T>>> = Some(next_node);

			curr_node.as_mut().unwrap().next = prev_node;
			
			prev_node = curr_node;
			curr_node = temp_next_node; 

		}

		curr_node.as_mut().unwrap().next = prev_node;
		self.head = curr_node;
		
	}

}


impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut curr_node = self.head.take();
        while let Some(mut node) = curr_node {
            curr_node = node.next.take();
        }
    }
}

mod test {
    #[test]
    fn check_empty_list() {

        let mut list = super::List::<i32>::new();

        assert_eq!(list.pop(),None);

    }
   
    #[test]
    fn check_push_and_pop() {
        let mut list = super::List::<i32>::new();

        list.push(2);
        list.push(3);

        assert_eq!(Some(3),list.pop());
        assert_eq!(Some(2),list.pop());

    }   

    #[test]
    fn check_empty_peek() {
        let list = super::List::<i32>::new();

        assert_eq!(None,list.peek() );
    }


    #[test]
    fn check_peek() {
        let mut list = super::List::<i32>::new();

        list.push(3);
        list.push(4);

        assert_eq!(Some(&4),list.peek());
    }


    #[test]
    fn check_peek_mut() {
        let mut list = super::List::<i32>::new();

        list.push(3);
        list.push(4);

        assert_eq!(list.peek_mut(), Some(&mut 4));

        let peek_value: &mut i32 = list.peek_mut().unwrap();

        *peek_value = 42;

        assert_eq!(list.peek_mut(), Some(&mut 42));
    

    }

    #[test]
    fn check_into_iter() {
        
        let mut list = super::List::<i32>::new();

        list.push(3);
        list.push(4);

        let mut into_iter = list.into_iter();

        assert_eq!(into_iter.next(), Some(4));
        assert_eq!(into_iter.next(), Some(3));


    }

    #[test]
    fn check_iter() {
        let mut list = super::List::<i32>::new();

        list.push(4);
        list.push(5);

        let mut iter = list.iter();

        assert_eq!(iter.next(),Some(&5));
        assert_eq!(iter.next(),Some(&4));

    }

    #[test]
    fn check_iter_mut() {
        let mut list = super::List::<i32>::new();

        list.push(4);
        list.push(5);

        let mut iter = list.iter_mut();

        assert_eq!(iter.next(),Some(&mut 5));
        assert_eq!(iter.next(),Some(&mut 4));

    }

    #[test]
    fn check_contains_cycle() {
        let mut list = super::List::<i32>::new();

        list.push(5);
        list.push(6);
        
        list.push(633);
        list.push(236);
        list.push(60);
        list.push(26);
        list.push(65);
        list.push(56);

        assert_eq!(list.contains_cycle(),false);

    }

	#[test]
	fn check_reverse() {
		let mut vals: Vec<i32> = vec![2,3,4,5,6,7];

		let mut list = super::List::<i32>::new();
		list.reverse();

		assert_eq!(list.iter().next(),None);

		for val in vals.iter() {
			list.push(*val);
		}

		vals.reverse();

		let mut list_iter = list.iter();

		for val in vals.iter() {
			assert_eq!(val,list_iter.next().unwrap());
		}

	}	

}
