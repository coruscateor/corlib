use std::{marker::PhantomData, rc::{Rc, Weak}};

use crate::cell::RefCellStore;

use delegate::delegate;

struct MutState<S, A, T> //, F>
    //where F: FnMut(&S, &A, Rc<T>)
{

    parent: Weak<T>,
    //func: F,
    func: Box<dyn FnMut(Rc<S>, &A, Rc<T>)>,
    phantom_s: PhantomData<S>,
    phantom_a: PhantomData<A>

}

impl<S, A, T> MutState<S, A, T> //, F>
    //where F: FnMut(&S, &A, Rc<T>)
{

    fn new<F>(parent: &Weak<T>, func: F) -> Self
        where F: FnMut(Rc<S>, &A, Rc<T>) + 'static
    {

        Self
        {

            parent: parent.clone(),
            func: Box::new(func),
            phantom_s: PhantomData::default(),
            phantom_a: PhantomData::default()

        }

    }

}

pub struct SingleSubArgsEvent<S, A, T> //, F>
    //where F: FnMut(&S, &A, Rc<T>) //-> bool
{

    mut_state: RefCellStore<Option<MutState<S, A, T>>>,
    weak_sender: Weak<S>

}

impl<S, A, T> SingleSubArgsEvent<S, A, T> //, F>
    //where F: FnMut(&S, &A, Rc<T>) //-> bool
{

    pub fn new(weak_sender: &Weak<S>) -> Self
    {

        Self
        {

            mut_state: RefCellStore::new(None),
            weak_sender: weak_sender.clone()

        }

    }

    pub fn subscribe<F>(&self, parent: &Weak<T>, func: F)
        where F: FnMut(Rc<S>, &A, Rc<T>) + 'static
    {

        self.mut_state.borrow_mut_with_param((parent, func),|mut ref_mut, (parent, func)|
        {

            *ref_mut = Some(MutState::new(parent, func));

        });

    }

    pub fn unsubscribe(&self)
    {

        self.mut_state.borrow_mut(|mut ref_mut|
        {

            *ref_mut = None;

        });

    }

    pub fn is_subscribed(&self) -> bool
    {

        self.mut_state.borrow(|ref_ref|
        {

            ref_ref.is_some()

        })
        
    }

    pub fn raise(&self, event_args: &A) -> bool
    {

        self.mut_state.borrow_mut(|mut ref_mut|
        {

            if let Some(val) = ref_mut.as_mut()
            {

                if let Some(sender) = self.weak_sender.upgrade()
                {

                    if let Some(parent) = val.parent.upgrade()
                    {

                        (val.func)(sender, event_args, parent);

                        return true;

                    }

                }

            }
               
            false

        })

    }

    pub fn pub_this<'a>(&'a self) -> PubSingleSubArgsEvent<'a, S, A, T>
    {

        PubSingleSubArgsEvent::new(self)

    }

}

pub struct PubSingleSubArgsEvent<'a, S, A, T>
{

    ssae: &'a SingleSubArgsEvent<S, A, T>

}

impl<'a, S, A, T> PubSingleSubArgsEvent<'a, S, A, T>
{

    pub fn new(ssae: &'a SingleSubArgsEvent<S, A, T>) -> Self
    {

        Self
        {

            ssae

        }
    
    }
    
    delegate!
    {

        to self.ssae
        {

            pub fn subscribe<F>(&self, sender_parent: &Weak<T>, func: F)
                where F: FnMut(Rc<S>, &A, Rc<T>) + 'static;

            pub fn unsubscribe(&self);

            pub fn is_subscribed(&self) -> bool;

        }

    } 

}

//Use in the sender impl block.

#[macro_export]
macro_rules! impl_pub_single_sub_args_event_method
{

    ($field:ident, $event_args_type:ty, $sender_parent_type:ty) =>
    {

        pub fn $field<'a>(&'a self) -> PubSingleSubArgsEvent<'a, Self, $event_args_type, $sender_parent_type>
        {
    
            self.$field.pub_this()
    
        }

    }

}
