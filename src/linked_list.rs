use core::ptr::NonNull;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::iter::{empty, Iterator};

#[macro_export]
macro_rules! new_node {
	( $val:expr ) => {{
		Box::leak(Box::new(Node::new($val))).into()
	}};
}
#[macro_export]
macro_rules! set_next_of {
	( $left:expr,$right:expr ) => {{
		unsafe { (*$left.as_ptr()).next = $right }
	}};
}
#[macro_export]
macro_rules! next_of {
	( $ptr:expr ) => {{
		unsafe { (*$ptr.as_ptr()).next }
	}};
}
#[macro_export]
macro_rules! val_of {
	( $ptr:expr ) => {{
		unsafe { (*$ptr.as_ptr()).val }
	}};
}
// An element in the cool-lex LinkedList algorithm
struct Node {
	next: NonNull<Node>,
	val: bool, // whether this element is selected for the combination
}
impl Node {
	fn new(val: bool) -> Self {
		Node {
			next: NonNull::dangling(),
			val,
		}
	}
}

pub struct Algorithm {
	// var names -> as found in the paper, for better understanding of the code here
	b: NonNull<Node>, // the head of the list; this is the node with the greatest "index"
	x: NonNull<Node>, // the first node, right-to-left, whose value is 1 and whose predecessor's value is 0

	dangling: NonNull<Node>, // "next" of a node that does not have a next node
}
impl Algorithm {
	/// Param <tt>s</tt>: the number of <tt>0</tt>-bits.
	///
	/// Param <tt>t</tt>: the number of <tt>1</tt>-bits. Must be <tt>0</tt>.
	pub fn new(s: usize, t: usize) -> Self {
		let b: NonNull<Node> = new_node!(true);
		let mut x = b;
		for _ in 1..t {
			x = new_node_next_to(x, true);
		}
		let mut last = x;
		for _ in 0..s {
			last = new_node_next_to(last, false);
		}
		Algorithm {
			b,
			x,
			dangling: next_of!(last),
		}
	}
	#[allow(unused_unsafe)]
	pub fn next_combination(&mut self) {
		let y = next_of!(self.x);
		set_next_of!(self.x, next_of!(y));
		set_next_of!(y, self.b);
		self.b = y;

		if !val_of!(self.b) && val_of!(next_of!(self.b)) {
			self.x = next_of!(self.b);
		}
	}
	pub fn has_more(&self) -> bool {
		next_of!(self.x) != self.dangling
	}
}
impl Display for Algorithm {
	fn fmt(&self, f: &mut Formatter) -> FmtResult {
		let mut curr = self.b;
		loop {
			let val_to_display = if val_of!(curr) { '1' } else { '0' };

			//fail fast
			if let Result::Err(e) = write!(f, "{}", val_to_display) {
				break Result::Err(e);
			}

			curr = next_of!(curr);
			if self.dangling == curr {
				break Result::Ok(());
			}
		}
	}
}

fn new_node_next_to(curr: NonNull<Node>, val: bool) -> NonNull<Node> {
	let next = new_node!(val);
	set_next_of!(curr, next);
	next
}

/// Iterator over the selected indices.
pub struct SelInd {
	alg: *const Algorithm,
	curr: NonNull<Node>,
	i: usize,
}
impl SelInd {
	fn new(alg: *const Algorithm) -> Self {
		SelInd {
			alg,
			curr: unsafe { (*alg).b },
			i: 0,
		}
	}
}
impl Iterator for SelInd {
	type Item = usize;
	fn next(&mut self) -> Option<Self::Item> {
		loop {
			if unsafe { (*self.alg).dangling } == self.curr {
				break None;
			}
			if val_of!(self.curr) {
				let r = Some(self.i);
				self.i += 1;
				self.curr = next_of!(self.curr);
				break r;
			}

			self.i += 1;
			self.curr = next_of!(self.curr);
		}
	}
}

// Helper taking care of iterator technicalities.
struct Combinations {
	alg: Algorithm,
	iter_next: fn(&mut Combinations) -> Option<SelInd>, // function serving as iterator.next()
}
impl Combinations {
	// handles the initial invocation of iterator.next()
	fn iter_first_combination(&mut self) -> Option<SelInd> {
		Some(SelInd::new(&self.alg))
	}
	// handles subsequent invocations of iterator.next()
	fn iter_next_combination(&mut self) -> Option<SelInd> {
		if self.alg.has_more() {
			self.alg.next_combination();
			Some(SelInd::new(&self.alg))
		} else {
			None
		}
	}
	// the entry point of iterator.next():
	// chains iter_first_combination and iter_next_combination,
	// which is necessary as the algorithm is initially set to the first combination.
	fn iter_entry(&mut self) -> Option<SelInd> {
		let iter_first = Combinations::iter_first_combination;
		self.iter_next = Combinations::iter_next_combination;
		iter_first(self)
	}
}
impl Iterator for Combinations {
	type Item = SelInd;

	fn next(&mut self) -> Option<Self::Item> {
		(self.iter_next)(self)
	}
}

///
/// The <em>cool-lex</em> order and algorithms have been invented by Frank Ruskey and Aaron Williams.
/// Hats off.
///
/// <p>See <a href= "http://webhome.cs.uvic.ca/~ruskey/Publications/Coollex/CoolComb.html">
/// http://webhome.cs.uvic.ca/~ruskey/Publications/Coollex/CoolComb.html</a>.
///
/// <p>See section<b> 3.2. Iterative Algorithms.</b>
///
pub struct CoollexLinkedList {}
impl CoollexLinkedList {
	/// Returns an iterator over the generated combinations. A combination is represented with the
	/// indices of the selected elements.
	///
	/// Param <tt>n</tt>: number of elements to combine; must be <tt> >= k</tt>.
	///
	/// Param <tt>k</tt>: number of elements in each combination; must be non-negative.
	///
	/// Returns an empty iterator if <tt>0</tt> was specified as the number of elements in a
	/// combination; the generated combinations otherwise, as an iterator over iterators over indices.
	///
	/// Panics if <tt>k < 0 || n < k</tt>
	pub fn combinations(n: usize, k: usize) -> Box<dyn Iterator<Item = SelInd>> {
		assert!( n>=k,"number of elements to combine is less than the number of elements in a combination: n={}, k={}",n,k);
		if k != 0 {
			Box::new(Combinations {
				alg: Algorithm::new(n - k, k),
				iter_next: Combinations::iter_entry,
			})
		} else {
			Box::new(empty::<SelInd>())
		}
	}
}
