use std::cmp::Ordering;

#[derive(Debug)]
pub struct Tree<T> (Option<Box<Node<T>>>);

#[derive(Debug)]
pub struct Node<T> {
	elem: T,
	left: Tree<T>,
	right: Tree<T>,
}

impl<T: Ord> Node<T> {
	fn new(elem: T) -> Box<Node<T>> {
		Box::new(Node {elem: elem, left: Tree(None), right: Tree(None)})
	}
}

impl<T: Ord> Tree<T> {
	fn new()->Tree<T> {
		Tree(None)
	}

	fn insert(&mut self, elem: T) -> bool {
		if let Some(ref mut node) = self.0 {
			match node.elem.cmp(&elem) {
				Ordering::Equal => false,
				Ordering::Greater => node.left.insert(elem),
				Ordering::Less => node.right.insert(elem) 
			}	
		} else {
			self.0 = Some(Node::new(elem));
			true
		}  
	}

	fn search(&self, elem: T) -> bool {
		self.0.as_ref().map_or(false, |node| {
			match node.elem.cmp(&elem) {
				Ordering::Equal => true,
				Ordering::Greater => node.left.search(elem),
				Ordering::Less => node.right.search(elem)
			}
		})
	}
}

//====== MOVE ITERATOR IMPLEMENTATION =======
//MOVE ITERATOR STRUCT
pub struct IntoIter<T> {
	next: Tree<T>
}

//MOVE ITERATOR NEXT
impl<T> Iterator for IntoIter<T> {
	type Item = T;
	fn next(&mut self)->Option<Self::Item> {
		self.next.0.take().map(|mut node| {
			self.next = Tree(node.right.0.take());
			node.elem
		})
	}
}

//Tree<T> to MOVE ITERATOR 
impl<T> IntoIterator for Tree<T> {
	type Item = T;
	type IntoIter = IntoIter<T>;
	fn into_iter(mut self)->Self::IntoIter {
		IntoIter {next: Tree(self.0.take())}
	}
}
/////////////////////////////////////////////////

//====== BORROW ITERATOR IMPLEMENTATION =======

//BORROW ITERATOR STRUCT
pub struct Iter<'a, T: 'a> {
	next: Option<&'a Node<T>>
}

//BORROW ITERATOR NEXT
impl<'a, T> Iterator for Iter<'a, T> {
	type Item = &'a T;
	fn next(&mut self)->Option<Self::Item> {
		self.next.take().map(|node| {
			self.next = node.right.0.as_ref().map(|node| &**node);
			&node.elem
		})
	}
}

//&Tree<T> to BORROW ITERATOR
impl<'a, T> IntoIterator for &'a Tree<T> {
	type Item = &'a T;
	type IntoIter = Iter<'a, T>;
	fn into_iter(self)->Self::IntoIter {
		Iter {next: self.0.as_ref().map(|node| &**node)}
	}
}
/////////////////////////////////////////////////

//====== MUTABLE BORROW ITERATOR IMPLEMENTATION =======

//MUTABLE BORROW STRUCT
pub struct IterMut<'a, T: 'a> {
	next: Option<&'a mut Node<T>>
}

//MUTABLE BORROW NEXT
impl<'a, T> Iterator for IterMut<'a, T> {
	type Item = &'a mut T;
	fn next(&mut self)->Option<Self::Item> {
		self.next.take().map(|node| {
			self.next = node.right.0.as_mut().map(|node| &mut **node);
			&mut node.elem
		})
	}
}

//&mut Tree<T> to MUTABLE BORROW ITERATOR
impl<'a, T> IntoIterator for &'a mut Tree<T> {
	type Item = &'a mut T;
	type IntoIter = IterMut<'a, T>;
	fn into_iter(self)->Self::IntoIter {
		IterMut {next: self.0.as_mut().map(|node| &mut **node)}
	}
}
/////////////////////////////////////////////////

#[cfg(test)]
mod test {
	use super::Tree;

	#[test]
	fn basics() {
		let mut tree = Tree::new();

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

		//The loops bellow test all 3 iterators
		for elt in &tree {
			println!("{:#?}", elt);
		}

		//this does not make logical sense, just
		//playing with mut ref iterators
		for elt in &mut tree {
			*elt += 1;
		}

		//consumes the tree
		for elt in tree {
			println!("{:#?}", elt);
		}
	}
}