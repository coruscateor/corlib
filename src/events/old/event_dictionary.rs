use std::rc::{Rc, Weak};

//use std::collections::HashMap;

//use std::hash::Hash;

use std::ops::Fn;

//use std::cell::RefCell;

//use std::marker::PhantomData;

//use crate::collections::hashset_item::HashsetItem;

//use super::{dictionary::Dictionary, weak_partial_eq::WeakPartialEq};

use crate::collections::{Dictionary}; //dictionary::

use crate::weak_partial_eq::WeakPartialEq;

use std::vec::Vec;

pub struct EventDictionary<K, F>
    where K: PartialEq //+ ?Sized
{

    contents: Dictionary<K, F> 

}

impl<K, F> EventDictionary<K, F>
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

    pub fn subscribe(&mut self, key: K, f: F) //Rc<RefCell<T>>, f: F)
    {

        self.contents.add_or_repace(key, f);

    }

    pub fn unsubscribe(&mut self, key: K)
    {

        self.contents.remove(key);

    }

    pub fn len(&self) -> usize
    {

        self.contents.len()

    }

    pub fn is_empty(&self) -> bool
    {

        self.contents.is_empty()

    }

}

impl<K, F> EventDictionary<K, F>
    where K: PartialEq, F: Fn()
{

    pub fn raise(&self)
    {

        for item in self.contents.iter()
        {

            item.get_value_ref()();

        }

    }

}

/* 
pub struct EventDictionary1P<K, F, P>
    where K: PartialEq
{

    contents: Dictionary<K, F>,
    phantom_data: PhantomData<P>

}
*/

impl<K, F> EventDictionary<K, F>
    where K: PartialEq,
{

    pub fn raise_copy<P>(&self, parameter: P)
        where F: Fn(P), P: Copy
    {

        for item in self.contents.iter()
        {

            item.get_value_ref()(parameter);

        }

    }

}

impl<K, F> EventDictionary<K, F>
    where K: PartialEq
{

    pub fn raise_clone<P>(&self, parameter: &P)
        where K: PartialEq, F: Fn(P), P: Clone
    {

        for item in self.contents.iter()
        {

            item.get_value_ref()(parameter.clone());

        }

    }

}

/*
impl<K, F> EventDictionary<K, F>
    where K: PartialEq
{

    pub fn raise_ref<P>(&self, parameter: &P)
        where K: PartialEq, F: Fn(&P)
    {

        for item in self.contents.iter()
        {

            item.get_value_ref()(parameter);

        }

    }

}

impl<K, F> EventDictionary<K, F>
    where K: PartialEq
{

    pub fn raise_mut<P>(&self, parameter: &mut P)
        where K: PartialEq, F: Fn(&mut P)
    {

        for item in self.contents.iter()
        {

            item.get_value_ref()(parameter);

        }

    }

}
*/

//

fn remove_items<K, F>(items: &mut Dictionary<K, F>, indexs_to_remove: Vec<usize>)
    where K: PartialEq
{

    for item in indexs_to_remove.iter().rev()
    {

        items.remove_at(*item);

    }

}

impl<K: ?Sized, F> EventDictionary<WeakPartialEq<K>, F> //<RefCell<K>>
    //where ?Sized //K: PartialEq + 
{

    //key

    pub fn rc_raise(&mut self)
        //where K: PartialEq
        where F: Fn(Rc<K>) //(Rc<RefCell<K>>)  //Rc<RefCell<K>>) //no sender
    {

        let mut index: usize = 0;

        let mut items_to_remove = Vec::new();

        for item in self.contents.iter()
        {

            let handler_object_option = item.get_key_ref().upgrade();

            if let Some(handler_object) = handler_object_option
            {

                item.get_value_ref()(handler_object); //(key_rc);

            }
            else
            {
            
                items_to_remove.push(index);
                
            }

            index += 1;

            //item.get_value_ref()(parameter);

        }

        remove_items(&mut self.contents, items_to_remove);

    }

    //key, sender

    pub fn rc_raise_p_copy<P>(&mut self, p: P)
        where F: Fn(Rc<K>, P), P: Copy //sender  //Rc<RefCell<K>>, P), P: Copy
    {

        let mut index: usize = 0;

        let mut items_to_remove = Vec::new();

        for item in self.contents.iter()
        {

            let handler_object_option = item.get_key_ref().upgrade();

            if let Some(handler_object) = handler_object_option
            {

                item.get_value_ref()(handler_object, p); //(key_rc, p);

            }
            else
            {
            
                items_to_remove.push(index);
                
            }

            index += 1;

            //item.get_value_ref()(parameter);

        }

        remove_items(&mut self.contents, items_to_remove);

    }

    //key, sender

    pub fn rc_raise_p_clone<P>(&mut self, p: P)
    where F: Fn(Rc<K>, P), P: Clone //sender //Rc<RefCell<K>>, P), P: Clone
    {

        let mut index: usize = 0;

        let mut items_to_remove = Vec::new();

        for item in self.contents.iter()
        {

            let handler_object_option = item.get_key_ref().upgrade();

            if let Some(handler_object) = handler_object_option
            {

                item.get_value_ref()(handler_object, p.clone()); //(key_rc, p.clone());

            }
            else
            {
            
                items_to_remove.push(index);
                
            }

            index += 1;

            //item.get_value_ref()(parameter);

        }

        remove_items(&mut self.contents, items_to_remove);

    }

    //key, sender, other

    pub fn rc_raise_2_p_copy<P2, P3>(&mut self, p2: P2, p3: P3)
        where F: Fn(Rc<K>, P2, P3), P2: Copy, P3: Copy //Rc<RefCell<K>>, P1, P2), P1: Copy, P2: Copy
    {

        let mut index: usize = 0;

        let mut items_to_remove = Vec::new();

        for item in self.contents.iter()
        {

            let handler_object_option = item.get_key_ref().upgrade();

            if let Some(handler_object) = handler_object_option
            {

                item.get_value_ref()(handler_object, p2, p3); //(key_rc, p1, p2);

            }
            else
            {
            
                items_to_remove.push(index);
                
            }

            index += 1;

            //item.get_value_ref()(parameter);

        }

        remove_items(&mut self.contents, items_to_remove);

    }

    //key, sender, other

    pub fn rc_raise_2_p_clone<P2, P3>(&mut self, p2: P2, p3: P3)
        where F: Fn(Rc<K>, P2, P3), P2: Clone, P3: Clone //Rc<RefCell<K>>, P1, P2), P1: Clone, P2: Clone
    {

        let mut index: usize = 0;

        let mut items_to_remove = Vec::new();

        for item in self.contents.iter()
        {

            let handler_object_option = item.get_key_ref().upgrade();

            if let Some(handler_object) = handler_object_option
            {

                item.get_value_ref()(handler_object, p2.clone(), p3.clone()); //key_rc, p1.clone(), p2.clone());

            }
            else
            {
            
                items_to_remove.push(index);
                
            }

            index += 1;

            //item.get_value_ref()(parameter);

        }

        remove_items(&mut self.contents, items_to_remove);

    }

    //key, sender, other

    pub fn rc_raise_p_copy_p_clone<P2, P3>(&mut self, p2: P2, p3: P3)
        where F: Fn(Rc<K>, P2, P3), P2: Copy, P3: Clone
    {

        let mut index: usize = 0;

        let mut items_to_remove = Vec::new();

        for item in self.contents.iter()
        {

            let handler_object_option = item.get_key_ref().upgrade();

            if let Some(handler_object) = handler_object_option
            {

                item.get_value_ref()(handler_object, p2, p3.clone());

            }
            else
            {
            
                items_to_remove.push(index);
                
            }

            index += 1;

        }

        remove_items(&mut self.contents, items_to_remove);

    }

    //key, sender, other

    pub fn rc_raise_p_clone_p_copy<P2, P3>(&mut self, p2: P2, p3: P3)
        where F: Fn(Rc<K>, P2, P3), P2: Clone, P3: Copy
    {

        let mut index: usize = 0;

        let mut items_to_remove = Vec::new();

        for item in self.contents.iter()
        {

            let handler_object_option = item.get_key_ref().upgrade();

            if let Some(handler_object) = handler_object_option
            {

                item.get_value_ref()(handler_object, p2.clone(), p3);

            }
            else
            {
            
                items_to_remove.push(index);
                
            }

            index += 1;

        }

        remove_items(&mut self.contents, items_to_remove);

    }

    //3

    pub fn rc_raise_3_p_copy<P2, P3, P4>(&mut self, p2: P2, p3: P3, p4: P4)
        where F: Fn(Rc<K>, P2, P3, P4), P2: Copy, P3: Copy, P4: Copy //Rc<RefCell<K>>, P1, P2), P1: Copy, P2: Copy
    {

        let mut index: usize = 0;

        let mut items_to_remove = Vec::new();

        for item in self.contents.iter()
        {

            let handler_object_option = item.get_key_ref().upgrade();

            if let Some(handler_object) = handler_object_option
            {

                item.get_value_ref()(handler_object, p2, p3, p4); //(key_rc, p1, p2);

            }
            else
            {
            
                items_to_remove.push(index);
                
            }

            index += 1;

            //item.get_value_ref()(parameter);

        }

        remove_items(&mut self.contents, items_to_remove);

    }

    //key, sender, other

    pub fn rc_raise_3_p_clone<P2, P3, P4>(&mut self, p2: P2, p3: P3, p4: P4)
        where F: Fn(Rc<K>, P2, P3, P4), P2: Clone, P3: Clone, P4: Clone //Rc<RefCell<K>>, P1, P2), P1: Clone, P2: Clone
    {

        let mut index: usize = 0;

        let mut items_to_remove = Vec::new();

        for item in self.contents.iter()
        {

            let handler_object_option = item.get_key_ref().upgrade();

            if let Some(handler_object) = handler_object_option
            {

                item.get_value_ref()(handler_object, p2.clone(), p3.clone(), p4.clone()); //key_rc, p1.clone(), p2.clone());

            }
            else
            {
            
                items_to_remove.push(index);
                
            }

            index += 1;

            //item.get_value_ref()(parameter);

        }

        remove_items(&mut self.contents, items_to_remove);

    }

    //key, sender, other

    /*
    pub fn rc_raise_p_copy_p_clone_p_copy<P2, P3, P4>(&mut self, p2: P2, p3: P3, p4: P4)
        where F: Fn(Rc<K>, P2, P3, P4), P2: Copy, P3: Clone, P4: Copy
    {

        let mut index: usize = 0;

        let mut items_to_remove = Vec::new();

        for item in self.contents.iter()
        {

            let handler_object_option = item.get_key_ref().upgrade();

            if let Some(handler_object) = handler_object_option
            {

                item.get_value_ref()(handler_object, p2, p3.clone(), p4);

            }
            else
            {
            
                items_to_remove.push(index);
                
            }

            index += 1;

        }

        remove_items(&mut self.contents, items_to_remove);

    }
    */

        //key, sender, other

    pub fn rc_raise_p_copy_p_clone_p_clone<P2, P3, P4>(&mut self, p2: P2, p3: P3, p4: P4)
        where F: Fn(Rc<K>, P2, P3, P4), P2: Copy, P3: Clone, P4: Clone
    {

        let mut index: usize = 0;

        let mut items_to_remove = Vec::new();

        for item in self.contents.iter()
        {

            let handler_object_option = item.get_key_ref().upgrade();

            if let Some(handler_object) = handler_object_option
            {

                item.get_value_ref()(handler_object, p2, p3.clone(), p4.clone());

            }
            else
            {
            
                items_to_remove.push(index);
                
            }

            index += 1;

        }

        remove_items(&mut self.contents, items_to_remove);

    }

    //key, sender, other

    pub fn rc_raise_p_clone_p_copy_p_copy<P2, P3, P4>(&mut self, p2: P2, p3: P3, p4: P4)
        where F: Fn(Rc<K>, P2, P3, P4), P2: Clone, P3: Copy, P4: Copy
    {

        let mut index: usize = 0;

        let mut items_to_remove = Vec::new();

        for item in self.contents.iter()
        {

            let handler_object_option = item.get_key_ref().upgrade();

            if let Some(handler_object) = handler_object_option
            {

                item.get_value_ref()(handler_object, p2.clone(), p3, p4);

            }
            else
            {
            
                items_to_remove.push(index);
                
            }

            index += 1;

        }

        remove_items(&mut self.contents, items_to_remove);

    }

    //key, sender, other

    pub fn rc_raise_p_clone_p_clone_p_copy<P2, P3, P4>(&mut self, p2: P2, p3: P3, p4: P4)
        where F: Fn(Rc<K>, P2, P3, P4), P2: Clone, P3: Clone, P4: Copy
    {

        let mut index: usize = 0;

        let mut items_to_remove = Vec::new();

        for item in self.contents.iter()
        {

            let handler_object_option = item.get_key_ref().upgrade();

            if let Some(handler_object) = handler_object_option
            {

                item.get_value_ref()(handler_object, p2.clone(), p3.clone(), p4);

            }
            else
            {
            
                items_to_remove.push(index);
                
            }

            index += 1;

        }

        remove_items(&mut self.contents, items_to_remove);

    }

    //key, sender, other

    pub fn rc_raise_p_clone_p_copy_p_clone<P2, P3, P4>(&mut self, p2: P2, p3: P3, p4: P4)
        where F: Fn(Rc<K>, P2, P3, P4), P2: Clone, P3: Copy, P4: Clone
    {

        let mut index: usize = 0;

        let mut items_to_remove = Vec::new();

        for item in self.contents.iter()
        {

            let handler_object_option = item.get_key_ref().upgrade();

            if let Some(handler_object) = handler_object_option
            {

                item.get_value_ref()(handler_object, p2.clone(), p3, p4.clone());

            }
            else
            {
            
                items_to_remove.push(index);
                
            }

            index += 1;

        }

        remove_items(&mut self.contents, items_to_remove);

    }


}

//

/*
impl<K, F, P> EventDictionary<WeakPartialEq<RefCell<K>>, F>
    where K: PartialEq, F: Fn(Rc<RefCell<K>, P>)
{

    pub fn rc_raise_ref(&mut self)
    //where K: PartialEq
    {

        let mut index: usize = 0;

        let mut items_to_remove = Vec::new();

        for item in self.contents.iter()
        {

            let handler_object_option = item.get_key_ref().upgrade();

            if let Some(key_rc) = handler_object_option
            {

                item.get_value_ref()(key_rc);

            }
            else
            {
            
                items_to_remove.push(index);
                
            }

            index += 1;

            //item.get_value_ref()(parameter);

        }

        for item in items_to_remove.iter().rev()
        {

            self.contents.remove_at(*item);

        }

    }

}

}
 */

pub trait SubscribeToThis<ST, K, F>
{

    fn subscribe(subscription_type: ST, key: K, f: F);

    fn unsubscribe(subscription_type: ST, key: K);
    
}

/*
impl<K, F> EventDictionary<Rc<RefCell<K>>, F>
    where K: PartialEq
{

    pub fn rc_raise_mut<P>(&self, parameter: &mut P)
        where K: PartialEq, F: Fn(&mut P)
    {

        for item in self.contents.iter()
        {

            item.get_value_ref()(parameter);

        }

    }

}
*/

/*
pub struct RCEventHashMap<K, F>
    where K: Eq + Hash //, F: Fn(Rc<RefCell<T>>)
{

    //kvs: HashMap<HashsetItem<Weak<RefCell<T>>>, F>
    contents: Dictionary<K, F> 

}

impl<T, F> RCEventHashMap<T, F>
    where T: Eq + Hash, F: Fn(Rc<RefCell<T>>)
{

    pub fn subscribe(&mut self, subscriber: Rc<RefCell<T>>, f: F)
    {

        //self.kvs.insert(HashsetItem::new(subscriber), f);

        self.contents.

    }

}
*/

/*
pub struct RCEventHashMap1P<K, F, P>
    where K: Eq + Hash
{

    phantom_data: PhantomData<P>,
    //kvs: HashMap<Weak<RefCell<T>>, F>

}

impl<T, F, P> RCEventHashMap1P<T, F, P>
    where T: Eq + Hash, F: Fn(Rc<RefCell<T>>, P)
{



}
*/
