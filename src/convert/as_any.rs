//!
//! The AsAny trait and impl_as_any macro, useful for when you want to convert a reference to &dyn Any.
//! 

use std::any::Any;

///
/// A trait with a method for returning a &dyn Any instance (presumably of &self).
/// 
pub trait AsAnyRef
{

    fn as_any_ref(&self) -> &dyn Any;

}

///
/// A macro for implementing the AsAnyRef trait.
/// 
/// Both AsAnyRef implementations in both match cases return &self.
/// 
#[macro_export]
macro_rules! impl_as_any_ref
{

    ($for_type:ty) =>
    {

        impl AsAnyRef for $for_type
        {

            fn as_any_ref(&self) -> &dyn Any
            {

                self
                
            }

        }

    };
    /*
    ($for_type:ty, $($generic_param:ty),+) =>
    {

        impl<$($generic_param),+> AsAnyRef for $for_type<$($generic_param),+>
        {

            fn as_any_ref(&self) -> &dyn Any
            {

                self
                
            }

        }

    }
    */
}

#[macro_export]
macro_rules! impl_as_any_ref_method
{

    () =>
    {

        fn as_any_ref(&self) -> &dyn Any
        {

            self
            
        }

    }

}

///
/// Like AsAnyRef but the reference returned is mutable.
/// 
pub trait AsAnyMut
{

    fn as_any_mut(&self) -> &mut dyn Any;

}

#[macro_export]
macro_rules! impl_as_any_mut
{

    ($for_type:ty) =>
    {

        impl AsAnyMut for $for_type
        {

            fn as_any_mut(&self) -> &dyn Any
            {

                self
                
            }

        }

    };

}

#[macro_export]
macro_rules! impl_as_any_mut_method
{

    () =>
    {

        fn as_any_mut(&self) -> &mut dyn Any
        {

            self
            
        }

    }

}
