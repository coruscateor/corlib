use std::{marker::PhantomData, rc::{Rc, Weak}};

use crate::cell::RefCellStore;

use delegate::delegate;

struct MutState<S, T> //, F>
    //where F: FnMut(&S, Rc<T>) //-> bool
{

    sender_parent: Weak<T>,
    //func: F,
    func: Box<dyn FnMut(Rc<S>, Rc<T>)>,
    phantom_s: PhantomData<S>

}

impl<S, T> MutState<S, T> //, F>
    //where F: FnMut(&S, Rc<T>) //-> bool
{

    fn new<F>(sender_parent: &Weak<T>, func: F) -> Self
        where F: FnMut(Rc<S>, Rc<T>) + 'static
    {

        Self
        {

            sender_parent: sender_parent.clone(),
            func: Box::new(func),
            phantom_s: PhantomData::default()

        }

    }

}

pub struct SingleSubEvent<S, T> //, F>
    //where F: FnMut(&S, Rc<T>) //-> bool
{

    mut_state: RefCellStore<Option<MutState<S, T>>>,
    weak_sender: Weak<S>

}

impl<S, T> SingleSubEvent<S, T> //, F>
    //where F: FnMut(&S, Rc<T>) //-> bool
{

    pub fn new(weak_sender: &Weak<S>) -> Self
    {

        Self
        {

            mut_state: RefCellStore::new(None),
            weak_sender: weak_sender.clone()

        }

    }

    pub fn subscribe<F>(&self, sender_parent: &Weak<T>, func: F)
        where F: FnMut(Rc<S>, Rc<T>) + 'static
    {

        //let moved_parent = parent;
        
        //let moved_func = func;

        self.mut_state.borrow_mut_with_param((sender_parent, func),|mut ref_mut, (parent, func)|
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

    pub fn raise(&self) -> bool
    {

        self.mut_state.borrow_mut(|mut ref_mut|
        {

            if let Some(val) = ref_mut.as_mut()
            {

                if let Some(sender) = self.weak_sender.upgrade()
                {

                    if let Some(sender_parent) = val.sender_parent.upgrade()
                    {
    
                        (val.func)(sender, sender_parent);
    
                        return true;
    
                    }

                }

            }
                
            false

        })

    }

    pub fn pub_this<'a>(&'a self) -> PubSingleSubEvent<'a, S, T>
    {

        PubSingleSubEvent::new(self)

    }

}

pub struct PubSingleSubEvent<'a, S, T>
{

    sse: &'a SingleSubEvent<S, T>

}

impl<'a, S, T> PubSingleSubEvent<'a, S, T>
{

    pub fn new(sse: &'a SingleSubEvent<S, T>) -> Self
    {

        Self
        {

            sse

        }
    
    }

    delegate!
    {

        to self.sse
        {

            pub fn subscribe<F>(&self, sender_parent: &Weak<T>, func: F)
                where F: FnMut(Rc<S>, Rc<T>) + 'static;

            pub fn unsubscribe(&self);

            pub fn is_subscribed(&self) -> bool;

        }

    } 

}

//Use in the sender impl block.

#[macro_export]
macro_rules! impl_pub_single_sub_event_method
{

    ($field:ident, $sender_parent_type:ty) =>
    {

        pub fn $field<'a>(&'a self) -> PubSingleSubEvent<'a, Self, $sender_parent_type>
        {
    
            self.$field.pub_this()
    
        }

    }

}


