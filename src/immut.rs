//#[cfg(all(feature="serde",not(any())))]
//use std::fmt;
use std::{fmt::Debug, ops::Deref};

//#[cfg(all(feature="serde",not(any())))]
//use serde::Deserializer;
//use serde::{Serializer, ser::SerializeStruct};

//use serde::de::Visitor;
#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize}; //, Serializer, ser::SerializeStruct, Deserializer};

///
/// This object makes its contained object externally immutable only.
/// 
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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

/*
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

                let mut state = serializer.serialize_struct("Immut", 1)?;

                state.serialize_field("item", &self.item)?;

                state.end()

            }

        }

        struct ImmutVisitor<T>
        {

            //item: T
            phantom: PhantomData<T>

        }

        impl<T> ImmutVisitor<T> //,T //<'de>
        {

            pub fn new()
            {
            }

        }

        impl<'de, T> Visitor<'de> for ImmutVisitor<T> //<'de>
        {

            type Value = Immut<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result
            {

                formatter.write_str("error")

            }

        }

        impl<'de> Visitor<'de> for ImmutVisitor<bool>
        {

            type Value = Immut<bool>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result
            {

                formatter.write_str("error")

            }

        }

        impl<'de, T> Deserialize<'de> for Immut<T>
            where T: Deserialize<'de>
        {

            fn deserialize<D>(deserialiser: D) -> Result<Self, D::Error>
                where D: Deserializer<'de>
            {

                let vis = ImmutVisitor::<T>::new();

                let res = deserialiser.deserialize_struct("Immut", &["item"], vis);

                res

                //T::

                //let item = T::deserialize(deserialiser)?;

                //Ok(Self::new(item))

                //deserialiser.deserialize_any(T::)

            }

        }

    }

}
*/
