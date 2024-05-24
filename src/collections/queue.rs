
use std::{collections::{VecDeque, TryReserveError, vec_deque::{Iter, IterMut, Drain}}, ops::RangeBounds, cmp::Ordering};

use delegate::delegate;

///
/// The standard double ended VecDeque, with the API of a single ended queue.
/// 
pub struct Queue<T>
{

    contents: VecDeque<T>

}

impl<T> Queue<T>
{

    pub const fn new() -> Self
    {

        Self
        {

            contents: VecDeque::new()

        }

    }

    pub fn with_capacity(capacity: usize) -> Self
    {

        Self
        {

            contents: VecDeque::with_capacity(capacity)

        }

    }

    pub fn from_vec_dequeue(&self, other: VecDeque<T>) -> Self
    {

        Self
        {

            contents: other

        }

    }

    pub fn into_vec_dequeue(self) -> VecDeque<T>
    {

        self.contents

    }

    ///
    /// Pops the front item of the Queue.
    /// 
    pub fn pop(&mut self) -> Option<T>
    {

        self.contents.pop_front()

    }

    ///
    /// Pushes the provided item to the back of the Queue.
    /// 
    pub fn push(&mut self, value: T)
    {

        self.contents.push_back(value)

    }

    ///
    /// Peek the front item of the Queue
    ///
    pub fn peek(&self) -> Option<&T>
    {

        self.contents.front()

    }

    ///
    /// Mutably peek the front item of the Queue
    ///
    pub fn peek_mut(&mut self) -> Option<&mut T>
    {

        self.contents.front_mut()

    }

    ///
    /// Peek the back item of the Queue
    ///
    pub fn last(&self) -> Option<&T>
    {

        self.contents.back()

    }

    ///
    /// Mutably peek the back item of the Queue
    ///
    pub fn last_mut(&mut self) -> Option<&mut T>
    {

        self.contents.back_mut()

    }

    delegate! {
        to self.contents {

            pub fn get(&self, index: usize) -> Option<&T>;

            pub fn get_mut(&mut self, index: usize) -> Option<&mut T>;

            pub fn swap(&mut self, i: usize, j: usize);

            pub fn capacity(&self) -> usize;

            pub fn reserve_exact(&mut self, additional: usize);

            pub fn reserve(&mut self, additional: usize);

            pub fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError>;

            pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError>;

            pub fn shrink_to_fit(&mut self);

            pub fn shrink_to(&mut self, min_capacity: usize);

            pub fn truncate(&mut self, len: usize);

            //pub fn allocator(&self) -> &A;

            pub fn iter(&self) -> Iter<'_, T>;

            pub fn iter_mut(&mut self) -> IterMut<'_, T>;

            pub fn as_slices(&self) -> (&[T], &[T]);

            pub fn as_mut_slices(&mut self) -> (&mut [T], &mut [T]);

            pub fn len(&self) -> usize;

            pub fn is_empty(&self) -> bool;

            pub fn range<R>(&self, range: R) -> Iter<'_, T>
                where R: RangeBounds<usize>;

            pub fn range_mut<R>(&mut self, range: R) -> IterMut<'_, T>
                where R: RangeBounds<usize>;

            pub fn drain<R>(&mut self, range: R) -> Drain<'_, T> //, A>
                where R: RangeBounds<usize>;

            pub fn clear(&mut self);            

            pub fn contains(&self, x: &T) -> bool
                where T: PartialEq<T>;
            
            //pub fn front(&self) -> Option<&T>;

            //pub fn front_mut(&mut self) -> Option<&mut T>;

            //pub fn back(&self) -> Option<&T>;

            //pub fn back_mut(&mut self) -> Option<&mut T>;

            //poping

            //pushing

            pub fn swap_remove_front(&mut self, index: usize) -> Option<T>;

            pub fn swap_remove_back(&mut self, index: usize) -> Option<T>;

            pub fn insert(&mut self, index: usize, value: T);

            pub fn remove(&mut self, index: usize) -> Option<T>;

            //pub fn split_off(&mut self, at: usize) -> VecDeque<T, A>
            //   where A: Clone;

            pub fn append(&mut self, other: &mut VecDeque<T>); //, A

            pub fn retain<F>(&mut self, f: F)
                where F: FnMut(&T) -> bool;

            pub fn retain_mut<F>(&mut self, f: F)
                where F: FnMut(&mut T) -> bool;

            pub fn resize_with(&mut self, new_len: usize, generator: impl FnMut() -> T);

            pub fn make_contiguous(&mut self) -> &mut [T];

            pub fn rotate_left(&mut self, mid: usize);

            pub fn rotate_right(&mut self, k: usize);

            pub fn binary_search(&self, x: &T) -> Result<usize, usize>
                where T: Ord;

            pub fn binary_search_by<'a, F>(&'a self, f: F) -> Result<usize, usize>
                where F: FnMut(&'a T) -> Ordering;

            pub fn partition_point<P>(&self, pred: P) -> usize
                where P: FnMut(&T) -> bool;
            


        }
    }

}

/*
impl<T, A> Queue<T>
    where T: Clone, A: Allocator
{

            
    pub fn resize(&mut self, new_len: usize, value: T);

}
*/


impl<T> Clone for Queue<T>
    where T: Clone
{

    fn clone(&self) -> Self
    {

        Self { contents: self.contents.clone() }

    }

}

impl<T> Queue<T>
    where T: Clone
{

    fn clone_from_vec_dequeue(&self, other: VecDeque<T>) -> Self
    {

        Self
        {

            contents: other.clone()

        }

    }

    fn clone_into_vec_dequeue(&self) -> VecDeque<T>
    {

        self.contents.clone()

    }

}

impl<T> Default for Queue<T>
{

    fn default() -> Self
    {

        Self
        {
            
            contents: Default::default()
        
        }

    }

}



