use core::panic;

use std::{ascii::escape_default, fmt::Display, ops::{Index, IndexMut}, slice::{Iter, IterMut}};

use crate::inc_dec::*;

use std::fmt::Debug;

use std::mem::take;

#[cfg(feature = "serde")]
use serde::ser::SerializeSeq;

#[cfg(feature = "serde")]
use serde::{Serialize, Serializer};

pub struct StackedVec<T, const N: usize>
    where T: Default + Copy
{

    array: [T; N],
    //last_index: usize //Index for pushing and poping.
    len: usize

}

impl<T, const N: usize> StackedVec<T, N>
    where T: Default + Copy
{

    pub fn new() -> Self
    {

        Self
        {

            array: [T::default(); N],
            len: 0

        }

    }

    /*
    pub const fn with_capacity() -> Self //<N> //(capacity: N)
    {

        Self
        {

            array: [const { None }; N],
            current_index: 0

        }

    }
    */

    pub fn push(&mut self, value: T) -> Option<T>
    {

        let next_last_index = self.len;

        if next_last_index >= self.capacity()
        {

            //Can't fit

            return Some(value);

        }

        self.array[next_last_index] = value;

        self.len.pp();

        None

    }

    pub fn pop(&mut self) -> Option<T>
    {

        if self.len == 0
        {

            return None;

        }

        let last_index = self.len - 1;

        let poped = take(&mut self.array[last_index]);

        //let poped = self.array[last_index];

        //self.array[last_index] = T::default();

        self.len = last_index;

        Some(poped)

    }

    pub fn len(&self) -> usize
    {

        self.len

    }

    pub const fn capacity(&self) -> usize
    {

        self.array.len()

    }

    pub fn last_index(&self) -> Option<usize>
    {

        let len = self.len;

        if len == 0
        {

            None

        }
        else
        {

            Some(len - 1)
            
        }

    }

    //Should array elements by mutable by reference? No.

    pub fn try_index(&mut self, index: usize) -> Option<&T>
    {

        if index < self.capacity()
        {

            Some(&self.array[index])

        }
        else
        {

            None
            
        }

    }

    pub fn try_mut_index(&mut self, index: usize) -> Option<&mut T>
    {

        if self.len == 0
        {

            return None;

        }

        let last_index = self.len - 1;

        if index <= last_index
        {

            Some(&mut self.array[index])

        }
        else
        {

            None
            
        }

    }

    pub fn is_full(&self) -> bool
    {

        if self.capacity() == 0
        {

            //An array with no capacity can never be full.

            return false;

        }

        self.len() == self.capacity()

    }

    pub fn is_empty(&self) -> bool
    {

        self.len() == 0

    }

    pub fn iter<'a>(&'a self) -> StackedVecIterator<'a, T>
    {

        let last_index;

        if self.len > 1
        {

            last_index = self.len - 1;

        }
        else
        {

            return StackedVecIterator::new(self.array[..].iter());
            
        }

        StackedVecIterator::new(self.array[..last_index].iter())

    }

    pub fn iter_mut<'a>(&'a mut self) -> StackedVecIteratorMut<'a, T>
    {

        let last_index;

        if self.len > 1 //.last_index > 1
        {

            last_index = self.len - 1; //.last_index - 1;

        }
        else
        {

            //if self.array.len() == 0
            //{

            return StackedVecIteratorMut::new(self.array[..].iter_mut());

            //}

            //last_index = 0;
            
        }

        StackedVecIteratorMut::new(self.array[..last_index].iter_mut())

    }

    pub fn insert(&mut self, index: usize, item: T) -> Option<T>
    {

        if self.len == 0
        {

            //No room

            Some(item)

        }
        else if index <= self.len && !self.is_full()
        {

            let mut current_index = self.len - 1;

            //Move all items including that at the specified index to the right.

            while current_index >= index
            {

                let current_item = self.array[current_index];

                self.array[current_index + 1] = current_item;

                current_index.mm();

            }

            //Finally insert the item at the specified index and increment the length. 

            self.array[index] = item;

            self.len.pp();

            None

        }
        else
        {

            Some(item)

        }

    }

    pub fn remove(&mut self, index: usize) -> Option<T>
    {

        if self.len == 0
        {

            None

        }
        else if index < self.len
        {

            let removed_item = self.array[index];

            //Move all items to the left to close the gap.

            let mut current_index = index + 1;

            while current_index < self.len
            {

                let current_item = self.array[current_index];

                self.array[current_index - 1] = current_item;

                current_index.pp();

            }

            self.len.mm();

            Some(removed_item)

        }
        else
        {

            None
            
        }

    }

    pub fn clear(&mut self)
    {

        if self.len > 0
        {

            let last_index = self.len - 1;

            for item in self.array[..last_index].iter_mut()
            {
    
                *item = T::default();
    
            }

            self.len = 0;

        }

    }

    pub fn first(&self) -> Option<&T>
    {

        if self.len == 0
        {

            None

        }
        else
        {

            Some(&self.array[0])
            
        }

    }

    pub fn first_mut(&mut self) -> Option<&mut T>
    {

        if self.len == 0
        {

            None

        }
        else
        {

            Some(&mut self.array[0])
            
        }

    }

    pub fn last(&self) -> Option<&T>
    {

        if self.len == 0
        {

            None

        }
        else
        {

            Some(&self.array[self.len - 1])
            
        }

    }

    pub fn last_mut(&mut self) -> Option<&mut T>
    {

        if self.len == 0
        {

            None

        }
        else
        {

            Some(&mut self.array[self.len - 1])
            
        }

    }

    /*
    pub fn as_slice(&self) -> &[&T]
    {

        let array = [&T; self.len];



    }
    */

    pub fn as_slice(&self) -> &[T]
    {

        if self.len == 0
        {

            &self.array[..]

        }
        else
        {
            let last_index = self.len - 1;

            &self.array[..last_index]
            
        }
        
    }

    //No mut slices

    pub fn contains(&self, val_ref: &T) -> bool
        where T: PartialEq
    {

        for item in self.iter()
        {

            if item == val_ref
            {

                return true;

            }

        }

        false

    }

}

impl<T, const N: usize> Index<usize> for StackedVec<T, N>
    where T: Default + Copy
{

    //type Output = Option<&T>;

    type Output = T;

    fn index(&self, index: usize) -> &Self::Output
    {

        if index >= self.len
        {

            panic!("Error: The provided index is out of bounds")

        }

        &self.array[index]

        /*
        let len = self.len;

        if len == 0
        {

            &None

        }
        else
        {

            if index < len
            {
    
                &Some(&self.array[index])
    
            }
            else
            {
                
                &None
    
            }

        }
        */

    }

}

impl<T, const N: usize> IndexMut<usize> for StackedVec<T, N>
    where T: Default + Copy
{

    fn index_mut(&mut self, index: usize) -> &mut Self::Output
    {

        if index >= self.len
        {

            panic!("Error: The provided index is out of bounds")

        }

        &mut self.array[index]

        /*
        let panic_message: &'static str = "Error: Invalid index";

        let len = self.len;

        if len == 0
        {

            panic!("{}", panic_message)

            //&None

        }
        else
        {
        
            if index < len //self.last_index
            {

                &mut self.array[index]

            }
            else
            {

                panic!("{}", panic_message)
                
                //panic!("Error: Invalid index")

                //&mut None

            }

        }
        */

    }

}

impl<T, const N: usize> Display for StackedVec<T, N>
    where T: Display + Default + Copy
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {

        let last_index;

        if self.capacity() == 0 || self.len() == 0
        {

            last_index = 0;

        }
        else
        {

            last_index = self.len() - 1;

        }

        let mut current_index = 0;

        f.write_str("[")?;

        for item in self.iter()
        {

            write!(f, "{}", item)?;

            if current_index < last_index
            {

                f.write_str(", ")?;

            }
            else
            {

                break;
                
            }

            current_index.pp();

        }

        f.write_str("]")?;

        Ok(())
        
    }

}

impl<T, const N: usize> Debug for StackedVec<T, N>
    where T: Debug + Default + Copy
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StackedVec").field("array", &self.array).field("len", &self.len).finish()
    }

}

impl<T, const N: usize> Clone for StackedVec<T, N>
    where T: Clone + Default + Copy
{

    fn clone(&self) -> Self {
        Self { array: self.array.clone(), len: self.len.clone() }
    }
    
}


impl<T, const N: usize> Copy for StackedVec<T, N>
    where T: Default + Copy
{

}

#[cfg(feature = "serde")]
impl<T, const N: usize> Serialize for StackedVec<T, N>
    where T: Default + Copy + Serialize
{

    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer
    {

        let mut seq = serializer.serialize_seq(Some(self.len))?;

        let valid_range = self.as_slice();

        for item in valid_range
        {

            seq.serialize_element(item)?;

        }

        seq.end()

        //serializer.serialize_str(self.as_str())

    }

}

//StackedVecIterator

pub struct StackedVecIterator<'a, T>
{

    opt_iter: Iter<'a, T>

}

impl<'a, T> StackedVecIterator<'a, T>
{

    pub fn new(opt_iter: Iter<'a, T>) -> Self
    {

        Self
        {

            opt_iter

        }

    }

}

impl<'a, T> Iterator for StackedVecIterator<'a, T>
{

    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item>
    {

        match self.opt_iter.next()
        {

            Some(val) =>
            {

                Some(val)

            }
            None => None

        }

    }

}

pub struct StackedVecIteratorMut<'a, T>
{

    opt_iter_mut: IterMut<'a, T>

}

impl<'a, T> StackedVecIteratorMut<'a, T>
{

    pub fn new(opt_iter_mut: IterMut<'a, T>) -> Self
    {

        Self
        {

            opt_iter_mut

        }

    }

}

impl<'a, T> Iterator for StackedVecIteratorMut<'a, T>
{

    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item>
    {

        match self.opt_iter_mut.next()
        {

            Some(val) =>
            {

                Some(val)

            }
            None => None
            
        }

    }

}

