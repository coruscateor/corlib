//use std::result::Result;

use std::ops::{Index, IndexMut, RangeBounds};

use std::slice::{Iter, IterMut, SliceIndex};

use std::vec::Drain;

use delegate::delegate;

use super::List;

///
/// A List that only takes items not already present in it. 
///
#[derive(Default)]
pub struct UniqueItemList<T>
    where T: PartialEq
{

    contents: List<T>

}

impl<T> UniqueItemList<T>
    where T: PartialEq
{

    //const

    pub fn new() -> Self
    {

        Self
        {

            contents: List::new()

        }

    }

    pub fn with_capacity(capacity: usize) -> Self
    {

        Self
        {

            contents: List::with_capacity(capacity)

        }

    }


    pub fn add(&mut self, value: T) -> bool
    {

        if !self.contents.contains(&value)
        {

            self.contents.add(value);

            return true;

        }

        false

    }

    pub fn add_copy(&mut self, value: &T) -> bool
        where T: Copy
    {

        if !self.contents.contains(value)
        {

            self.contents.add(*value);

            return true;

        }

        false

    }

    pub fn add_clone(&mut self, value: &T) -> bool
        where T: Clone
    {

        if !self.contents.contains(value)
        {

            self.contents.add(value.clone());

            return true;

        }

        false

    }

    //ensure uniqness

    delegate! {
        to self.contents {

            //pub const fn new(&self) -> Self;

            pub fn add_or_repace(&mut self, value: T);

            pub fn remove(&mut self, value: T) ->  bool;

            pub fn contains(&self, x: &T) -> bool;

            pub fn len(&self) -> usize;

            pub fn is_empty(&self) -> bool;

            pub fn to_vec(&self) -> Vec<T> where T: Clone;

            pub fn pop(&mut self) -> Option<T>;

            pub fn insert(&mut self, index: usize, element: T);

            //#[call(remove)]
            pub fn remove_at(&mut self, index: usize) -> T;

            pub fn first(&self) -> Option<&T>;

            //pub fn first_mut(&mut self) -> Option<&mut T>;

            pub fn last(&self) -> Option<&T>;

            //pub fn last_mut(&mut self) -> Option<&mut T>;

            pub fn iter(&self) -> Iter<'_, T>;

            //pub fn iter_mut(&mut self) -> IterMut<'_, T>;

            //#[call(push)]
            //pub fn add(&mut self, value: T);

            pub fn get_last_index(&self) -> Option<usize>;

            pub fn index_of(&self, value: &T) -> Option<usize>;

            pub fn reverse(&mut self);

            pub fn get<I>(&self, index: I) -> Option<&<I as SliceIndex<[T]>>::Output> where I: SliceIndex<[T]>;

            //pub fn get_mut<I>(&mut self, index: I) -> Option<&mut <I as SliceIndex<[T]>>::Output> where I: SliceIndex<[T]>;

            pub fn drain<R>(&mut self, range: R) -> Drain<'_, T> where R: RangeBounds<usize>;

            pub fn drain_all(&mut self) -> Drain<'_, T>;

        }

    }

}

impl<T> UniqueItemList<T> where
    T: Copy + PartialEq
{

    delegate! {
        to self.contents {

            pub fn add_or_repace_copy(&mut self, value: T) -> Option<T>;

        }
    }

}

impl<T> UniqueItemList<T> where
    T: Clone + PartialEq
{

    //no default values

    /*
    pub fn resize(&mut self, new_len: usize, value: T)
    {

        self.contents.resize(new_len, value)

    }
    */

    delegate! {
        to self.contents {

            pub fn add_or_repace_clone(&mut self, value: T) -> Option<T>;

        }
        
    }

}

impl<T> Index<usize> for UniqueItemList<T> where
    T: PartialEq
{

    type Output = T;

    delegate! {
        to self.contents {

            fn index(&self, index: usize) -> &Self::Output;

        }
        
    }

    /*
    fn index(&self, index: usize) -> &Self::Output
    {
        
        self.contents.index(index)

    }
    */

}

impl<T> IndexMut<usize> for UniqueItemList<T> where
    T: PartialEq
{


    delegate! {
        to self.contents {

            fn index_mut(&mut self, index: usize) -> &mut Self::Output;

        }
        
    }

}


impl<T> Clone for UniqueItemList<T> where
    T: PartialEq + Clone
{

    /*
    mismatched types
    expected struct `UniqueItemList<T>`
    found struct `list::List<T>`rustcE0308
    */

    /*
    delegate! {
        to self.contents {

            fn clone(&self) -> Self;

        }
        
    }
    */

    fn clone(&self) -> Self
    {

        Self
        { 
            
            contents: self.contents.clone()
        
        }

    }

}

