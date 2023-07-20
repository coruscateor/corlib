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

//type FuncType<S> = dyn Fn(&S) -> bool; //mut 

pub type SenderEventFunc<S> = dyn Fn(&S) -> bool; 

///
/// A UniqueItemList based event containing event handler functions that takes an event-arguments reference in addition to the sender reference argument,
/// 
pub struct ListEvent<S>
{

    //contents: List<RcEq<F>>,
    //phantom: PhantomData<S>

    ble: BaseListEvent<SenderEventFunc<S>, S>

}

impl<S> ListEvent<S>
    //where F: Fn(&mut S)
{

    pub fn new() -> Self
    {

        Self
        {

            ble: BaseListEvent::new()

        }

    }

    pub fn with_capacity(capacity: usize) -> Self
    {

        Self
        {

            ble: BaseListEvent::with_capacity(capacity)

        }

    }

    pub fn raise(&mut self, sender: &S) -> usize //_ref
    {

        let mut items_to_remove = Vec::new();

        let mut index: usize = 0;

        for item in self.ble.iter()
        {

            if !item.get_contents_ref()(sender)
            {

                items_to_remove.push(index);

            }

            index += 1;

        }

        self.ble.remove_at_indexes(items_to_remove)

    }

    delegate! {
        to self.ble { //contents {

            pub fn subscribe(&mut self, f: &Rc<SenderEventFunc<S>>) -> bool;

            pub fn unsubscribe(&mut self, f: &Rc<SenderEventFunc<S>>) -> bool;

            pub fn get_pub_event<'a>(&'a mut self) -> LEPubEvent<'a, SenderEventFunc<S>, S>; //<'a>(&'a mut self, )  //&Rc<dyn Fn(&mut S)>


            pub fn len(&self) -> usize;

            pub fn is_empty(&self) -> bool;

        }
    }

}

//https://doc.rust-lang.org/error_codes/E0403.html

/*
impl<S> ListEvent<S> 
    //where F: Fn(&mut S)
{

    /*
    pub fn raise_mut(&mut self, sender: &mut S)
        //where F: Fn(&mut S)
    {

        for item in self.ble.iter()
        {

            item.get_contents_ref()(sender); 

        }

    }
    */
    
    pub fn raise(&mut self, sender: &S) -> usize //_ref
    {

        let mut items_to_remove = Vec::new();

        for item in self.ble.iter()
        {

            let mut index: usize = 0;

            if !item.get_contents_ref()(sender)
            {

                items_to_remove.push(index);

            }

            index += 1;

        }

        self.ble.remove_at_indexes(items_to_remove)

    }

}

/*
pub struct ListEvent<F, S>
    //where F: PartialEq
    where F: Fn(&mut S)
{

    //contents: List<RcEq<F>>,
    //phantom: PhantomData<S>

    ble: BaseListEvent<dyn F, S>

}

impl<F, S> ListEvent<F, S>
    where F: Fn(&mut S) //PartialEq
{

    pub fn new() -> Self
    {

        Self
        {

            //contents: List::new(),
            //phantom: PhantomData::default()

            ble: BaseListEvent::new()

        }

    }

    pub fn with_capacity(capacity: usize) -> Self
    {

        Self
        {

            //contents: List::with_capacity(capacity),
            //phantom: PhantomData::default()

            ble: BaseListEvent::with_capacity(capacity)

        }

    }

    /*
    pub fn subscribe(&mut self, f: &Rc<F>)
    {

        self.contents.add(RcEq::new(f.clone()));

    }

    pub fn unsubscribe(&mut self, f: &Rc<F>)
    {

        self.contents.remove(RcEq::new(f.clone()));

    }

    /*
    pub fn subscribe_rc(&mut self, key: &Rc<K>, f: F)
    {

        self.contents.add_or_repace(WeakPartialEq::new(Rc::downgrade(key)), f);

    }

    pub fn unsubscribe_rc(&mut self, key: &Rc<K>)
    {

        self.contents.remove(WeakPartialEq::new(Rc::downgrade(key)));

    }
    */

    pub fn get_pub_event<'a>(&'a mut self) -> LEPubEvent<'a, F, S>
    {

        LEPubEvent::new(self)

    }
    */

    delegate! {
        to self.ble { //contents {

            fn subscribe(&mut self, f: &Rc<F>) -> bool;

            fn unsubscribe(&mut self, f: &Rc<F>) -> bool;

            fn get_pub_event<'a>(&'a mut self) -> LEPubEvent<'a, F, S>;



            fn len(&self) -> usize;

            fn is_empty(&self) -> bool;

        }
    }

}


//https://doc.rust-lang.org/error_codes/E0403.html

impl<F, S> ListEvent<F, S> 
    where F: Fn(&mut S)
{

    pub fn raise(&mut self, sender: &mut S)
        where F: Fn(&mut S)
    {

        for item in self.ble.iter()
        {

            item.get_contents_ref()(sender); 

        }

    }

}
 */

/*
impl<F, S, P> ListEvent<F, S> 
    where F: Fn(&mut S, &P)
{

    pub fn raise_ref(&mut self, sender: &mut S, event_args: &P)
        where F: Fn(&mut S, &P)
    {

        for item in self.contents.iter()
        {

            item.get_contents_ref()(sender, event_args); 

        }

    }

}
*/

/*
impl<F> ListEvent<F> 
    where F: PartialEq
{


    pub fn raise<S>(&mut self, sender: &mut S)
        where F: Fn(&mut S)
    {

        for item in self.contents.iter()
        {

            item(sender); 

        }

    }

    pub fn raise_ref<S, P>(&mut self, sender: &mut S, event_args: &P)
        where F: Fn(&mut S, &P)
    {

        for item in self.contents.iter()
        {

            item(sender, event_args); 

        }

    }

    pub fn raise_copy<S, P>(&mut self, sender: &mut S, event_args: P)
        where F: Fn(&mut S, P), P: Copy
    {

        for item in self.contents.iter()
        {

            item(sender, event_args); 

        }

    }
    

    pub fn raise_clone<S, P>(&mut self, sender: &mut S, event_args: P)
        where F: Fn(&mut S, P), P: Clone
    {

        for item in self.contents.iter()
        {

            item(sender, event_args.clone()); 

        }

    }

}
*/

//"Associated type bounds are unstable"

//Tracking issue for RFC 2289, "Associated type bounds" #52662
//https://github.com/rust-lang/rust/issues/52662
//

/*
impl<F> AddAssign<Rhs: F> for ListEvent<F>
    where F: PartialEq
{

    //type AddAssign::Rhs = F;

    fn add_assign(&mut self, rhs: Rhs) {
    
        self.subscribe(f);

    }

}
*/

/*
pub struct LEPubEvent<'a, F, S>
    //where F: PartialEq
{

    le: &'a mut ListEvent<F, S>

}

impl<'a, F, S> LEPubEvent<'a, F, S>
    //where F: PartialEq
{

    pub fn new(le: &'a mut ListEvent<F, S>) -> Self
    {

        Self
        {

            le

        }

    }

    delegate! {
        to self.le {

            fn subscribe(&mut self, f: &Rc<F>);

            fn unsubscribe(&mut self, f: &Rc<F>);

            /*
            fn subscribe_rc(&mut self, key: &Rc<K>, f: F);

            fn unsubscribe_rc(&mut self, key: &Rc<K>);
            */

            fn len(&self) -> usize;

            fn is_empty(&self) -> bool;

        }

    }

}
 */
*/