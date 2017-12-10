#![feature(core)]

extern crate core;

#[cfg(test)]
mod test;

use core::iter::FromIterator;
use core::marker::PhantomData;
use core::ops::{Deref,DerefMut};
use core::{iter,mem,ptr,slice};

///Fixed size circular/cyclic/ring buffer
///
///Almost a FIFO (first in, first out) queue.
///· It cannot represent an empty buffer.
///· The queue is always filled (Cannot dequeue without queueing).
///
///When constructed, the internal `list` must not be empty.
#[derive(Copy,Clone,Debug,Eq,PartialEq,Hash)]
pub struct CircularBuffer<T,L = Box<[T]>>{
	pub(crate) list: L,
	pub(crate) first: usize,
	pub(crate) t: PhantomData<Box<[T]>>,
}

impl<T,L> CircularBuffer<T,L>{
	///Constructs the structure from its raw components.
	///
	///# Unsafety
	///
	///This function is unsafe as there is no guarantee that `first < list.len()`, nor whether `list` is non-empty.
	#[inline(always)]
	pub unsafe fn from_raw_parts(list: L,first: usize) -> Self{
		CircularBuffer{list: list,first: first,t: PhantomData}
	}

	///Deconstructs the structure into its raw components
	#[inline(always)]
	pub fn into_raw_parts(self) -> (L,usize){
		(self.list,self.first)
	}
}

impl<T,L> CircularBuffer<T,L> where
	L: Deref<Target=[T]>
{
	///////////////////////////////////////////////////////////////////
	// Methods independent of order
	//

	///Returns the number of elements (before starting to loop around).
	#[inline(always)]
	pub fn len(&self) -> usize{self.list.len()}

	///Returns an iterator over the buffer looping around at the end.
	///This iterator is initially iterating from the most recently queued element to the oldest, and then looping around (relative to `queue`).
	///This creates a never ending iterator
	#[inline]
	pub fn iter_circular<'s>(&'s self) -> IterCircular<'s,T>{
		self.list.iter().cycle().skip(self.first)
	}

	///Returns an iterator over the buffer without looping around.
	///Iterates from the most recently queued element to the oldest queued element (relative to `queue`).
	#[inline]
	pub fn iter<'s>(&'s self) -> Iter<'s,T>{
		self.iter_circular().take(self.len())
	}


	///////////////////////////////////////////////////////////////////
	// Order: Most recently queued to oldest
	//

	#[inline(always)]
	pub fn internal_index(&self,index: usize) -> usize{
		(self.first + index) % self.len()
	}

	///Returns a reference to the element at the given index.
	///Smaller indices are more recently queued elements (relative to `queue`) (0 is the newest).
	///When `index` is out of range, it loops around.
	#[inline]
	pub fn get(&self,index: usize) -> &T{
		unsafe{self.list.get_unchecked(self.internal_index(index))}
	}

	///////////////////////////////////////////////////////////////////
	// Order: Oldest to most recently queued
	//

	#[inline]
	pub fn internal_index_reversed(&self,index: usize) -> usize{
		let len = self.len();
		(self.first + (len - (index % len))) % len
	}
}

impl<T,L> CircularBuffer<T,L> where
	L: DerefMut<Target=[T]>
{
	///////////////////////////////////////////////////////////////////
	// Methods independent of order
	//

	///Swaps the most recently queued element.
	#[inline]
	pub fn swap(&mut self,mut elem: T) -> T{
		mem::swap(unsafe{self.list.get_unchecked_mut(self.first)},&mut elem);
		elem
	}

	///////////////////////////////////////////////////////////////////
	// Order: Most recently queued to oldest
	//

	///Enqueues (push at beginning) the given element at the beginning of the buffer.
	///Dequeues (pop at end) the last element and returns it.
	pub fn queue(&mut self,mut elem: T) -> T{
		self.first = self.internal_index_reversed(1);
		mem::swap(unsafe{self.list.get_unchecked_mut(self.first)},&mut elem); //TODO: copy instead of swap may be faster? Less assignments? But it seems like swap is optimized?
		elem
	}

	///Sets the offset for the first element, relative to the currently first element.
	///Smaller indices are more recently queued elements (relative to `queue`) (0 is the newest).
	///When `index` is out of range, it loops around.
	#[inline]
	pub fn set_first(&mut self,index: usize){
		self.first = self.internal_index(index);
	}

	///Returns a mutable reference to the element at the given index.
	///Smaller indices are more recently queued elements (relative to `queue`) (0 is the newest).
	///When `index` is out of range, it loops around.
	#[inline]
	pub fn get_mut(&mut self,index: usize) -> &mut T{
		let i = self.internal_index(index);
		unsafe{self.list.get_unchecked_mut(i)}
	}

	///Swaps the two elements at the given indices `a` and `b`.
	///Smaller indices are more recently queued elements (relative to `queue`) (0 is the newest).
	///When `a` or `b` are out of range, they loop around.
	#[inline]
	pub fn swap_internal(&mut self,a: usize,b: usize){
		let ia = self.internal_index(a);
		let ib = self.internal_index(b);
		self.list.swap(ia,ib);
	}

	///Swaps the element at the given index with the specifiied new one.
	///When `a` or `b` are out of range, they loop around.
	#[inline]
	pub fn swap_at(&mut self,index: usize,mut elem: T) -> T{
		mem::swap(self.get_mut(index),&mut elem);
		elem
	}

	///////////////////////////////////////////////////////////////////
	// Order: Oldest to most recently queued
	//

	pub fn queue_reversed(&mut self,mut elem: T) -> T{
		mem::swap(unsafe{self.list.get_unchecked_mut(self.first)},&mut elem); //TODO: copy instead of swap may be faster? Less assignments? But it seems like swap is optimized?
		self.first = self.internal_index(1);
		elem
	}
}

impl<T> CircularBuffer<T>{
	#[inline]
	unsafe fn uninitialized(size: usize) -> Self{
		assert!(size > 0);

		let mut buffer = Vec::with_capacity(size);
		buffer.set_len(size);

		CircularBuffer{
			list: buffer.into_boxed_slice(),
			first: 0,
			t: PhantomData
		}
	}
}

impl<T> From<Vec<T>> for CircularBuffer<T>{
	///Constructs an already filled circular buffer from the elements in a vec.
	///The first element in the Vec will be interpreted as the most reecntly queued element, and the last element as the oldest.
	#[inline]
	fn from(vec: Vec<T>) -> Self{
		assert!(vec.len() > 0);
		CircularBuffer{
			list: vec.into_boxed_slice(),
			first: 0,
			t: PhantomData
		}
	}
}

impl<T,L> From<L> for CircularBuffer<T,L> where
	L: Deref<Target=[T]>
{
	#[inline]
	fn from(l: L) -> Self{
		assert!(l.len() > 0);
		CircularBuffer{
			list: l,
			first: 0,
			t: PhantomData
		}
	}
}

impl<T> FromIterator<T> for CircularBuffer<T>{
	#[inline]
	fn from_iter<I>(i: I) -> Self
		where I: IntoIterator<Item=T>
	{
		CircularBuffer::from(Vec::from_iter(i))
	}
}

pub type Iter<'t,T> = iter::Take<IterCircular<'t,T>>;
pub type IterCircular<'t,T> = iter::Skip<iter::Cycle<slice::Iter<'t,T>>>;

pub mod iters{
	use super::*;

	pub struct SavedValues<I: Iterator>(pub CircularBuffer<<I as Iterator>::Item>,pub I);

	impl<I: Iterator> SavedValues<I>{
		pub fn new(mut iter: I,count: usize) -> Option<Self>{
			if count == 0{
				return None;
			}

			let mut buffer = unsafe{CircularBuffer::uninitialized(count)};

			for i in 0..count{
				if let Some(ref x) = iter.next(){
					unsafe{ptr::copy_nonoverlapping(x,buffer.list.get_unchecked_mut(count - i - 1),1)};
				}else{
					return None;
				}
			}

			Some(SavedValues(CircularBuffer::from(buffer),iter))
		}

		#[inline(always)]
		pub fn get(&self,i: usize) -> &<I as Iterator>::Item{
			self.0.get(i)
		}
	}

	impl<I: Iterator> Iterator for SavedValues<I> where
		<I as Iterator>::Item: Clone
	{
		type Item = <I as Iterator>::Item;

		#[inline]
		fn next(&mut self) -> Option<Self::Item>{
			if let Some(x) = self.1.next(){
				self.0.queue(x.clone());
				Some(x)
			}else{
				None
			}
		}
	}
}
