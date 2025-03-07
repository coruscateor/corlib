use std::{fmt::Debug, ops::Deref};

///
/// This object makes its contained object externally immutable only.
/// 
pub struct Immut<T>
{

    item: T

}

impl<T> Immut<T>
{

    pub fn new(item: T) -> Self
    {

        Self
        {

            item

        }

    }

}

impl<T> Deref for Immut<T>
{

    type Target = T;

    fn deref(&self) -> &Self::Target
    {

        &self.item

    }

}

impl<T> AsRef<T> for Immut<T>
{

    fn as_ref(&self) -> &T
    {

        &self.item
    
    }

}

impl<T> Default for Immut<T>
    where T: Default
{

    fn default() -> Self
    {

        Self
        {
            
            item: Default::default()
        
        }

    }
    
}

impl<T> Debug for Immut<T>
    where T: Debug
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Immut").field("item", &self.item).finish()
    }

}

