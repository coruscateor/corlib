use std::any::Any;

///
/// A trait with a method for returning a &dyn Any instance (of &self presumably).
/// 
pub trait AsAnyRef
{

    fn as_any_ref(&self) -> &dyn Any;

}

///
/// Like AsAnyRef but the reference returned is mutable.
/// 
pub trait AsAnyMut : AsAnyRef
{

    fn as_any_mut(&mut self) -> &mut dyn Any;

}
