use std::marker::PhantomData;
use std::rc::{Rc, Weak};

use std::ops::Fn;

use crate::collections::List;

use crate::weak_by_ptr::WeakByPtr;

use std::vec::Vec;

use delegate::delegate;

use super::base_list_event::{BaseListEvent, LEPubEvent};

//use std::ops::{AddAssign, SubAssign};

//UniqueList

use crate::rc_by_ptr::RcByPtr;

//type FuncType<S, A> = dyn Fn(&S, &A); //mut 

pub type SenderArgEventFunc<S, A> = dyn Fn(&S, &A) -> bool; 

///
/// A UniqueItemList based event containing event handler functions that each take an event-arguments reference in addition to the sender reference argument.
/// 
pub struct ListArgsEvent<S, A>
    //where F: Fn(&mut S, &A)
{

    ble: BaseListEvent<SenderArgEventFunc<S, A>, S>,
    phantom: PhantomData<A>

}

impl<S, A> ListArgsEvent<S, A>
    //where F: Fn(&mut S, &A)
{

    pub fn new() -> Self
    {

        Self
        {

            ble: BaseListEvent::new(),
            phantom: PhantomData::default()

        }

    }

    pub fn with_capacity(capacity: usize) -> Self
    {

        Self
        {


            ble: BaseListEvent::with_capacity(capacity),
            phantom: PhantomData::default()

        }

    }

    delegate! {
        to self.ble { //contents {

            fn subscribe(&mut self, f: &Rc<SenderArgEventFunc<S, A>>) -> bool;

            fn unsubscribe(&mut self, f: &Rc<SenderArgEventFunc<S, A>>) -> bool;

            fn get_pub_event<'a>(&'a mut self) -> LEPubEvent<'a, SenderArgEventFunc<S, A>, S>;

            fn len(&self) -> usize;

            fn is_empty(&self) -> bool;

        }
    }

}

//https://doc.rust-lang.org/error_codes/E0403.html

impl<S, A> ListArgsEvent<S, A> 
    //where F: Fn(&mut S, &A)
{

    pub fn raise(&mut self, sender: &S, event_args: &A) -> usize //mut 
        //where F: Fn(&mut S, &A)
    {

        let mut items_to_remove = Vec::new();

        let mut index: usize = 0;

        for item in self.ble.iter()
        {

            //if !item.get_contents_ref()(sender, event_args)
            if !item.contents()(sender, event_args)
            {

                items_to_remove.push(index);

            }

            index += 1;

        }

        self.ble.remove_at_indexes(items_to_remove)
        
    }

}
