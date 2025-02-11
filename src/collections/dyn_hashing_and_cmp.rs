use std::{any::Any, hash::{Hash, Hasher}};

//Hash

///
/// For all your dynamic hashing needs 
///
/// Inspired by: <https://users.rust-lang.org/t/workaround-for-hash-trait-not-being-object-safe/53332/4>
///
pub trait DynHash
{

    fn dyn_hash(&self, state: &mut dyn Hasher);

}

///
/// PartialEq or Eq (May be renamed to DynPartialEq)
///
pub trait DynPartialEqOrEq
{

    fn dyn_eq(&self, other: &dyn Any) -> bool;

}

///
/// A Struct for adapting objects that implement DynHash and DynPartialEqOrEq to the standard Hash and PartialEq traits.
/// 
pub struct DynHashAdapter<T>
    where T: DynHash + DynPartialEqOrEq
{

    dyn_hashable_object: T

}

impl<T> DynHashAdapter<T>
    where T: DynHash + DynPartialEqOrEq
{

    pub fn new(dyn_hashable_object: T) -> Self
    {

        Self
        {

            dyn_hashable_object

        }

    }

    pub fn object(&self) -> &T
    {

        &self.dyn_hashable_object

    }

    pub fn object_mut(&mut self) -> &mut T
    {

        &mut self.dyn_hashable_object

    }

    pub fn take(self) -> T
    {

        self.dyn_hashable_object

    }

}

impl<T> Hash for DynHashAdapter<T>
    where T: DynHash + DynPartialEqOrEq
{

    fn hash<H: Hasher>(&self, state: &mut H)
    {

        self.dyn_hashable_object.dyn_hash(state);

    }

}

impl<T> PartialEq for DynHashAdapter<T>
    where T: DynHash + DynPartialEqOrEq + 'static
{
    
    fn eq(&self, other: &Self) -> bool
    {
        
        self.dyn_hashable_object.dyn_eq(other.object())

    }
    
    /*
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
    */

}

/*
impl<T> Eq for DynHashAdapter<T>
    where T: DynHash + DynPartialEqOrEq + 'static
{

}
*/

//impl macro

///
/// An adapter struct for use where only a PartialEq is needed (may be renamed to DynPartialEqAdapter).
/// 
pub struct DynPartialEqOrEqAdapter<T>
    where T: DynPartialEqOrEq
{

    dyn_partial_eq_or_eq_object: T

}

impl<T> DynPartialEqOrEqAdapter<T>
    where T: DynPartialEqOrEq
{

    pub fn new(dyn_partial_eq_or_eq_object: T) -> Self
    {

        Self
        {

            dyn_partial_eq_or_eq_object

        }

    }

    pub fn object(&self) -> &T
    {

        &self.dyn_partial_eq_or_eq_object

    }

    pub fn object_mut(&mut self) -> &mut T
    {

        &mut self.dyn_partial_eq_or_eq_object

    }

    pub fn take(self) -> T
    {

        self.dyn_partial_eq_or_eq_object

    }

}

impl<T> PartialEq for DynPartialEqOrEqAdapter<T>
    where T: DynPartialEqOrEq + 'static
{
    
    fn eq(&self, other: &Self) -> bool
    {
        
        self.dyn_partial_eq_or_eq_object.dyn_eq(other.object())

    }
    
    /*
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
    */

}

//Combined

/*
pub struct DynHashPartialEqOrEqAdapter<T>
    where T: DynHash + DynPartialEqOrEq
{

    dyn_hashable_object: T

}

impl<T> DynHashPartialEqOrEqAdapter<T>
    where T: DynHash + DynPartialEqOrEq
{

    pub fn new(dyn_hashable_object: T) -> Self
    {

        Self
        {

            dyn_hashable_object

        }

    }

    pub fn object(&self) -> &T
    {

        &self.dyn_hashable_object

    }

    pub fn object_mut(&mut self) -> &mut T
    {

        &mut self.dyn_hashable_object

    }

    pub fn take(self) -> T
    {

        self.dyn_hashable_object

    }
    
}
*/

