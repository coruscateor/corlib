use std::{any::TypeId, fmt::write, marker::PhantomData, mem::transmute_copy};

use serde::{Deserialize, de::Visitor};

use crate::serde::GenericVisitorExpected;


pub struct GenericVisitor<'de, T> //'de,
    where T: Deserialize<'de>
{

    phantom_data: PhantomData<T>,
    phantom_de: PhantomData<&'de ()>,
    //visit_none_fn: Fn()

}

impl<'de, T> GenericVisitor<'de, T>
    where
        T: Deserialize<'de> + 'static
{

    pub fn new() -> Self
    {

        Self
        {

            phantom_data: PhantomData::default(),
            phantom_de: PhantomData::default()

        }

    }

    fn transmute_copy<T2, E>(self, v: T2) -> Result<T, E>
        where
            E: serde::de::Error,
            T2: 'static
    {

        let type_of_t = TypeId::of::<T>();

        let type_of_t2 = TypeId::of::<T2>();

        if type_of_t == type_of_t2
        {

            unsafe
            {

                Ok(transmute_copy(&v))

            }

        }
        else
        {

            let expected = GenericVisitorExpected::new(type_of_t);

            Err(E::invalid_type(serde::de::Unexpected::Other("Unexpected type provided"), &expected))

        }

    }

    fn transmute_str_ref_copy<E>(self, v: &str) -> Result<T, E>
        where
            E: serde::de::Error,
    {

        let type_of_t = TypeId::of::<T>();

        let type_of_t2 = TypeId::of::<&str>();

        if type_of_t == type_of_t2
        {

            unsafe
            {

                Ok(transmute_copy(&v))

            }

        }
        else
        {

            let expected = GenericVisitorExpected::new(type_of_t);

            Err(E::invalid_type(serde::de::Unexpected::Other("Unexpected type provided"), &expected))

        }

    }

    fn transmute_bytes_ref_copy<E>(self, v: &[u8]) -> Result<T, E>
        where
            E: serde::de::Error,
    {

        let type_of_t = TypeId::of::<T>();

        let type_of_t2 = TypeId::of::<&[u8]>();

        if type_of_t == type_of_t2
        {

            unsafe
            {

                Ok(transmute_copy(&v))

            }

        }
        else
        {

            let expected = GenericVisitorExpected::new(type_of_t);

            Err(E::invalid_type(serde::de::Unexpected::Other("Unexpected type provided"), &expected))

        }

    }
    
    /*
    fn transmute_into_string<T2, E>(self, v: &str) -> Result<T, E>
        where
            E: serde::de::Error,
            T2: 'static
    {



    }
    */

}

impl<'de, T> Visitor<'de> for GenericVisitor<'de, T>
    where
        T: Deserialize<'de> + 'static
{

    type Value = T;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        
        write!(formatter, "Expecting anything")
        
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
    {
        
        self.transmute_copy(v)

    }

    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
    {
        
        self.transmute_copy(v)

    }

    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
    {
        
        self.transmute_copy(v)

    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
    {
        
        self.transmute_copy(v)

    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
    {
        
        self.transmute_copy(v)

    }

    fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
    {
        
        self.transmute_copy(v)

    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
    {
        
        self.transmute_copy(v)

    }

    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
    {
        
        self.transmute_copy(v)

    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
    {
        
        self.transmute_copy(v)

    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
    {
        
        self.transmute_copy(v)

    }

    fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
    {
        
        self.transmute_copy(v)

    }

    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
    {
        
        self.transmute_copy(v)

    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
    {
        
        self.transmute_copy(v)

    }

    fn visit_char<E>(self, v: char) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
    {
        
        self.transmute_copy(v)

    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
    {
        
        self.transmute_str_ref_copy(v)

    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
    {
        
        self.transmute_str_ref_copy(v)

    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
    {
        
        self.transmute_copy(v)

    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
    {
        
        self.transmute_bytes_ref_copy(v)

    }

    fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
    {

        self.transmute_bytes_ref_copy(v)
        
    }

    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        
        self.transmute_copy(v)

    }

    /*
    fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
    {
        
        

    }
    
    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let _ = deserializer;
        Err(serde::de::Error::invalid_type(serde::de::Unexpected::Option, &self))
    }
    */
    
    fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
    {
        
        self.transmute_copy(())

    }
    
    fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: serde::Deserializer<'de>,
    {

        T::deserialize(deserializer)

        //deserializer.deserialize_newtype_struct("", visitor)

    }
    
    fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {

        //T::

        let _ = seq;
        Err(serde::de::Error::invalid_type(serde::de::Unexpected::Seq, &self))
    }
    
    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let _ = map;
        Err(serde::de::Error::invalid_type(serde::de::Unexpected::Map, &self))
    }
    
    fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::EnumAccess<'de>,
    {
        let _ = data;
        Err(serde::de::Error::invalid_type(serde::de::Unexpected::Enum, &self))
    }

}