use std::cmp::Ordering;

pub type Tree<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub struct Node<T> {
	elem: T,
	left: Tree<T>,
	right: Tree<T>,
}

fn new<T: Ord>()->Tree<T> {
	None
}

fn new_node<T: Ord>(elem: T) -> Box<Node<T>> {
	Box::new(Node {elem: elem, left: None, right: None})
}

trait InsertSearch<T> {
	fn insert(&mut self, elem: T) -> bool;
	fn search(&self, elem: T) -> bool;
	fn tree_into_iter(self) -> IntoIter<T>;
}

impl<T: Ord> InsertSearch<T> for Tree<T> {
	fn insert(&mut self, elem: T) -> bool {
		if let Some(ref mut node) = *self {
			match node.elem.cmp(&elem) {
				Ordering::Equal => false,
				Ordering::Greater => node.left.insert(elem),
				Ordering::Less => node.right.insert(elem) 
			}	
		} else {
			*self = Some(new_node(elem));
			true
		}  
	}

	fn search(&self, elem: T) -> bool {
		self.as_ref().map_or(false, |node| {
			match node.elem.cmp(&elem) {
				Ordering::Equal => true,
				Ordering::Greater => node.left.search(elem),
				Ordering::Less => node.right.search(elem)
			}
		})
	}

	fn tree_into_iter(mut self) -> IntoIter<T> {
		IntoIter {next: self.take()}
	}
}

pub struct IntoIter<T> {
	next: Tree<T>
}

impl<T> Iterator for IntoIter<T> {
	type Item = T;
	fn next(&mut self)->Option<Self::Item> {
		self.next.take().map(|mut node| {
			self.next = node.right.take();
			node.elem
		})
	}
}

#[cfg(test)]
mod test {
	use super::InsertSearch;

	#[test]
	fn basics() {
		let mut tree = super::new();
		assert_eq!(tree.search(8), false);
		assert_eq!(tree.insert(5), true);
		assert_eq!(tree.search(8), false);
		assert_eq!(tree.search(5), true);
		assert_eq!(tree.insert(5), false);
		assert_eq!(tree.search(5), true);
		assert_eq!(tree.insert(3), true);
		assert_eq!(tree.insert(2), true);
		assert_eq!(tree.search(3), true);
		assert_eq!(tree.insert(6), true);
		assert_eq!(tree.insert(7), true);
		assert_eq!(tree.insert(2), false);

		assert_eq!(tree.search(6), true);
		assert_eq!(tree.search(7), true);
		assert_eq!(tree.search(5), true);
		assert_eq!(tree.search(3), true);
		assert_eq!(tree.search(2), true);
		assert_eq!(tree.search(8), false);
		assert_eq!(tree.search(10), false);
		assert_eq!(tree.search(1), false);
		assert_eq!(tree.search(0), false);
		assert_eq!(tree.search(-1), false);

		//println!("{:#?}", tree);

		for el in tree.tree_into_iter() {
			println!("{:?}", el);
		}
	}
}