use std::any::Any;

/// A trait with a method (as_any) for returning a &dyn Any instance (presumably of &self). 
pub trait AsAny
{

    fn as_any(&self) -> &dyn Any;

}

/// A macro for implementing the AsAny trait.
/// 
/// Both AsAny implementations in both match cases return &self.
#[macro_export]
macro_rules! impl_as_any
{

    ($for_type:ty) =>
    {

        impl AsAny for $for_type
        {

            fn as_any(&self) -> &dyn Any
            {

                self
                
            }

        }

    };
    ($for_type:ty, $generic_param:ty) =>
    {

        impl<$generic_param> AsAny for $for_type<$generic_param>
        {

            fn as_any(&self) -> &dyn Any
            {

                self
                
            }

        }

    }

}


