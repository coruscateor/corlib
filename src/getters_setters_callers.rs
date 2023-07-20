//!
//! Convenience macroes for implementing getters, setters and calling stuff.
//! 

use paste::paste;

//traits - declarations

#[macro_export]
macro_rules! trait_get
{

    ($name:ident, $name_type:ty) =>
    {

        paste! {

            fn [<get_ $name>](&self) -> $name_type;

        }

    }

}

#[macro_export]
macro_rules! trait_set
{

    ($name:ident, $name_type:ty) =>
    {

        paste! {

            fn [<set_ $name>](&mut self, value: $name_type);

        }

    }

}

//

#[macro_export]
macro_rules! trait_get_set
{

    ($name:ident, $name_type:ty) =>
    {

        trait_get!($name, $name_type);

        trait_set!($name, $name_type);

    }

}

//

#[macro_export]
macro_rules! trait_get_ref
{

    ($name:ident, $name_type:ty) =>
    {

        paste! {

            fn [<get_ $name _ref>](&self) -> &$name_type;

        }

    }

}

#[macro_export]
macro_rules! trait_get_mut
{

    ($name:ident, $name_type:ty) =>
    {

        paste! {

            fn [<get_ $name _mut>](&mut self) -> &mut $name_type;

        }

    }

}

//

#[macro_export]
macro_rules! trait_get_ref_mut
{

    ($name:ident, $name_type:ty) =>
    {

        trait_get_ref!($name, $name_type);

        trait_get_mut!($name, $name_type);

    }

}

#[macro_export]
macro_rules! trait_get_set_ref_mut
{

    ($name:ident, $name_type:ty) =>
    {

        trait_get!($name, $name_type);

        trait_set!($name, $name_type);

        trait_get_ref!($name, $name_type);

        trait_get_mut!($name, $name_type);

    }

}

//traits - impl

/// Get macro to be used in the impl block of a struct. Assumes name impelments Copy.
/// 
/*
#[macro_export]
macro_rules! trait_impl_get
{

    ($name:ident, $name_type:ty) =>
    {

        paste! {

            fn [<get_ $name>](&self) -> $name_type
            {

                self.$name

            }

        }

    }

}

#[macro_export]
macro_rules! trait_impl_set
{

    ($name:ident, $name_type:ty) =>
    {

        paste! {

            fn [<set_ $name>](&mut self, value: $name_type)
            {

                self.$name = value;

            }

        }

    }

}

//

#[macro_export]
macro_rules! trait_impl_get_set
{

    ($name:ident, $name_type:ty) =>
    {

        trait_impl_get!($name, $name_type);

        trait_impl_set!($name, $name_type);

    }

}
*/

//impl

#[macro_export]
macro_rules! impl_get
{

    ($name:ident, $name_type:ty) =>
    {

        paste!
        {

            pub fn [<get_ $name>](&self) -> $name_type
            {

                self.$name

            }

        }

    }

}

#[macro_export]
macro_rules! impl_set
{

    ($name:ident, $name_type:ty) =>
    {

        paste!
        {

            pub fn [<set_ $name>](&mut self, value: $name_type)
            {

                self.$name = value;

            }

        }

    }

}

//

#[macro_export]
macro_rules! impl_get_set
{

    ($name:ident, $name_type:ty) =>
    {

        impl_get!($name, $name_type);

        impl_set!($name, $name_type);

    }

}

//

#[macro_export]
macro_rules! impl_get_ref
{

    ($name:ident, $name_type:ty) =>
    {

        paste! {

            pub fn [<get_ $name _ref>](&self) -> &$name_type
            {

                &self.$name

            }

        }

    }

}

#[macro_export]
macro_rules! impl_get_mut
{

    ($name:ident, $name_type:ty) =>
    {

        paste! {

            pub fn [<get_ $name _mut>](&mut self) -> &mut $name_type
            {

                &mut self.$name

            }

        }

    }

}

//

#[macro_export]
macro_rules! impl_get_ref_mut
{

    ($name:ident, $name_type:ty) =>
    {

        impl_get_ref!($name, $name_type);

        impl_get_mut!($name, $name_type);

    }

}

#[macro_export]
macro_rules! impl_get_set_ref_mut
{

    ($name:ident, $name_type:ty) =>
    {

        impl_get!($name, $name_type);

        impl_set!($name, $name_type);

        impl_get_ref!($name, $name_type);

        impl_get_mut!($name, $name_type);

    }

}

//Alias refs

#[macro_export]
macro_rules! impl_alias_get_ref
{

    ($field:ident, $alias:ident, $name_type:ty) =>
    {

        paste!
        {

            pub fn [<get_ $alias _ref>](&self) -> &$name_type
            {

                &self.$field
            }

        }

    }

}

#[macro_export]
macro_rules! impl_alias_get_mut
{

    ($field:ident, $alias:ident, $name_type:ty) =>
    {

        paste!
        {

            pub fn [<get_ $alias _mut>](&mut self) -> &mut $name_type
            {

                &mut self.$field

            }

        }

    }

}

#[macro_export]
macro_rules! impl_alias_get_ref_mut
{

    ($field:ident, $alias:ident, $name_type:ty) =>
    {

        impl_alias_get_ref!($field, $alias, $name_type);

        impl_alias_get_mut!($field, $alias, $name_type);

    }

}

//refcell - traits - declaration

//rfc = refcell

/*
#[macro_export]
macro_rules! trait_def_rfc_get_ref
{

    ($name:ident, $name_type:ty) =>
    {

        paste! {

            fn [<get_ $name _ref>](&self) -> Ref<'_, $name_type>;

        }

    }

}

#[macro_export]
macro_rules! trait_def_rfc_get_mut
{

    ($name:ident, $name_type:ty) =>
    {

        paste! {

            fn [<get_ $name _mut>](&self) -> Ref<'_, $name_type>;

        }

    }

}

#[macro_export]
macro_rules! trait_def_rfc_get_ref_mut
{

    ($name:ident, $name_type:ty) =>
    {

        trait_def_rfc_get_ref!($name, $name_type);

        trait_def_rfc_get_mut!($name, $name_type);

    }

}
*/

#[macro_export]
macro_rules! trait_def_rfc_borrow
{

    ($name:ident, $name_type:ty) =>
    {

        paste! {

            fn [<borrow_ $name>](&self) -> Ref<'_, $name_type>;

        }

    }

}

#[macro_export]
macro_rules! trait_def_rfc_borrow_mut
{

    ($name:ident, $name_type:ty) =>
    {

        paste! {

            fn [<borrow_mut_ $name>](&self) -> Ref<'_, $name_type>;

        }

    }

}

#[macro_export]
macro_rules! trait_def_rfc_borrow_and_mut
{

    ($name:ident, $name_type:ty) =>
    {

        trait_def_rfc_borrow!($name, $name_type);

        trait_def_rfc_borrow_mut!($name, $name_type);

    }

}

//refcell

//macros don't seem to be able to call other macros

/*
#[macro_export]
macro_rules! impl_rfc_get_ref
{

    ($name:ident, $name_type:ty) =>
    {

        paste! {

            pub fn [<get_ $name _ref>](&self) -> Ref<'_, $name_type>
            {

                self.$name.borrow()

            }

        }

    }

}

#[macro_export]
macro_rules! impl_rfc_get_mut
{

    ($name:ident, $name_type:ty) =>
    {

        paste! {

            pub fn [<get_ $name _mut>](&self) -> RefMut<'_, $name_type>
            {

                self.$name.borrow_mut()

            }

        }

    }

}

#[macro_export]
macro_rules! impl_rfc_get_ref_mut
{

    ($name:ident, $name_type:ty) =>
    {

        impl_rfc_get_ref!($name, $name_type);

        impl_rfc_get_mut!($name, $name_type);

    }

}
*/

#[macro_export]
macro_rules! impl_rfc_borrow
{

    ($name:ident, $name_type:ty) =>
    {

        paste! {

            pub fn [<borrow_ $name>](&self) -> Ref<'_, $name_type>
            {

                self.$name.borrow()

            }

        }

    }

}

#[macro_export]
macro_rules! impl_rfc_borrow_mut
{

    ($name:ident, $name_type:ty) =>
    {

        paste! {

            pub fn [<borrow_mut_ $name>](&self) -> RefMut<'_, $name_type>
            {

                self.$name.borrow_mut()

            }

        }

    }

}

#[macro_export]
macro_rules! impl_rfc_borrow_and_mut
{

    ($name:ident, $name_type:ty) =>
    {

        impl_rfc_borrow!($name, $name_type);

        impl_rfc_borrow_mut!($name, $name_type);

    }

}

//

/*
#[macro_export]
macro_rules! impl_rfc_get_rfc_field_delegate
{

    ($rfc_field:ident, $field:ident, $field_type:ty) =>
    {

        paste! {

            pub fn [<get_ $field>](&self) -> $field_type
            {

                &self.$rfc_field.borrow().$field

            }

        }

    }

}

#[macro_export]
macro_rules! impl_rfc_deref_get_rfc_field_delegate
{

    ($rfc_field:ident, $field:ident, $field_type:ty) =>
    {

        paste! {

            pub fn [<get_ $field>](&self) -> $field_type
            {

                *&self.$rfc_field.borrow().$field

            }

        }

    }

}

#[macro_export]
macro_rules! impl_rfc_set_rfc_field_delegate
{

    ($rfc_field:ident, $field:ident, $field_type:ty) =>
    {

        paste! {

            pub fn [<set_ $field>](&self, value: $field_type)
            {

                self.$rfc_field.borrow_mut().$field = value;

            }

        }

    }

}

#[macro_export]
macro_rules! impl_rfc_get_set_rfc_field_delegates
{

    ($rfc_field:ident, $field:ident, $field_type:ty) =>
    {

        impl_rfc_get_rfc_field_delegate!($rfc_field, $field, $field_type);

        impl_rfc_set_rfc_field_delegate!($rfc_field, $field, $field_type);

    }

}

#[macro_export]
macro_rules! impl_rfc_deref_get_set_rfc_field_delegate
{

    ($rfc_field:ident, $field:ident, $field_type:ty) =>
    {

        impl_rfc_deref_get_rfc_field_delegate!($rfc_field, $field, $field_type);

        impl_rfc_set_rfc_field_delegate!($rfc_field, $field, $field_type);

    }

}
*/

#[macro_export]
macro_rules! impl_rfc_borrow_get
{

    ($rfc_field:ident, $field:ident, $field_type:ty) =>
    {

        paste! {

            pub fn [<get_ $field>](&self) -> $field_type
            {

                //*(&self.$rfc_field.borrow().$field)

                self.$rfc_field.borrow().$field

            }

        }

    }

}

/*
#[macro_export]
macro_rules! impl_rfc_borrow_get_deref
{

    ($rfc_field:ident, $field:ident, $field_type:ty) =>
    {

        paste! {

            pub fn [<get_ $field>](&self) -> $field_type
            {

                *&self.$rfc_field.borrow().$field

            }

        }

    }

}
*/

#[macro_export]
macro_rules! impl_rfc_borrow_mut_set
{

    ($rfc_field:ident, $field:ident, $field_type:ty) =>
    {

        paste! {

            pub fn [<set_ $field>](&self, value: $field_type)
            {

                //*(self.$rfc_field.borrow_mut().$field) = value;

                self.$rfc_field.borrow_mut().$field = value;

            }

        }

    }

}

#[macro_export]
macro_rules! impl_rfc_borrow_mut_get_set
{

    ($rfc_field:ident, $field:ident, $field_type:ty) =>
    {

        impl_rfc_borrow_get!($rfc_field, $field, $field_type);

        impl_rfc_borrow_mut_set!($rfc_field, $field, $field_type);

    }

}

//corlib::

//corlib::

//Calling

#[macro_export]
macro_rules! impl_rfc_borrow_call
{

    ($field:ident, $method:ident) =>
    {

        pub fn $method(&self)
        {

            self.$field.borrow().$method();

        }

    };
    ($field:ident, $method:ident, $return_type:ty) =>
    {

        pub fn $method(&self) -> $return_type
        {

            self.$field.borrow().$method()

        }

    };
    ($field:ident, $method:ident, $($param_name:ident: param_type:ty,)*) =>
    {

        pub fn $method(&self, $($param_name:ident: param_type:ty,)*)
        {

            self.$field.borrow().$method($($param_name:ident,)*);

        }

    };
    ($field:ident, $method:ident, $return_type:ty, $($param_name:ident: param_type:ty,)*) =>
    {

        pub fn $method(&self, $($param_name:ident: param_type:ty,)*) -> $return_type
        {

            self.$field.borrow().$method($($param_name:ident,)*)

        }

    };

}

#[macro_export]
macro_rules! impl_rfc_borrow_mut_call
{

    ($field:ident, $method:ident) =>
    {

        pub fn $method(&self)
        {

            self.$field.borrow_mut().$method();

        }

    };
    ($field:ident, $method:ident, $return_type:ty) =>
    {

        pub fn $method(&self) -> $return_type
        {

            self.$field.borrow_mut().$method()

        }

    };
    ($field:ident, $method:ident, $($param_name:ident: param_type:ty,)*) =>
    {

        pub fn $method(&self, $($param_name:ident: param_type:ty,)*)
        {

            self.$field.borrow_mut().$method($($param_name:ident,)*);

        }

    };
    ($field:ident, $method:ident, $return_type:ty, $($param_name:ident: param_type:ty,)*) =>
    {

        pub fn $method(&self, $($param_name:ident: param_type:ty,)*) -> $return_type
        {

            self.$field.borrow_mut().$method($($param_name:ident,)*)

        }

    };

}

//Aliased

#[macro_export]
macro_rules! impl_rfc_borrow_aliased_call
{

    ($field:ident, $method:ident, $method_alias:ident) =>
    {

        pub fn $method_alias(&self)
        {

            self.$field.borrow().$method();

        }

    };
    ($field:ident, $method:ident, $method_alias:ident, $return_type:ty) =>
    {

        pub fn $method_alias(&self) -> $return_type
        {

            self.$field.borrow().$method()

        }

    };
    ($field:ident, $method:ident, $method_alias:ident, $($param_name:ident: param_type:ty,)*) =>
    {

        pub fn $method_alias(&self)
        {

            self.$field.borrow().$method($($param_name:ident,)*);

        }

    };
    ($field:ident, $method:ident, $method_alias:ident, $return_type:ty, $($param_name:ident: param_type:ty,)*) =>
    {

        pub fn $method_alias(&self) -> $return_type
        {

            self.$field.borrow().$method($($param_name:ident,)*)

        }

    };

}

#[macro_export]
macro_rules! impl_rfc_borrow_mut_aliased_call
{

    ($field:ident, $method:ident, $method_alias:ident) =>
    {

        pub fn $method_alias(&self)
        {

            self.$field.borrow_mut().$method();

        }

    };
    ($field:ident, $method:ident, $method_alias:ident, $return_type:ty) =>
    {

        pub fn $method_alias(&self) -> $return_type
        {

            self.$field.borrow_mut().$method()

        }

    };
    ($field:ident, $method:ident, $method_alias:ident, $($param_name:ident: param_type:ty,)*) =>
    {

        pub fn $method_alias(&self)
        {

            self.$field.borrow_mut().$method($($param_name:ident,)*);

        }

    };
    ($field:ident, $method:ident, $method_alias:ident, $return_type:ty, $($param_name:ident: param_type:ty,)*) =>
    {

        pub fn $method_alias(&self) -> $return_type
        {

            self.$field.borrow_mut().$method($($param_name:ident,)*)

        }

    };

}

//rfc subscriptions

/*
#[macro_export]
macro_rules! impl_rfc_field_subscribe_delegate
{

    ($rfc_field:ident, $event:ident, $event_delegate_sig:ty) =>
    {

        paste! {

            pub fn [<$event _subscribe>](&self, delegate: &Rc<$event_delegate_sig>) -> bool
            {

                self.$rfc_field.borrow_mut().$event.subscribe(delegate)

            }

        }

    }

}

#[macro_export]
macro_rules! impl_rfc_field_unsubscribe_delegate
{

    ($rfc_field:ident, $event:ident, $event_delegate_sig:ty) =>
    {

        paste! {

            pub fn [<$event _unsubscribe>](&self, delegate: &Rc<$event_delegate_sig>) -> bool
            {

                self.$rfc_field.borrow_mut().$event.unsubscribe(delegate)

            }

        }

    }

}

#[macro_export]
macro_rules! impl_rfc_field_subscription_delegate
{

    ($rfc_field:ident, $event:ident, $event_delegate_sig:ty) =>
    {

        impl_rfc_field_subscribe_delegate!($rfc_field, $event, $event_delegate_sig);

        impl_rfc_field_unsubscribe_delegate!($rfc_field, $event, $event_delegate_sig);

    }

}
*/

#[macro_export]
macro_rules! impl_rfc_borrow_mut_subscribe
{

    ($rfc_field:ident, $event:ident, $event_delegate_sig:ty) =>
    {

        paste! {

            pub fn [<$event _subscribe>](&self, delegate: &Rc<$event_delegate_sig>) -> bool
            {

                self.$rfc_field.borrow_mut().$event.subscribe(delegate)

            }

        }

    }

}

#[macro_export]
macro_rules! impl_rfc_borrow_mut_unsubscribe
{

    ($rfc_field:ident, $event:ident, $event_delegate_sig:ty) =>
    {

        paste! {

            pub fn [<$event _unsubscribe>](&self, delegate: &Rc<$event_delegate_sig>) -> bool
            {

                self.$rfc_field.borrow_mut().$event.unsubscribe(delegate)

            }

        }

    }

}

#[macro_export]
macro_rules! impl_rfc_borrow_mut_subscription
{

    ($rfc_field:ident, $event:ident, $event_delegate_sig:ty) =>
    {

        impl_rfc_borrow_mut_subscribe!($rfc_field, $event, $event_delegate_sig);

        impl_rfc_borrow_mut_unsubscribe!($rfc_field, $event, $event_delegate_sig);

    }

}

//corlib::

//corlib::

///Support the weak_self pattern 

#[macro_export]
macro_rules! impl_rfc_get_weak_self
{

    () =>
    {

        pub fn get_weak_self(&self) -> Weak<Self>
        {
    
            self.weak_self.borrow().get_ref().clone()
    
        }    

    };
    ($weak_self_field:ident) =>
    {

        paste! {

            pub fn [<get_ $weak_self_field>]f(&self) -> Weak<Self>
            {
        
                self.$weak_self_field.borrow().get_ref().clone()
        
            }

        }

    }

}
