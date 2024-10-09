use core::panic;

use std::{fmt::Display, ops::{Index, IndexMut}, slice::{Iter, IterMut}};

use crate::inc_dec::*;

use std::fmt::Debug;

pub struct StackedVec<T, const N: usize>
{

    array: [Option<T>; N],
    last_index: usize //Index for pushing and poping.

}

impl<T, const N: usize> StackedVec<T, N>
{

    pub const fn new() -> Self
    {

        Self
        {

            array: [const { None }; N],
            last_index: 0

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

        let next_last_index = self.len();

        if next_last_index >= self.array.len()
        {

            //Can't fit

            return Some(value);

        }

        self.array[next_last_index] = Some(value);

        self.last_index = next_last_index;

        None

    }

    pub fn pop(&mut self) -> Option<T>
    {

        if self.array.len() == 0
        {

            return None;

        }

        let poped = self.array[self.last_index].take();

        if self.last_index > 0
        {

            self.last_index.mm();

        }

        poped

    }

    pub fn len(&self) -> usize
    {

        self.last_index + 1

    }

    pub const fn capacity(&self) -> usize
    {

        self.array.len()

    }

    pub fn last_index(&self) -> usize
    {

        self.last_index

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

        if self.capacity() == 0
        {

            return None;

        }

        let last_index = self.last_index;

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

            return true;

        }

        self.len() == self.capacity()

    }

    pub fn is_empty(&self) -> bool
    {

        self.len() == 0

    }

    pub fn iter<'a>(&'a self) -> StackedVecIterator<'a, T> //Iter<'a, Option<T>>
    {

        let last_index;

        if self.last_index > 1
        {

            last_index = self.last_index;

        }
        else
        {

            if self.array.len() == 0
            {

                return StackedVecIterator::new(self.array[..].iter());

            }

            last_index = 0;
            
        }

        StackedVecIterator::new(self.array[..last_index].iter())

    }

    pub fn iter_mut<'a>(&'a mut self) -> StackedVecIteratorMut<'a, T>
    {

        let last_index;

        if self.last_index > 1
        {

            last_index = self.last_index - 1;

        }
        else
        {

            if self.array.len() == 0
            {

                return StackedVecIteratorMut::new(self.array[..].iter_mut());

            }

            last_index = 0;
            
        }

        StackedVecIteratorMut::new(self.array[..last_index].iter_mut())

    }

}

impl<T, const N: usize> Index<usize> for StackedVec<T, N>
{

    type Output = Option<T>;

    fn index(&self, index: usize) -> &Self::Output
    {
        
        if index <= self.last_index
        {

            &self.array[index]

        }
        else
        {
            
            &None

        }

    }

}

impl<T, const N: usize> IndexMut<usize> for StackedVec<T, N>
{
    
    //type Output = Option<T>;

    fn index_mut(&mut self, index: usize) -> &mut Self::Output
    {
        
        if index <= self.last_index
        {

            &mut self.array[index]

        }
        else
        {
            
            panic!("Error: Invalid index")

            //&mut None

        }

    }

}

impl<T, const N: usize> Display for StackedVec<T, N>
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

impl<T, const N: usize> Debug for StackedVec<T, N>
    where T: Debug //?Sized +
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StackedVec").field("array", &self.array).field("last_index", &self.last_index).finish()
    }

}

impl<T, const N: usize> Clone for StackedVec<T, N>
    where T: Clone 
{

    fn clone(&self) -> Self {
        Self { array: self.array.clone(), last_index: self.last_index.clone() }
    }
    
}

/*
impl<'a, T, const N: usize> IntoIterator for &'a StackedVec<T, N>
{

    type Item = &'a Option<T>;

    type IntoIter = Iter<'a, Option<T>>;

    fn into_iter(self) -> Self::IntoIter
    {

        //Iter::
    }
}
*/

pub struct StackedVecIterator<'a, T>
{

    opt_iter: Iter<'a, Option<T>>

}

impl<'a, T> StackedVecIterator<'a, T>
{

    pub fn new(opt_iter: Iter<'a, Option<T>>) -> Self
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

                val.as_ref()

            }
            None => None

        }

    }

}

pub struct StackedVecIteratorMut<'a, T>
{

    opt_iter_mut: IterMut<'a, Option<T>>

}

impl<'a, T> StackedVecIteratorMut<'a, T>
{

    pub fn new(opt_iter_mut: IterMut<'a, Option<T>>) -> Self
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

                val.as_mut()

            }
            None => None
            
        }

    }

}


