use std::fmt::Display;

#[cfg(feature="serde")]
use std::fmt;

use std::{fmt::Debug, ops::Deref};

#[cfg(feature="serde")]
use serde::Deserializer;

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize, Serializer};

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

impl<T> Display for Immut<T>
    where T: Display
{

    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
    {

        write!(f, "{}", self.object)

    }
    
}

impl<T> Debug for Immut<T>
    where T: Debug
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
