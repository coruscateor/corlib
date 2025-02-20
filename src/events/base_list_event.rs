use std::marker::PhantomData;
use std::rc::{Rc, Weak};

use std::ops::Fn;
use std::slice::Iter;

use crate::collections::UniqueItemList;

use crate::rc::WeakByPtr;

use std::vec::Vec;

use delegate::delegate;

//use std::ops::{AddAssign, SubAssign};

//UniqueList

use crate::rc::RcByPtr;

///
/// A base struct for UniqueItemList based events containing event handler functions,
/// 
pub struct BaseListEvent<F: ?Sized, S>
    //where F: PartialEq
{

    contents: UniqueItemList<RcByPtr<F>>,
    phantom: PhantomData<S>

}

impl<F: ?Sized, S> BaseListEvent<F, S>
    //where F: PartialEq
{

    pub fn new() -> Self
    {

        Self
        {

            contents: UniqueItemList::new(),
            phantom: PhantomData::default()

        }

    }

    pub fn with_capacity(capacity: usize) -> Self
    {

        Self
        {

            contents: UniqueItemList::with_capacity(capacity),
            phantom: PhantomData::default()

        }

    }

    pub fn subscribe(&mut self, f: &Rc<F>) -> bool
    {

        self.contents.add(RcByPtr::new(&f)) //f.clone()))

    }

    pub fn unsubscribe(&mut self, f: &Rc<F>) -> bool
    {

        self.contents.remove(RcByPtr::new(&f)) //f.clone()))

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

    delegate! {
        to self.contents {

            pub fn len(&self) -> usize;

            pub fn is_empty(&self) -> bool;

            //pub fn iter() -> Iter<'_, RcEq<F>>>;

        }
    }
    
    pub fn iter(&self) -> Iter<'_, RcByPtr<F>>
    {

        self.contents.iter()

    }

    pub fn remove_at_indexes(&mut self, indexs_to_remove: Vec<usize>) -> usize
    {

        for item in indexs_to_remove.iter().rev()
        {

            self.contents.remove_at(*item);

        }

        indexs_to_remove.len()

    }

    //proc macro panicked
    //message: Do not include implementation of delegated functions (iter)rustcClick for full compiler diagnostic

}


pub struct LEPubEvent<'a, F: ?Sized, S>
    //where F: PartialEq
{

    le: &'a mut BaseListEvent<F, S>

}


impl<'a, F: ?Sized, S> LEPubEvent<'a, F, S>
    //where F: PartialEq
{

    pub fn new(le: &'a mut BaseListEvent<F, S>) -> Self
    {

        Self
        {

            le

        }

    }

    delegate! {
        to self.le {

            fn subscribe(&mut self, f: &Rc<F>) -> bool;

            fn unsubscribe(&mut self, f: &Rc<F>) -> bool;

            /*
            fn subscribe_rc(&mut self, key: &Rc<K>, f: F);

            fn unsubscribe_rc(&mut self, key: &Rc<K>);
            */

            fn len(&self) -> usize;

            fn is_empty(&self) -> bool;

        }

    }

}