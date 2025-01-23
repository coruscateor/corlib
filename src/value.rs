//!
//! For when you want to do stuff with values and generic type constraints.
//! 

pub trait HasValueGetter
{

    type HasValueType;

    fn value(&self) -> Self::HasValueType;

}

pub trait HasOptionalValueGetter
{

    type HasValueType;

    fn value(&self) -> Option<Self::HasValueType>;

}

pub trait HasValueSetter
{

    type HasValueType;

    fn set_value(&self, value: Self::HasValueType);

}

pub trait HasValueSetReturn
{

    type HasValueType;

    type HasValueReturnType;

    fn set_value(&self, value: Self::HasValueType) -> Self::HasValueReturnType;

}

pub trait HasValueSetterMut
{

    type HasValueType;

    fn set_value(&mut self, value: Self::HasValueType);

}

pub trait HasValueSetReturnMut
{

    type HasValueType;

    type HasValueReturnType;

    fn set_value(&mut self, value: Self::HasValueType) -> Self::HasValueReturnType;

}

pub trait HasValuePassRef
{

    type HasValueType;

    fn pass_value_ref(&self, value: &Self::HasValueType);

}

pub trait HasMutValuePassRef
{

    type HasValueType;

    fn pass_value_ref(&mut self, value: &Self::HasValueType);

}

pub trait HasValuePassRefReturn
{

    type HasValueType;

    type HasValueReturnType;

    fn pass_value_ref(&self, value: &Self::HasValueType) -> Self::HasValueReturnType;

}

pub trait HasMutValuePassRefReturn
{

    type HasValueType;

    type HasValueReturnType;

    fn pass_value_ref(&mut self, value: &Self::HasValueType) -> Self::HasValueReturnType;

}

pub trait HasValueRef
{

    type HasValueType;

    fn value_ref(&self) -> &Self::HasValueType;

}

pub trait HasValueMut
{

    type HasValueType;

    fn value_ref(&mut self) -> &mut Self::HasValueType;

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