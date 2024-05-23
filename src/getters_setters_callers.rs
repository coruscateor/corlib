//!
//! Convenience macroes for implementing getters, setters and calling stuff.
//! 

//use paste::paste;

//traits - declarations

#[deprecated(since = "0.2.0")]
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

#[deprecated(since = "0.2.0")]
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

#[deprecated(since = "0.2.0")]
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

#[deprecated(since = "0.2.0")]
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

#[deprecated(since = "0.2.0")]
#[macro_export]
macro_rules! trait_get_ref_mut
{

    ($name:ident, $name_type:ty) =>
    {

        trait_get_ref!($name, $name_type);

        trait_get_mut!($name, $name_type);

    }

}

#[deprecated(since = "0.2.0")]
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

#[deprecated(since = "0.2.0")]
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

#[deprecated(since = "0.2.0")]
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

#[deprecated(since = "0.2.0")]
#[macro_export]
macro_rules! impl_get_set
{

    ($name:ident, $name_type:ty) =>
    {

        impl_get!($name, $name_type);

        impl_set!($name, $name_type);

    }

}

#[deprecated(since = "0.2.0")]
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

#[deprecated(since = "0.2.0")]
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

#[deprecated(since = "0.2.0")]
#[macro_export]
macro_rules! impl_get_ref_mut
{

    ($name:ident, $name_type:ty) =>
    {

        impl_get_ref!($name, $name_type);

        impl_get_mut!($name, $name_type);

    }

}

#[deprecated(since = "0.2.0")]
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

#[deprecated(since = "0.2.0")]
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

#[deprecated(since = "0.2.0")]
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

#[deprecated(since = "0.2.0")]
#[macro_export]
macro_rules! impl_alias_get_ref_mut
{

    ($field:ident, $alias:ident, $name_type:ty) =>
    {

        impl_alias_get_ref!($field, $alias, $name_type);

        impl_alias_get_mut!($field, $alias, $name_type);

    }

}

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

#[deprecated(since = "0.2.0")]
#[macro_export]
macro_rules! impl_rfc_borrow_get
{

    ($rfc_field:ident, $field:ident, $field_type:ty) =>
    {

        paste! {

            pub fn [<get_ $field>](&self) -> $field_type
            {

                self.$rfc_field.borrow().$field

            }

        }

    }

}

#[deprecated(since = "0.2.0")]
#[macro_export]
macro_rules! impl_rfc_borrow_mut_set
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

#[deprecated(since = "0.2.0")]
#[macro_export]
macro_rules! impl_rfc_borrow_mut_get_set
{

    ($rfc_field:ident, $field:ident, $field_type:ty) =>
    {

        impl_rfc_borrow_get!($rfc_field, $field, $field_type);

        impl_rfc_borrow_mut_set!($rfc_field, $field, $field_type);

    }

}

//Calling

///
/// Calls borrow on a RefCelled field and calls a method on the returned Ref.
/// Don't try to return a reference though (It wont work).
/// 
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

        pub fn $method(&self, $($param_name: $param_type,)*)
        {

            self.$field.borrow().$method($($param_name,)*);

        }

    };
    ($field:ident, $method:ident, $return_type:ty, $($param_name:ident: param_type:ty,)*) =>
    {

        pub fn $method(&self, $($param_name: param_type,)*) -> $return_type
        {

            self.$field.borrow().$method($($param_name,)*)

        }

    };

}

/// Calls borrow_mut on a RefCelled field and calls a method on the returned RefMut.
/// Don't try to return a reference though (It wont work).
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

        pub fn $method(&self, $($param_name: param_type,)*)
        {

            self.$field.borrow_mut().$method($($param_name,)*);

        }

    };
    ($field:ident, $method:ident, $return_type:ty, $($param_name:ident: param_type:ty,)*) =>
    {

        pub fn $method(&self, $($param_name:ident: param_type:ty,)*) -> $return_type
        {

            self.$field.borrow_mut().$method($($param_name,)*)

        }

    };

}

//Aliased

/// impl_rfc_borrow_call with an an aliased method name.
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

/// impl_rfc_borrow_mut_call with an an aliased method name.
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

///Subscribe to an event through a RefCell.
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

///Unsubscribe to an event through a RefCell.
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

///Subscribe and unsubscribe to an event through a RefCell.
#[macro_export]
macro_rules! impl_rfc_borrow_mut_subscription
{

    ($rfc_field:ident, $event:ident, $event_delegate_sig:ty) =>
    {

        impl_rfc_borrow_mut_subscribe!($rfc_field, $event, $event_delegate_sig);

        impl_rfc_borrow_mut_unsubscribe!($rfc_field, $event, $event_delegate_sig);

    }

}

///Support the weak_self pattern 

#[deprecated(since = "0.2.0")]
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
