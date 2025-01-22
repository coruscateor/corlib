//!
//! For when you want to do stuff with values and generic type constraints.
//! 

pub trait HasValueGetter<T>
{

    fn value(&self) -> T;

}

pub trait HasOptionalValueGetter<T>
{

    fn value(&self) -> Option<T>;

}

pub trait HasValueSetter<T>
{

    fn set_value(&self, value: T);

}

pub trait HasValueSetReturn<T, R>
{

    fn set_value(&self, value: T) -> R;

}

pub trait HasValueSetterMut<T>
{

    fn set_value(&mut self, value: T);

}

pub trait HasValueSetReturnMut<T, R>
{

    fn set_value(&mut self, value: T) -> R;

}

pub trait HasValuePassRef<T>
{

    fn pass_value_ref(&self, value: &T);

}

pub trait HasMutValuePassRef<T>
{

    fn pass_value_ref(&mut self, value: &T);

}

pub trait HasValuePassRefReturn<T, R>
{

    fn pass_value_ref(&self, value: &T) -> R;

}

pub trait HasMutValuePassRefReturn<T, R>
{

    fn pass_value_ref(&mut self, value: &T) -> R;

}

pub trait HasValueRef<T>
{

    fn value_ref(&self) -> &T;

}

pub trait HasValueMut<T>
{

    fn value_ref(&mut self) -> &mut T;

}

/*
#[macro_export]
macro_rules! impl_value_getter_clone_field
{

    ($implementor:ident, $name_type:ty) =>
    {

        impl<T> ValueGetter<T> for $implementor
        {

            fn value(&self) -> T
            {



            }

        }

    }

}
*/