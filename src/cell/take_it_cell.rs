use std::{cell::Cell, fmt::Debug};

use delegate::delegate;

pub struct TakeItCell<T>
{

    cell: Cell<Option<T>>

}

impl<T> TakeItCell<T>
{

    pub fn new(value: T) -> Self
    {

        Self
        {

            cell: Cell::new(Some(value))

        }

    }

    pub fn empty() -> Self
    {

        Self
        {

            cell: Cell::new(None)

        }
        
    }

    pub fn set(&self, val: T)
    {

        self.cell.set(Some(val));

    }

    pub fn set_opt(&self, val: Option<T>)
    {

        self.cell.set(val);

    }

    pub fn replace(&self, val: T) -> Option<T>
    {

        self.cell.replace(Some(val))

    }

    pub fn replace_opt(&self, val: Option<T>) -> Option<T>
    {

        self.cell.replace(val)

    }

    pub fn take(&self) -> Option<T>
    {

        self.cell.take()

    }

    delegate!
    {

        to self.cell
        {

            pub fn swap(&self, other: &Cell<Option<T>>);

            pub fn into_inner(self) -> Option<T>; //const

        }

    }
    
}


impl<T> TakeItCell<T>
    where T: Copy
{

    delegate!
    {

        to self.cell
        {

            pub fn get(&self) -> Option<T>;

        }

    }
}

impl<T> Clone for TakeItCell<T>
    where T: Copy
{

    fn clone(&self) -> Self {
        Self { cell: self.cell.clone() }
    }

}

impl<T> Debug for TakeItCell<T>
    where T: Copy + Debug
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TakeItCell").field("cell", &self.cell).finish()
    }

}

impl<T> Default for TakeItCell<T>
{

    fn default() -> Self {
        Self { cell: Default::default() }
    }
    
}
