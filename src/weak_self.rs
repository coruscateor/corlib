use std::rc::Weak;


pub trait WeakSelf
{

    fn weak_self(&self) -> Weak<Self>;

    fn weak_self_ref(&self) -> &Weak<Self>;

}

#[macro_export]
macro_rules! impl_weak_self_trait
{

    ($object_type:ty) =>
    {

        impl WeakSelf for $object_type
        {

            fn weak_self(&self) -> Weak<Self>
            {

                self.weak_self.clone()
                
            }

            fn weak_self_ref(&self) -> &Weak<Self>
            {

                &self.weak_self
                
            }

        }

    }

}


