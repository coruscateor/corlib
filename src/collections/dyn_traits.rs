use std::{any::Any, hash::Hasher};

///
///For all your dynamic hashing needs 
///
///Inspired by: https://users.rust-lang.org/t/workaround-for-hash-trait-not-being-object-safe/53332/4
///
pub trait DynHash
{

    fn dyn_hash(&self, state: &mut dyn Hasher);

}

//hash slice?

/*
pub trait DynPartialEqOrEq
{

    fn dyn_eq(&self, other: &dyn Any);

}
*/
