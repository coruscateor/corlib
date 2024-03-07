use std::any::Any;
use std::marker::PhantomData;
use std::rc::{Rc, Weak};

use std::ops::Fn;

use crate::collections::{Dictionary};

use crate::weak_by_ptr::WeakByPtr;

use std::vec::Vec;

use delegate::delegate;

use super::base_dictionary_weak_rc_event::{BaseDictionaryWeakRcEvent, WDEPubEvent};

//use std::ops::{AddAssign, SubAssign};

//type FuncType<S> = dyn Fn(&S, Rc<dyn Any>); //mut 

pub type SenderRcAnyEventFunc<S> = dyn Fn(&S, Rc<dyn Any>) -> bool;

///
/// A Dictionary based event indexed by weak handler object references.
/// 
pub struct DictionaryWeakRcEvent<S> //K, 
    //where K: PartialEq
    //where F: Fn(&mut S, Rc<K>)
{

    //contents: Dictionary<WeakPtrEq<K>, F>,
    //pd: PhantomData
    bdwre: BaseDictionaryWeakRcEvent<SenderRcAnyEventFunc<S>, S>, //K, 


}

impl<S> DictionaryWeakRcEvent<S> //K,  
    //where K: PartialEq
    //where F: Fn(&mut S, Rc<K>)
{

    pub fn new() -> Self
    {

        Self
        {

            bdwre: BaseDictionaryWeakRcEvent::new()

        }

    }

    pub fn with_capacity(capacity: usize) -> Self
    {

        Self
        {

            bdwre: BaseDictionaryWeakRcEvent::with_capacity(capacity)

        }

    }

    delegate! {
        to self.bdwre {

            pub fn subscribe(&mut self, key: Weak<dyn Any>, f: &Rc<SenderRcAnyEventFunc<S>>) -> bool;

            pub fn unsubscribe(&mut self, key: Weak<dyn Any>) -> bool;

            pub fn subscribe_rc(&mut self, key: &Rc<dyn Any>, f: &Rc<SenderRcAnyEventFunc<S>>) -> bool;

            pub fn unsubscribe_rc(&mut self, key: &Rc<dyn Any>) -> bool;

            pub fn get_pub_event<'a>(&'a mut self) -> WDEPubEvent<'a, SenderRcAnyEventFunc<S>, S>;

            pub fn len(&self) -> usize;

            pub fn is_empty(&self) -> bool;
        }
    }

}

impl<S> DictionaryWeakRcEvent<S> //F, K,
    //where K: PartialEq
    //where F: Fn(&mut S, Rc<K>)
{

    pub fn raise(&mut self, sender: &S) -> usize //mut 
    {

        let mut items_to_remove = Vec::new();

        let mut index: usize = 0;

        for item in self.bdwre.iter()
        {

            let kr = item.get_key_ref();

            let handler_object_option = kr.upgrade_contents();

            if let Some(handler_object) = handler_object_option
            {

                if !item.get_value_ref()(sender, handler_object)
                {

                    items_to_remove.push(index);
    
                }

            }
            else
            {
            
                items_to_remove.push(index);
                
            }

            index += 1;

        }

        self.bdwre.remove_at_indexes(items_to_remove)

    }

}

/*
pub struct DictionaryWeakRcEvent<K, F> //'a, //, S, P>
    where K: PartialEq //, F: Fn(&Rc<S>, &Rc<K>, P)
{

    contents: Dictionary<WeakPtrEq<K>, F>,
    //pd: PhantomData

}

impl<K, F> DictionaryWeakRcEvent<K, F> //'a, 
    where K: PartialEq
{

    pub fn new() -> Self
    {

        Self
        {

            contents: Dictionary::new()

        }

    }

    pub fn with_capacity(capacity: usize) -> Self
    {

        Self
        {

            contents: Dictionary::with_capacity(capacity)

        }

    }

    pub fn subscribe(&mut self, key: Weak<K>, f: F) -> bool
    {

        //self.contents.add_or_repace(WeakPtrEq::new(key), f);

        self.contents.add(WeakPtrEq::new(key), f)

    }

    pub fn unsubscribe(&mut self, key: Weak<K>) -> bool
    {

        self.contents.remove(WeakPtrEq::new(key))

    }

    pub fn subscribe_rc(&mut self, key: &Rc<K>, f: F) -> bool
    {

        //self.contents.add_or_repace(WeakPtrEq::new(Rc::downgrade(key)), f);

        self.contents.add(WeakPtrEq::new(Rc::downgrade(key)), f)

    }

    pub fn unsubscribe_rc(&mut self, key: &Rc<K>) -> bool
    {

        self.contents.remove(WeakPtrEq::new(Rc::downgrade(key)))

    }

    pub fn get_pub_event<'a>(&'a mut self) -> WDEPubEvent<'a, K, F>
    {
        //<'a>

        WDEPubEvent::new(self)

    }

    delegate! {
        to self.contents {

            fn len(&self) -> usize;

            fn is_empty(&self) -> bool;

        }
    }

    fn remove_at_indexes(&mut self, indexs_to_remove: Vec<usize>) -> usize //items: &mut Dictionary<K, F>,
    {

        for item in indexs_to_remove.iter().rev()
        {

            self.contents.remove_at(*item);

        }

        indexs_to_remove.len()

    }

}

impl<K, F> DictionaryWeakRcEvent<K, F> //'a, 
    where K: PartialEq
{

    pub fn raise<S>(&mut self, sender: &mut S) -> usize
        where F: Fn(&mut S, Rc<K>)
    {

        let mut items_to_remove = Vec::new();

        for item in self.contents.iter()
        {

            let mut index: usize = 0;

            let kr = item.get_key_ref();

            let handler_object_option = kr.upgrade();

            if let Some(handler_object) = handler_object_option
            {

                item.get_value_ref()(sender, handler_object);

            }
            else
            {
            
                items_to_remove.push(index);
                
            }

            index += 1;

        }

        self.remove_at_indexes(items_to_remove)

    }

    pub fn raise_ref<S, P>(&mut self, sender: &mut S, event_args: &P) -> usize
        where F: Fn(&mut S, Rc<K>, &P)
    {

        let mut items_to_remove = Vec::new();

        for item in self.contents.iter()
        {

            let mut index: usize = 0;

            let kr = item.get_key_ref();

            let handler_object_option = kr.upgrade();

            if let Some(handler_object) = handler_object_option
            {

                item.get_value_ref()(sender, handler_object, event_args); //item.get_value_ref()(sender, );

            }
            else
            {
            
                items_to_remove.push(index);
                
            }

            index += 1;

        }

        self.remove_at_indexes(items_to_remove) //&mut self.contents,

    }

    pub fn raise_copy<S, P>(&mut self, sender: &mut S, event_args: P) -> usize
        where F: Fn(&mut S, Rc<K>, P), P: Copy
    {

        let mut items_to_remove = Vec::new();

        for item in self.contents.iter()
        {

            let mut index: usize = 0;

            let kr = item.get_key_ref();

            let handler_object_option = kr.upgrade();

            if let Some(handler_object) = handler_object_option
            {

                item.get_value_ref()(sender, handler_object, event_args);

            }
            else
            {
            
                items_to_remove.push(index);
                
            }

            index += 1;

        }

        self.remove_at_indexes(items_to_remove)

    }

    pub fn raise_clone<S, P>(&mut self, sender: &mut S, event_args: P) -> usize
        where F: Fn(&mut S, Rc<K>, P), P: Clone
    {

        let mut items_to_remove = Vec::new();

        for item in self.contents.iter()
        {

            let mut index: usize = 0;

            let kr = item.get_key_ref();

            let handler_object_option = kr.upgrade();

            if let Some(handler_object) = handler_object_option
            {

                item.get_value_ref()(sender, handler_object, event_args.clone());

            }
            else
            {
            
                items_to_remove.push(index);
                
            }

            index += 1;

        }

        self.remove_at_indexes(items_to_remove)

    }

}

//impl AddAssign<>

pub struct WDEPubEvent<'a, K, F>
    where K: PartialEq
{

    wde: &'a mut DictionaryWeakRcEvent<K, F>

}

impl<'a, K, F> WDEPubEvent<'a, K, F>
    where K: PartialEq
{

    pub fn new(wde: &'a mut DictionaryWeakRcEvent<K, F>) -> Self
    {

        Self
        {

            wde

        }

    }

    delegate! {
        to self.wde {

            fn subscribe(&mut self, key: Weak<K>, f: F) -> bool;

            fn unsubscribe(&mut self, key: Weak<K>) -> bool;

            fn subscribe_rc(&mut self, key: &Rc<K>, f: F) -> bool;

            fn unsubscribe_rc(&mut self, key: &Rc<K>) -> bool;

            fn len(&self) -> usize;

            fn is_empty(&self) -> bool;

        }

    }

}
*/