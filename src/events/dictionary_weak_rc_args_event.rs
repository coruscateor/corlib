use std::any::Any;
use std::marker::PhantomData;
use std::rc::{Rc, Weak};

use std::ops::Fn;

use crate::collections::{Dictionary};

use crate::rc::WeakByPtr;

use std::vec::Vec;

use delegate::delegate;

use super::base_dictionary_weak_rc_event::{BaseDictionaryWeakRcEvent, WDEPubEvent};

//use std::ops::{AddAssign, SubAssign};

//type FuncType<S, A> = dyn Fn(&S, Rc<dyn Any>, &A); //mut 

pub type SenderRcAnyArgEventFunc<S, A> = dyn Fn(&S, Rc<dyn Any>, &A) -> bool;

///
/// A dictionary based event indexed by weak handler object references and taking functions that requre event arguments structs.
/// 
pub struct DictionaryWeakRcArgsEvent<S, A> //F,  //K, 
    //where K: PartialEq
    //where F: Fn(&mut S, Rc<dyn Any>, &A)
{

    //contents: Dictionary<WeakPtrEq<K>, F>,
    //pd: PhantomData
    bdwre: BaseDictionaryWeakRcEvent<SenderRcAnyArgEventFunc<S, A>, S>, //K,
    phantom: PhantomData<A>

}

impl<S, A> DictionaryWeakRcArgsEvent<S, A> //K,
    //where K: PartialEq
    //where F: Fn(&mut S, Rc<dyn Any>, &A)
{

    pub fn new() -> Self
    {

        Self
        {

            bdwre: BaseDictionaryWeakRcEvent::new(),
            phantom: PhantomData::default()

        }

    }

    pub fn with_capacity(capacity: usize) -> Self
    {

        Self
        {

            bdwre: BaseDictionaryWeakRcEvent::with_capacity(capacity),
            phantom: PhantomData::default()

        }

    }

    delegate! {
        to self.bdwre {

            pub fn subscribe(&mut self, key: Weak<dyn Any>, f: &Rc<SenderRcAnyArgEventFunc<S, A>>) -> bool;

            pub fn unsubscribe(&mut self, key: Weak<dyn Any>) -> bool;

            pub fn subscribe_rc(&mut self, key: &Rc<dyn Any>, f: &Rc<SenderRcAnyArgEventFunc<S, A>>) -> bool;

            pub fn unsubscribe_rc(&mut self, key: &Rc<dyn Any>) -> bool;

            pub fn get_pub_event<'a>(&'a mut self) -> WDEPubEvent<'a, SenderRcAnyArgEventFunc<S, A>, S>; //K, 

            pub fn len(&self) -> usize;

            pub fn is_empty(&self) -> bool;
        }
    }

}

impl<S, A> DictionaryWeakRcArgsEvent<S, A> //K, 
    //where F: Fn(&mut S, Rc<K>, &A)
{

    pub fn raise(&mut self, sender: &S, event_args: &A) -> usize //mut 
        //where F: Fn(&mut S, Rc<K>, &A)
    {

        let mut items_to_remove = Vec::new();

        let mut index: usize = 0;

        for item in self.bdwre.iter()
        {

            let kr = item.get_key_ref();

            let handler_object_option = kr.upgrade_contents();

            if let Some(handler_object) = handler_object_option
            {

                if !item.get_value_ref()(sender, handler_object, event_args)
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