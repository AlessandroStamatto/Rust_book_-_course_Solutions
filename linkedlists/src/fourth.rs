use std::rc::Rc;
use std::cell::RefCell;

pub struct List<T> {
	head: Link<T>,
	tail: Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
	elem: T,

	prev: Link<T>,
	next: Link<T>,
}

impl<T> Node<T> {
	fn new(elem: T) -> Rc<RefCell<Self>> {
		Rc::new(RefCell::new({
			Node {elem: elem, prev: None, next:None}
		}))
	}
}

impl<T> List<T> {
	pub fn new() -> Self {
		List {head: None, tail: None}
	}

	pub fn push_front(&mut self, elem: T) -> &mut Self {
		let new_head = Node::new(elem);
		match self.head.take() {
			Some(old_head) => {
				old_head.borrow_mut().prev = Some(new_head.clone());
				new_head.borrow_mut().next = Some(old_head.clone());
				self.head = Some(new_head);
			}
			None => {
				self.tail = Some(new_head.clone());
				self.head = Some(new_head);
			}
		}
		self
	}

	pub fn pop_front(&mut self) -> Option<T> {
		self.head.take().map(|old_head| { // -1 old
			match old_head.borrow_mut().next.take() { //-1 new
				Some(new_head) => { 
					new_head.borrow_mut().prev.take(); //-1 old
					self.head = Some(new_head); //+1 new
				}
				None => {
					self.tail.take(); //-1 old
				}
			}
			Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem	
		}) 
	}

	pub fn pop_discard(&mut self) -> &mut Self {
		self.head.take().map(|old_head| { // -1 old
			match old_head.borrow_mut().next.take() { //-1 new
				Some(new_head) => { 
					new_head.borrow_mut().prev.take(); //-1 old
					self.head = Some(new_head); //+1 new
				}
				None => {
					self.tail.take(); //-1 old
				}
			}
			Rc::try_unwrap(old_head).ok().unwrap();
		});
		self
	}

	// pub fn peek_front(&self) -> Option<Ref<T>> {
	// 	self.head.as_ref().map(|node| {
	// 		Ref::map(node.borrow(), |node| &node.elem)
	// 	})
	// }
}

impl<T> Drop for List<T> {
	fn drop(&mut self) {
		while self.pop_front().is_some() {}
	}
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        assert_eq!(list.pop_front(), None);

        list.push_front(1).push_front(2).push_front(3);

        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));

        list.push_front(4).push_front(5);

        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), Some(4));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.pop_front(), None);

        list.push_front(1).push_front(2).push_front(3);
        list.pop_discard().pop_discard();
        assert_eq!(list.pop_front(), Some(1));

        list.pop_discard().pop_discard();
        assert_eq!(list.pop_front(), None);
    }
}