use core::fmt;

use std::ops::{Deref, DerefMut};

use std::fmt::{Debug, Display};

use serde::{Serialize, Deserialize};

///
/// Prevents serialisation of its contained object instance. 
/// 
#[derive(Serialize, Deserialize)]
pub struct Skip<T>
{

    #[serde(skip)]
    object: T

}

impl<T> Skip<T>
{

    pub fn new(object: T) -> Self
    {

        Self
        {

            object

        }

    }

    pub fn take(self) -> T
    {

        self.object

    }

}

impl<T> Deref for Skip<T>
{

    type Target = T;

    fn deref(&self) -> &Self::Target
    {

        &self.object

    }

}

impl<T> DerefMut for Skip<T>
{

    fn deref_mut(&mut self) -> &mut Self::Target
    {

        &mut self.object

    }

}

impl<T> AsRef<T> for Skip<T>
{

    fn as_ref(&self) -> &T
    {

        &self.object
    
    }

}

impl<T> AsMut<T> for Skip<T>
{

    fn as_mut(&mut self) -> &mut T
    {

        &mut self.object
    
    }

}

impl<T> Default for Skip<T>
    where T: Default
{

    fn default() -> Self
    {

        Self
        {
            
            object: Default::default()
        
        }

    }
    
}

impl<T> Clone for Skip<T>
    where T: Clone
{

    fn clone(&self) -> Self
    {

        Self
        {
            
            object: self.object.clone()
        
        }

    }

}

impl<T> Display for Skip<T>
    where T: Display
{

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {

        write!(f, "{}", self.object)

    }
    
}

impl<T> Debug for Skip<T>
    where T: Debug
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Skip").field("object", &self.object).finish()
    }

}
