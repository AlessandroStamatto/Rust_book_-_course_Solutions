use std::cmp::Ordering;

type BoxNode<T> = Box<Node<T>>;
type TreeLink<T> = Option<BoxNode<T>>;


pub struct Node<T> {
	elem: T,
	left: TreeLink<T>,
	right: TreeLink<T>,
}

impl<T: Ord> Node<T> {
	fn new(elem: T) -> BoxNode<T> {
		Box::new(Node {elem: elem, left: None, right: None})
	}

	fn insert(&mut self, elem: T) -> bool {
		match self.elem.cmp(&elem) {
			Ordering::Equal => false,
			Ordering::Greater => {
				if let Some(ref mut node) = self.left {
					node.insert(elem)
				} else {
					self.left = Some(Node::new(elem)); true
				}	
			}
			Ordering::Less => {
				if let Some(ref mut node) = self.right {
					node.insert(elem)
				} else {
					self.right = Some(Node::new(elem)); true
				}	
			}
		}
	}

	fn exists(&self, elem: T) -> bool {
		match self.elem.cmp(&elem) {
			Ordering::Equal => true,
			Ordering::Greater => self.left.as_ref().map_or(false, |node| node.exists(elem)),
			Ordering::Less => self.right.as_ref().map_or(false, |node| node.exists(elem)),
		}
	}
}

pub struct IntoIter<T> {
	next: TreeLink<T>,
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

impl<T> IntoIterator for BoxNode<T> {
	type Item = T;
	type IntoIter = IntoIter<T>;
	fn into_iter(self)->Self::IntoIter {
		IntoIter {next: Some(self)}
	}
}

#[cfg(test)]
mod test {
	use super::Node;

	#[test]
	fn basics() {
		let mut tree = Node::new(5);
		assert_eq!(tree.insert(5), false);
		assert_eq!(tree.insert(3), true);
		assert_eq!(tree.insert(2), true);
		assert_eq!(tree.insert(6), true);
		assert_eq!(tree.insert(7), true);
		assert_eq!(tree.insert(2), false);

		assert_eq!(tree.exists(6), true);
		assert_eq!(tree.exists(7), true);
		assert_eq!(tree.exists(5), true);
		assert_eq!(tree.exists(3), true);
		assert_eq!(tree.exists(2), true);
		assert_eq!(tree.exists(8), false);
		assert_eq!(tree.exists(10), false);
		assert_eq!(tree.exists(1), false);
		assert_eq!(tree.exists(0), false);
		assert_eq!(tree.exists(-1), false);
	}
}