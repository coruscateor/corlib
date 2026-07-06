use core::fmt;

use core::ops::{Deref, DerefMut};

use core::fmt::{Debug, Display};

use serde::{Serialize, Deserialize};

use core::hash::Hash;

///
/// Prevents serialisation of its contained object. 
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

    pub fn from(object_ref: &T) -> Self
        where T: Clone
    {

        Self
        {

            object: object_ref.clone()

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

impl<T> Copy for Skip<T>
    where T: Copy
{
}

impl<T> Display for Skip<T>
    where T: Display
{

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {

        write!(f, "{}", self.object)

    }
    
}

impl<T> PartialEq for Skip<T>
    where T: PartialEq
{

    fn eq(&self, other: &Self) -> bool
    {

        self.object == other.object

    }

}

impl<T> Eq for Skip<T>
    where T: Eq
{
}

impl<T> PartialOrd for Skip<T>
    where T: PartialOrd
{

    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering>
    {

        self.object.partial_cmp(&other.object)

    }

}

impl<T> Ord for Skip<T>
    where T: Ord
{

    fn cmp(&self, other: &Self) -> core::cmp::Ordering
    {
        
        self.object.cmp(other)

    }

}

impl<T> Hash for Skip<T>
    where T: Hash
{

    fn hash<H: core::hash::Hasher>(&self, state: &mut H)
    {

        self.object.hash(state);

    }

}

impl<T> Debug for Skip<T>
    where T: Debug
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Skip").field("object", &self.object).finish()
    }

}
