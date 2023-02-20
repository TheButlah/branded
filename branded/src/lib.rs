//! Branded lifetimes in Rust, for dummies!
//!
//! This library lets you "brand" your types using a special lifetime that is guaranteed
//! to be unique, with zero runtime cost. Importantly, creating brands at runtime is
//! free and can be done over and over, with each generated lifetime being unique.
//!
//! # Example Use Cases
//! - Zero-cost array indexing. Check that your index is in bounds *once*, then brand
//!   the index. Branded indices can safely skip bounds checks, because you already
//!   verified it beforehand. You'll get a compile error if you ever try to use
//!   mismatched brands.
//! - The [GhostCell] datastructure, which lets you safely separate a `Mutex` or other
//!   lock from the actual data it is protecting. This lets certain data structures,
//!   like graphs or linked lists, be implemented much more safely without resorting to
//!   wrapping each node in a Mutex
//! -
//!
//! # Safety of this library
//! The only unsafe code is in the examples. There is no `unsafe` in any of the actual
//! library code.
//!
//! However, because `branded` allows users of the library to assume certain properties
//! such as uniquess of a generated `Brand`s lifetime, a bug in branded where that
//! property is failed to be upheld will cause downstream unsafe code to be unsound.
//!
//! For this reason, we welcome any suggested tests or formal proofs to ensure that
//! our guarantees are accurate.
//!
//! # Further reading
//! - <https://lord.io/lifetimes-as-tokens/>
//! - <https://plv.mpi-sws.org/rustbelt/ghostcell/paper.pdf>

#![no_std]

use core::marker::PhantomData;

/// Represents a branded lifetime. Branded lifetimes are guaranteed to be unique.
#[derive(Default, Copy, Clone)]
pub struct Brand<'brand> {
	/// Phantom lifetime type that is invariant in `'a`. Because it is invariant, only the
	/// exact, original lifetime will match.
	///
	/// See also: <https://doc.rust-lang.org/nomicon/subtyping.html#variance>
	_marker: PhantomData<&'brand mut &'brand ()>,
}

impl<'brand> Brand<'brand> {
	/// Creates a new `Brand`, which will be passed to a closure `f`. Then calls `f`
	/// and returns its returned value of type `R`.
	pub fn new<R, F: WithBrand<R>>(f: F) -> R {
		let b = Self {
			_marker: PhantomData,
		};
		f(b)
	}
}

/// Any `FnOnce` closure that accepts a `Brand` and returns an `R`.
///
/// Instead of directly using `FnOnce`, we use this trait alias because there are some
/// additional restrictions placed on the lifetime of the `Brand` that ensure a unique
/// lifetime. Letting users write `impl WithBrand<R>` makes things look less scary.
pub trait WithBrand<R = ()>: for<'b> FnOnce(Brand<'b>) -> R {}
/// Implements `WithBrand` on every posible closure.
impl<F, R> WithBrand<R> for F where F: for<'b> FnOnce(Brand<'b>) -> R {}

#[cfg(test)]
mod tests {

	#[test]
	fn it_works() {
		assert!(true);
	}
}
