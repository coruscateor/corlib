use std::any::Any;
use std::marker::PhantomData;
use std::rc::{Rc, Weak};

use std::ops::Fn;
use std::slice::Iter;

use crate::collections::{Dictionary, KeyValuePair};

use crate::rc::RcByPtr;

use crate::rc::WeakByPtr;

use std::vec::Vec;

use delegate::delegate;

//use std::ops::{AddAssign, SubAssign};

///
/// A base struct for Dictionary oriented events indexed by weak Rc pointers to event handler objects.
/// 
pub struct BaseDictionaryWeakRcEvent<F: ?Sized, S> //K: ?Sized, 'a, //, S, P>
    //where K: PartialEq //, F: Fn(&Rc<S>, &Rc<K>, P)
{

    contents: Dictionary<WeakByPtr<dyn Any>, Rc<F>>,
    //pd: PhantomData
    phantom: PhantomData<S>

}

impl<F: ?Sized, S> BaseDictionaryWeakRcEvent<F, S> //K: ?Sized, //K, 'a, 
    //where K: PartialEq
{

    pub fn new() -> Self
    {

        Self
        {

            contents: Dictionary::new(),
            phantom: PhantomData::default()

        }

    }

    pub fn with_capacity(capacity: usize) -> Self
    {

        Self
        {

            contents: Dictionary::with_capacity(capacity),
            phantom: PhantomData::default()

        }

    }

    pub fn subscribe(&mut self, key: Weak<dyn Any>, f: &Rc<F>) -> bool
    {

        //self.contents.add_or_repace(WeakPtrEq::new(key), f);

        self.contents.add(WeakByPtr::new(&key), f.clone())

    }

    pub fn unsubscribe(&mut self, key: Weak<dyn Any>) -> bool
    {

        self.contents.remove(WeakByPtr::new(&key))

    }

    pub fn subscribe_rc(&mut self, key: &Rc<dyn Any>, f: &Rc<F>) -> bool
    {

        //self.contents.add_or_repace(WeakPtrEq::new(Rc::downgrade(key)), f);

        self.contents.add(WeakByPtr::new(&Rc::downgrade(key)), f.clone())

    }

    pub fn unsubscribe_rc(&mut self, key: &Rc<dyn Any>) -> bool
    {

        self.contents.remove(WeakByPtr::new(&Rc::downgrade(key)))

    }

    pub fn get_pub_event<'a>(&'a mut self) -> WDEPubEvent<'a, F, S> //K,
    {
        //<'a>

        WDEPubEvent::new(self)

    }

    delegate! {
        to self.contents {

            pub fn len(&self) -> usize;

            pub fn is_empty(&self) -> bool;

        }
    }

    pub fn iter(&self) -> Iter<'_, KeyValuePair<WeakByPtr<dyn Any>, Rc<F>>>
    {

        self.contents.iter()

    }

    pub fn remove_at_indexes(&mut self, indexs_to_remove: Vec<usize>) -> usize //items: &mut Dictionary<K, F>,
    {

        for item in indexs_to_remove.iter().rev()
        {

            self.contents.remove_at(*item);

        }

        indexs_to_remove.len()

    }

}


///
/// A wrapper for BaseDictionaryWeakRcEvent intended for public usage.
/// 
pub struct WDEPubEvent<'a, F: ?Sized, S> //K: ?Sized, 
    //where K: PartialEq
{

    wde: &'a mut BaseDictionaryWeakRcEvent<F, S> //K, 

}

impl<'a, F: ?Sized, S> WDEPubEvent<'a, F, S> //K: ?Sized, //K, 
    //where K: PartialEq
{

    pub fn new(wde: &'a mut BaseDictionaryWeakRcEvent<F, S>) -> Self //K, 
    {

        Self
        {

            wde

        }

    }

    delegate! {
        to self.wde {

            pub fn subscribe(&mut self, key: Weak<dyn Any>, f: &Rc<F>) -> bool;

            pub fn unsubscribe(&mut self, key: Weak<dyn Any>) -> bool;

            pub fn subscribe_rc(&mut self, key: &Rc<dyn Any>, f: &Rc<F>) -> bool;

            pub fn unsubscribe_rc(&mut self, key: &Rc<dyn Any>) -> bool;

            pub fn get_pub_event(&'a mut self) -> WDEPubEvent<'a, F, S>; //K,

            pub fn len(&self) -> usize;

            pub fn is_empty(&self) -> bool;

        }

    }

}