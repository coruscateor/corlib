use core::panic;

use std::{ops::{Index, IndexMut}, slice::Iter};

use crate::inc_dec::*;

pub struct StackedVec<T, const N: usize>
{

    array: [Option<T>; N],
    current_index: usize //Index for pushing and poping.

}

impl<T, const N: usize> StackedVec<T, N>
{

    pub const fn with_capacity() -> Self //<N> //(capacity: N)
    {

        Self
        {

            array: [const { None }; N],
            current_index: 0

        }

    }

    pub fn push(&mut self, value: T) -> Option<T>
    {

        let at_length = self.current_index + 1;

        if at_length >= self.array.len()
        {

            return Some(value);

        }

        self.array[self.current_index] = Some(value);

        self.current_index.pp();

        None

    }

    pub fn pop(&mut self) -> Option<T>
    {

        if self.array.len() == 0
        {

            return None;

        }

        let poped = self.array[self.current_index].take();

        if self.current_index > 0
        {

            self.current_index.mm();

        }

        poped

    }

    pub fn len(&self) -> usize
    {

        self.current_index + 1

    }

    pub fn capacity(&self) -> usize
    {

        self.array.len()

    }

    pub fn current_index(&self) -> usize
    {

        self.current_index

    }

    //Should array elements by mutable by reference?

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

    pub fn is_at_capacity(&self) -> bool
    {

        self.len() == self.capacity()

    }

}

impl<T, const N: usize> Index<usize> for StackedVec<T, N>
{

    type Output = Option<T>;

    fn index(&self, index: usize) -> &Self::Output
    {
        
        if index <= self.current_index
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
        
        if index <= self.current_index
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




