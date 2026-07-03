use core::{fmt::{Debug, Display}, ops::Deref, hash::Hash};

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize, Serializer, Deserializer};

///
/// Guarantees external immutability for its contained object.
/// 
/// Only the internal object is serialised or deserialised, if it supports doing this and any of the relevant serde features are enabled.
/// 
pub struct Immut<T>
{

    object: T

}

impl<T> Immut<T>
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

}

impl<T> Deref for Immut<T>
{

    type Target = T;

    fn deref(&self) -> &Self::Target
    {

        &self.object

    }

}

impl<T> AsRef<T> for Immut<T>
{

    fn as_ref(&self) -> &T
    {

        &self.object
    
    }

}

impl<T> Default for Immut<T>
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

impl<T> Clone for Immut<T>
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

impl<T> Copy for Immut<T>
    where T: Copy
{
}

impl<T> Display for Immut<T>
    where T: Display
{

    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
    {

        write!(f, "{}", self.object)

    }
    
}

impl<T> PartialEq for Immut<T>
    where T: PartialEq
{

    fn eq(&self, other: &Self) -> bool
    {

        self.object == other.object

    }

}

impl<T> Eq for Immut<T>
    where T: Eq
{
}

impl<T> PartialOrd for Immut<T>
    where T: PartialOrd
{

    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering>
    {

        self.object.partial_cmp(&other.object)

    }

}

impl<T> Ord for Immut<T>
    where T: Ord
{

    fn cmp(&self, other: &Self) -> core::cmp::Ordering
    {
        
        self.object.cmp(other)

    }

}

impl<T> Hash for Immut<T>
    where T: Hash
{

    fn hash<H: core::hash::Hasher>(&self, state: &mut H)
    {

        self.object.hash(state);

    }

}

impl<T> Debug for Immut<T>
    where T: Debug
{

    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
    {

        f.debug_struct("Immut").field("object", &self.object).finish()

    }

}

cfg_if::cfg_if!
{

    if #[cfg(feature = "serde")]
    {

        impl<T> Serialize for Immut<T>
            where T: Serialize
        {

            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where S: Serializer
            {

                T::serialize(&self, serializer)

            }

        }

        impl<'de, T> Deserialize<'de> for Immut<T>
            where T: Deserialize<'de>
        {

            fn deserialize<D>(deserialiser: D) -> Result<Self, D::Error>
                where D: Deserializer<'de>
            {

                let res = T::deserialize(deserialiser)?;
                
                Ok(Immut::new(res))

            }

        }

    }

}
