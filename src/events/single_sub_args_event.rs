use std::{marker::PhantomData, rc::{Rc, Weak}};

use crate::cell::RefCellStore;

use delegate::delegate;

struct MutState<S, A, T> //, F>
    //where F: FnMut(&S, &A, Rc<T>)
{

    parent: Weak<T>,
    //func: F,
    func: Box<dyn FnMut(&S, &A, Rc<T>)>,
    phantom_s: PhantomData<S>,
    phantom_a: PhantomData<A>

}

impl<S, A, T> MutState<S, A, T> //, F>
    //where F: FnMut(&S, &A, Rc<T>)
{

    fn new<F>(parent: &Weak<T>, func: F) -> Self
        where F: FnMut(&S, &A, Rc<T>) + 'static
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

    mut_state: RefCellStore<Option<MutState<S, A, T>>>

}

impl<S, A, T> SingleSubArgsEvent<S, A, T> //, F>
    //where F: FnMut(&S, &A, Rc<T>) //-> bool
{

    pub fn new() -> Self
    {

        Self
        {

            mut_state: RefCellStore::new(None)

        }

    }

    pub fn subscribe<F>(&self, parent: &Weak<T>, func: F)
        where F: FnMut(&S, &A, Rc<T>) + 'static
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

    pub fn raise(&self, sender: &S, event_args: &A) -> bool
    {

        self.mut_state.borrow_mut(|mut ref_mut|
        {

            if let Some(val) = ref_mut.as_mut()
            {

                if let Some(parent) = val.parent.upgrade()
                {

                    (val.func)(sender, event_args, parent);

                    true

                }
                else
                {
                    
                    false

                }

            }
            else
            {
                
                false

            }

        })

    }

    pub fn get_sub_un_sub_args<'a>(&'a self) -> SubUnSubArgs<'a, S, A, T>
    {

        SubUnSubArgs::new(self)

    }

}

pub struct SubUnSubArgs<'a, S, A, T>
{

    ssae: &'a SingleSubArgsEvent<S, A, T>

}

impl<'a, S, A, T> SubUnSubArgs<'a, S, A, T>
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

            pub fn subscribe<F>(&self, parent: &Weak<T>, func: F)
                where F: FnMut(&S, &A, Rc<T>) + 'static;

            pub fn unsubscribe(&self);

            pub fn is_subscribed(&self) -> bool;

        }

    } 

}
