use core::panic;

use std::{ascii::escape_default, fmt::Display, ops::{Index, IndexMut}, slice::{Iter, IterMut}};

use crate::inc_dec::*;

use std::fmt::Debug;

pub struct NonDefaultStackedVec<T, const N: usize>
{

    array: [Option<T>; N],
    len: usize

}

impl<T, const N: usize> NonDefaultStackedVec<T, N>
{

    pub const fn new() -> Self
    {

        Self
        {

            array: [const { None }; N],
            len: 0

        }

    }

    pub fn push(&mut self, value: T) -> Option<T>
    {

        let next_last_index = self.len;

        if next_last_index >= self.capacity()
        {

            //Can't fit

            return Some(value);

        }

        self.array[next_last_index] = Some(value);

        //self.last_index = next_last_index;

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

        let poped = self.array[last_index].take(); //self.

        self.len = last_index;

        /*
        if last_index > 0 //self.
        {

            self.last_index.mm();

        }
        */

        poped

    }

    pub fn len(&self) -> usize
    {

        self.len

        //self.last_index + 1

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

        //self.last_index

    }

    //Should array elements by mutable by reference? No.

    pub fn try_index(&mut self, index: usize) -> Option<&T>
    {

        if index < self.capacity()
        {

            self.array[index].as_ref()

        }
        else
        {

            None
            
        }

    }

    /*
    pub fn try_index(&mut self, index: usize) -> Option<&Option<T>>
    {

        if index <= self.current_index
        {

            Some(&self.array[index])

        }
        else
        {

            None
            
        }

    }
    */

    pub fn try_mut_index(&mut self, index: usize) -> Option<&mut T>
    {

        if self.len == 0
        {

            return None;

        }

        let last_index = self.len - 1; //.last_index;

        /*
        if self.last_index == 0
        {

            last_index = 0;

        }
        else
        {

            last_index = self.last_index;
            
        }
        */

        /*
        if self.is_full()
        {

            last_index = self.capacity() - 1;

        }
        */

        if index <= last_index //last_index
        {

            self.array[index].as_mut()

        }
        else
        {

            None
            
        }

    }

    /*
    pub fn try_mut_index(&mut self, index: usize) -> Option<&mut Option<T>>
    {

        if index <= self.current_index
        {

            Some(&mut self.array[index])

        }
        else
        {

            None
            
        }

    }
    */

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

    pub fn iter<'a>(&'a self) -> NonDefaultStackedVecIterator<'a, T> //Iter<'a, Option<T>>
    {

        let last_index;

        if self.len > 1 //.last_index > 1
        {

            last_index = self.len - 1; //.last_index;

        }
        else
        {

            //if self.array.len() == 0
            //{

            return NonDefaultStackedVecIterator::new(self.array[..].iter());

            //}

            //last_index = 0;
            
        }

        NonDefaultStackedVecIterator::new(self.array[..last_index].iter())

    }

    pub fn iter_mut<'a>(&'a mut self) -> NonDefaultStackedVecIteratorMut<'a, T>
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

            return NonDefaultStackedVecIteratorMut::new(self.array[..].iter_mut());

            //}

            //last_index = 0;
            
        }

        NonDefaultStackedVecIteratorMut::new(self.array[..last_index].iter_mut())

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

                let current_item = self.array[current_index].take();

                self.array[current_index + 1] = current_item;

                current_index.mm();

            }

            //Finally insert the item at the specified index and increment the length. 

            self.array[index] = Some(item);

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

            let removed_item = self.array[index].take();

            //Move all items to the left to close the gap.

            let mut current_index = index + 1;

            while current_index < self.len
            {

                let current_item = self.array[current_index].take();

                self.array[current_index - 1] = current_item;

                current_index.pp();

            }

            self.len.mm();

            removed_item

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
    
                *item = None;
    
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

            self.array[0].as_ref()
            
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

            self.array[0].as_mut()
            
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

            self.array[self.len - 1].as_ref()
            
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

            self.array[self.len - 1].as_mut()
            
        }

    }

    /*
    pub fn as_slice(&self) -> &[&T]
    {

        let array = [&T; self.len];



    }
    */

    /*
    pub fn as_slice(&self) -> &[Option<T>]
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
    */

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

impl<T, const N: usize> Index<usize> for NonDefaultStackedVec<T, N>
{

    type Output = Option<T>;

    fn index(&self, index: usize) -> &Self::Output
    {

        let len = self.len;

        if len == 0
        {

            &None

        }
        else
        {

            if index < len //self.last_index
            {
    
                &self.array[index]
    
            }
            else
            {
                
                &None
    
            }

        }

    }

}

impl<T, const N: usize> IndexMut<usize> for NonDefaultStackedVec<T, N>
{
    
    //type Output = Option<T>;

    fn index_mut(&mut self, index: usize) -> &mut Self::Output
    {

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

    }

}

impl<T, const N: usize> Display for NonDefaultStackedVec<T, N>
    where T: Display
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

impl<T, const N: usize> Debug for NonDefaultStackedVec<T, N>
    where T: Debug //?Sized +
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NonDefaultStackedVec").field("array", &self.array).field("len", &self.len).finish()
    }

}

impl<T, const N: usize> Clone for NonDefaultStackedVec<T, N>
    where T: Clone 
{

    fn clone(&self) -> Self {
        Self { array: self.array.clone(), len: self.len.clone() }
    }
    
}

impl<T, const N: usize> Copy for NonDefaultStackedVec<T, N>
    where T: Copy
{

}

//NonDefaultStackedVecIterator

pub struct NonDefaultStackedVecIterator<'a, T>
{

    opt_iter: Iter<'a, Option<T>>

}

impl<'a, T> NonDefaultStackedVecIterator<'a, T>
{

    pub fn new(opt_iter: Iter<'a, Option<T>>) -> Self
    {

        Self
        {

            opt_iter

        }

    }

}

impl<'a, T> Iterator for NonDefaultStackedVecIterator<'a, T>
{

    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item>
    {

        match self.opt_iter.next()
        {

            Some(val) =>
            {

                val.as_ref()

            }
            None => None

        }

    }

}

pub struct NonDefaultStackedVecIteratorMut<'a, T>
{

    opt_iter_mut: IterMut<'a, Option<T>>

}

impl<'a, T> NonDefaultStackedVecIteratorMut<'a, T>
{

    pub fn new(opt_iter_mut: IterMut<'a, Option<T>>) -> Self
    {

        Self
        {

            opt_iter_mut

        }

    }

}

impl<'a, T> Iterator for NonDefaultStackedVecIteratorMut<'a, T>
{

    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item>
    {

        match self.opt_iter_mut.next()
        {

            Some(val) =>
            {

                val.as_mut()

            }
            None => None
            
        }

    }

}


