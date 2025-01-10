use std::ops::{Deref, DerefMut};


pub struct AsCopy<T>
    where T: Clone
{

    item: T

}

impl<T> AsCopy<T>
    where T: Clone
{

    pub fn new(item: T) -> Self
    {

        Self
        {

            item

        }

    }

    pub fn take(self) -> T
    {

        self.item

    }

}

impl<T> Deref for AsCopy<T>
    where T: Clone
{

    type Target = T;

    fn deref(&self) -> &Self::Target
    {

        &self.item

    }

}

impl<T> DerefMut for AsCopy<T>
    where T: Clone
{

    fn deref_mut(&mut self) -> &mut Self::Target
    {

        &mut self.item

    }

}

impl<T> Clone for AsCopy<T>
    where T: Clone
{

    fn clone(&self) -> Self
    {

        Self
        {

            item: self.item.clone()
        
        }

    }

}

/*
the trait `std::marker::Copy` cannot be implemented for this typerustcClick for full compiler diagnostic
as_copy.rs(8, 5): this field does not implement `std::marker::Copy`
as_copy.rs(83, 7): consider restricting type parameter `T`: `: std::marker::Copy`
 */

impl<T> Copy for AsCopy<T>
    where T: Clone
{

    

}