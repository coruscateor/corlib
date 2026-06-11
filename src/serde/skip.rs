use std::ops::{Deref, DerefMut};

use std::fmt::Debug;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Skip<T>
{

    #[serde(skip)]
    item: T

}

impl<T> Skip<T>
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

impl<T> Deref for Skip<T>
{

    type Target = T;

    fn deref(&self) -> &Self::Target
    {

        &self.item

    }

}

impl<T> DerefMut for Skip<T>
{

    fn deref_mut(&mut self) -> &mut Self::Target
    {

        &mut self.item

    }

}

impl<T> AsRef<T> for Skip<T>
{

    fn as_ref(&self) -> &T
    {

        &self.item
    
    }

}

impl<T> AsMut<T> for Skip<T>
{

    fn as_mut(&mut self) -> &mut T
    {

        &mut self.item
    
    }

}

impl<T> Default for Skip<T>
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

impl<T> Debug for Skip<T>
    where T: Debug
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Skip").field("item", &self.item).finish()
    }

}
