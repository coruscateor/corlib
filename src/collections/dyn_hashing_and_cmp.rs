//!
//! For when you need to dynamically hash stuff. 
//! 

use std::{any::Any, hash::{Hash, Hasher}};

//Hash

///
/// For when you need to dynamically hash objects. 
///
/// Inspired by: <https://users.rust-lang.org/t/workaround-for-hash-trait-not-being-object-safe/53332/4>
///
pub trait DynHash
{

    fn dyn_hash(&self, state: &mut dyn Hasher);

}

///
/// PartialEq but dynamic
///
pub trait DynPartialEq
{

    fn dyn_eq(&self, other: &dyn Any) -> bool;

}

///
/// A struct for adapting objects that implement DynHash and DynPartialEq to work with the standard Hash and PartialEq traits.
/// 
pub struct DynHashAdapter<T>
    where T: DynHash + DynPartialEq
{

    dyn_hashable_object: T

}

impl<T> DynHashAdapter<T>
    where T: DynHash + DynPartialEq
{

    pub fn new(dyn_hashable_object: T) -> Self
    {

        Self
        {

            dyn_hashable_object

        }

    }

    pub fn object_ref(&self) -> &T
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
    where T: DynHash + DynPartialEq
{

    fn hash<H: Hasher>(&self, state: &mut H)
    {

        self.dyn_hashable_object.dyn_hash(state);

    }

}

impl<T> PartialEq for DynHashAdapter<T>
    where T: DynHash + DynPartialEq + 'static
{
    
    fn eq(&self, other: &Self) -> bool
    {
        
        self.dyn_hashable_object.dyn_eq(other.object_ref())

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
/// An adapter struct for use where only a PartialEq is needed.
/// 
pub struct DynPartialEqAdapter<T>
    where T: DynPartialEq
{

    dyn_partial_eq_object: T

}

impl<T> DynPartialEqAdapter<T>
    where T: DynPartialEq
{

    pub fn new(dyn_partial_eq_object: T) -> Self
    {

        Self
        {

            dyn_partial_eq_object

        }

    }

    pub fn object_ref(&self) -> &T
    {

        &self.dyn_partial_eq_object

    }

    pub fn object_mut(&mut self) -> &mut T
    {

        &mut self.dyn_partial_eq_object

    }

    pub fn take(self) -> T
    {

        self.dyn_partial_eq_object

    }

}

impl<T> PartialEq for DynPartialEqAdapter<T>
    where T: DynPartialEq + 'static
{
    
    fn eq(&self, other: &Self) -> bool
    {
        
        self.dyn_partial_eq_object.dyn_eq(other.object_ref())

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

