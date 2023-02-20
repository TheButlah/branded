use branded::Brand;

fn main() {
	FastVec::new(vec![0, 1, 2, 3], |fv| {
		let idx1 = fv.make_idx(1).unwrap();
		let idx3 = fv.make_idx(3).unwrap();
		assert!(fv.make_idx(4).is_none());

		for _ in 0..10 {
			println!("{}", fv.get(idx1) + fv.get(idx3));
		}
	})
}

struct FastVec<'b, T> {
	inner: Vec<T>,
	b: Brand<'b>,
}
#[derive(Clone, Copy)]
struct Idx<'b> {
	idx: usize,
	_b: Brand<'b>,
}

impl<'b, T> FastVec<'b, T> {
	/// Create a new branded `FastVec`, passing it to a closure for use.
	fn new(inner: Vec<T>, f: impl for<'new_brand> FnOnce(FastVec<'new_brand, T>)) {
		Brand::new(|b: Brand<'_>| {
			let v = FastVec { inner, b };
			f(v)
		});
	}

	/// One-time validation that `idx` is in bounds.
	fn make_idx(&self, idx: usize) -> Option<Idx<'b>> {
		if idx < self.inner.len() {
			Some(Idx { idx, _b: self.b })
		} else {
			None
		}
	}
	/// Skips bounds checking, using an already validated index.
	fn get(&self, idx: Idx<'b>) -> &T {
		unsafe { self.inner.get_unchecked(idx.idx) }
	}
}
