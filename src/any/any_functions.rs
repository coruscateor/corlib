use std::any::{Any, TypeId};

pub fn type_id(object_ref: &dyn Any) -> TypeId
{

    object_ref.type_id()

}

pub fn is<T>(object_ref: &dyn Any) -> bool
    where T: 'static
{

    object_ref.is::<T>()

}

pub fn downcast_ref<T>(object_ref: &dyn Any) -> Option<&T>
    where T: 'static
{

    object_ref.downcast_ref::<T>()

}

pub fn downcast_mut<T>(object_mut: &mut dyn Any) -> Option<&mut T>
    where T: 'static
{

    object_mut.downcast_mut::<T>()

}

pub fn as_any_ref(object_ref: &dyn Any) -> &dyn Any
{

    object_ref

} 

pub fn as_any_mut(object_mut: &mut dyn Any) -> &mut dyn Any
{

    object_mut

} 
