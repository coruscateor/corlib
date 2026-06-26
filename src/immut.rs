use std::fmt::Display;
//#[cfg(all(feature="serde",not(any())))]
//use std::marker::PhantomData;

#[cfg(feature="serde")]
use std::fmt;

use std::{fmt::Debug, ops::Deref};

//#[cfg(all(feature="serde",not(any())))]
//use serde::Deserializer;
//use serde::{Serializer, ser::SerializeStruct};

#[cfg(feature="serde")]
use serde::Deserializer;
//#[cfg(feature = "serde")]
//use serde::de::Visitor;

//#[cfg(all(feature="serde",not(any())))]
//use serde::ser::SerializeStruct;
#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize, Serializer}; //, Serializer, ser::SerializeStruct, Deserializer};

///
/// This object makes its contained object externally immutable only.
/// 
/// #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))] //Disabled
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

impl<T> Display for Immut<T>
    where T: Display
{

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
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

                /*
                let mut state = serializer.serialize_struct("Immut", 1)?;

                state.serialize_field("item", &self.object)?;

                state.end()
                */

                T::serialize(&self, serializer)


            }

        }

        /*
        struct ImmutVisitor<T>
        {

            //item: T
            phantom: PhantomData<T>

        }

        impl<T> ImmutVisitor<T> //,T //<'de>
        {

            pub fn new() -> Self
            {

                Self
                {

                    phantom: PhantomData::default()

                }

            }

        }

        impl<'de, T> Visitor<'de> for ImmutVisitor<T>
        {

            type Value = Immut<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result
            {

                formatter.write_str("error")

            }

        }
        */

        /*
        impl<'de, T> Visitor<'de> for ImmutVisitor<T> //<'de>
            where T: From<bool>
        {

            type Value = Immut<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result
            {

                formatter.write_str("error")

            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
                //T: From<bool>
            {
                
                Ok(Immut::new(v.into()))

            }

        }
        */

        /*
        impl<'de, T> Visitor<'de> for ImmutVisitor<T>
            where  T: From<bool>
        {

        }
        */

        /*
        impl<'de> Visitor<'de> for ImmutVisitor<bool>
            //where  T: From<bool>
        {

            type Value = Immut<bool>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result
            {

                formatter.write_str("error")

            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                
                Ok(Immut::new(v))

            }

        }
        */

        /*
        struct ImmutVisitor<T>
        {

            phantom: PhantomData<T>

        }

        impl<T> ImmutVisitor<T>
        {

            pub fn new() -> Self
            {

                Self
                {

                    phantom: PhantomData::default()

                }

            }

        }

        impl<'de, T> Visitor<'de> for ImmutVisitor<T>
        {

            type Value = Immut<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result
            {

                formatter.write_str("error")

            }

            fn visit_struct

            /*
            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                
            }
            */

        }
        */

        impl<'de, T> Deserialize<'de> for Immut<T>
            where T: Deserialize<'de>
        {

            fn deserialize<D>(deserialiser: D) -> Result<Self, D::Error>
                where D: Deserializer<'de>
            {

                /*
                let visitor = ImmutVisitor::<T>::new();

                let type_of_t = TypeId::of::<T>();

                let type_of_bool = TypeId::of::<bool>();

                match type_of_t
                {

                    type_of_bool =>
                    {

                        deserialiser.deserialize_bool(visitor)

                    }

                    
                }
                */

                let res = T::deserialize(deserialiser)?;
                
                Ok(Immut::new(res))

                /*
                let vis = ImmutVisitor::<T>::new();

                let res = deserialiser.deserialize_struct("Immut", &["item"], vis);

                res
                */

                //T::

                //let item = T::deserialize(deserialiser)?;

                //Ok(Self::new(item))

                //deserialiser.deserialize_any(T::)

            }

        }

    }

}
