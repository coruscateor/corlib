use std::{marker::PhantomData, rc::{Rc, Weak}};

use crate::cell::RefCellStore;

use delegate::delegate;

struct MutState<S, T> //, F>
    //where F: FnMut(&S, Rc<T>) //-> bool
{

    parent: Weak<T>,
    //func: F,
    func: Box<dyn FnMut(&S, Rc<T>)>,
    phantom_s: PhantomData<S>

}

impl<S, T> MutState<S, T> //, F>
    //where F: FnMut(&S, Rc<T>) //-> bool
{

    fn new<F>(parent: &Weak<T>, func: F) -> Self
        where F: FnMut(&S, Rc<T>) + 'static
    {

        Self
        {

            parent: parent.clone(),
            func: Box::new(func),
            phantom_s: PhantomData::default()

        }

    }

}

pub struct SingleSubEvent<S, T> //, F>
    //where F: FnMut(&S, Rc<T>) //-> bool
{

    mut_state: RefCellStore<Option<MutState<S, T>>>

}

impl<S, T> SingleSubEvent<S, T> //, F>
    //where F: FnMut(&S, Rc<T>) //-> bool
{

    pub fn new() -> Self
    {

        Self
        {

            mut_state: RefCellStore::new(None)

        }

    }

    pub fn subscribe<F>(&self, parent: &Weak<T>, func: F)
        where F: FnMut(&S, Rc<T>) + 'static
    {

        //let moved_parent = parent;
        
        //let moved_func = func;

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

    pub fn raise(&self, sender: &S) -> bool
    {

        self.mut_state.borrow_mut(|mut ref_mut|
        {

            if let Some(val) = ref_mut.as_mut()
            {

                if let Some(parent) = val.parent.upgrade()
                {

                    (val.func)(sender, parent);

                    /*
                    if !(val.func)(parent)
                    {

                        *ref_mut = None;

                    }
                    */

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

    pub fn get_sub_un_sub<'a>(&'a self) -> SubUnSub<'a, S, T>
    {

        SubUnSub::new(self)

    }

}

pub struct SubUnSub<'a, S, T>
{

    sse: &'a SingleSubEvent<S, T>

}

impl<'a, S, T> SubUnSub<'a, S, T>
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

            pub fn subscribe<F>(&self, parent: &Weak<T>, func: F)
                where F: FnMut(&S, Rc<T>) + 'static;

            pub fn unsubscribe(&self);

            pub fn is_subscribed(&self) -> bool;

        }

    } 

}