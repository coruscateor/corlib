
use std::slice::{Iter, IterMut};

use super::{list::List, key_value_pair::KeyValuePair};

use std::fmt::Debug;

///
/// A .NET style Dictionary
/// 
pub struct Dictionary<K, V>
    where K: PartialEq
{

    contents: List<KeyValuePair<K, V>>

}

impl<K, V> Dictionary<K, V>
    where K: PartialEq
{

    //const

    pub fn new() -> Self
    {

        Self
        {

            contents: List::new()

        }

    }

    pub fn with_capacity(capacity: usize) -> Self
    {

        Self
        {

            contents: List::with_capacity(capacity)

        }

    }

    pub fn add(&mut self, key: K, value: V) -> bool
    {

        let kvp = KeyValuePair::new(key, value);

        if !self.contents.contains(&kvp)
        {

            self.contents.add(kvp);

            return true;

        }

        false

    }

    pub fn add_or_repace(&mut self, key: K, value: V) //-> bool
    {

        let kvp = KeyValuePair::new(key, value);

        self.contents.add_or_repace(kvp);

    }

    pub fn remove(&mut self, key: K) -> bool
    {

        let mut index = 0;

        let mut item_found = false;

        for item in self.contents.iter()
        {

            if item.get_key_ref().eq(&key)
            {

                item_found = true;

                break;

            }

            index += 1;

        }

        if item_found
        {

            self.contents.remove_at(index);

        }

        item_found


    }

    pub fn remove_ref(&mut self, key: &K) -> bool
    {

        let mut index = 0;

        let mut item_found = false;

        for item in self.contents.iter()
        {

            if item.get_key_ref() == key
            {

                item_found = true;

                break;

            }

            index += 1;

        }

        if item_found
        {

            self.contents.remove_at(index);

        }
        
        item_found


    }

    pub fn remove_at(&mut self, index: usize) //-> Result<T, String>
    {

        self.contents.remove_at(index);

    }

    pub fn contains(&self, key: K) -> bool
    {

        for item in self.contents.iter()
        {

            if item.get_key_ref().eq(&key)
            {

                return true;

            }

        }

        false

    }

    pub fn contains_ref(&self, key: &K) -> bool
    {

        for item in self.contents.iter()
        {

            if item.get_key_ref() == key
            {

                return true;

            }

        }

        false

    }

    pub fn len(&self) -> usize
    {

        self.contents.len()

    }

    pub fn is_empty(&self) -> bool
    {

        self.contents.is_empty()

    }

    pub fn clear(&mut self)
    {

        self.contents.clear();

    }

    pub fn get_value_ref(&self, key: &K) -> Option<&V>
    {

        for item in self.contents.iter()
        {

            if item.get_key_ref() == key
            {

                return Some(item.get_value_ref());

            }

        }

        None

    }

    pub fn get_value_mut(&mut self, key: &K) -> Option<&mut V>
    {

        for item in self.contents.iter_mut()
        {

            if item.get_key_ref() == key
            {

                return Some(item.get_value_mut());

            }

        }

        None

    }

    pub fn get_kvp_ref(&self, key: &K) -> Option<&KeyValuePair<K, V>>
    {
        
        for item in self.contents.iter()
        {

            if item.get_key_ref() == key
            {

                return Some(item);

            }

        }

        None

    }

    pub fn get_kvp_mut(&mut self, key: &K) -> Option<&mut KeyValuePair<K, V>>
    {

        for item in self.contents.iter_mut()
        {

            if item.get_key_ref() == key
            {

                return Some(item);

            }

        }

        None

    }

    pub fn iter(&self) -> Iter<'_, KeyValuePair<K, V>>
    {

        self.contents.iter()

    }

    fn iter_mut(&mut self) -> IterMut<'_, KeyValuePair<K, V>>
    {

        self.contents.iter_mut()

    }

}

impl<K, V> Dictionary<K, V>
    where K: PartialEq, V: Copy
{

    pub fn add_or_replace_copy(&mut self, key: K, value: V) -> Option<V>
    {

        let kvp_option = self.get_kvp_mut(&key);

        if let Some(kvp) = kvp_option
        {

            let res = *kvp.get_value_ref();

            let v_mut = kvp.get_value_mut();

            *v_mut = value;

            return Some(res);

        }

        self.add(key, value);

        None

        /*
        let kvp = KeyValuePair::new(key, value);

        //let res_option = self.contents.add_or_repace_copy(kvp);

        let mut index = 0;

        //let mut  has_found_index = false;

        let mut old_kvp_option = None;

        for item in self.contents.iter()
        {

            if item.get_key_ref() == kvp.get_key_ref()
            {

                //has_found_index = true;

                old_kvp_option = Some(item);

                break;

            }

            index += 1;

        }

        if let Some(old_kvp) = old_kvp_option
        {

            let value = *old_kvp.get_value_ref();

            self.contents[index] = kvp;

            return Some(value);

        }
        */

        //None

    }

    pub fn remove_get_copy(&mut self, key: K) -> Option<V>
    {

        let mut index = 0;

        let mut item_found = false;

        for item in self.contents.iter()
        {

            if item.get_key_ref().eq(&key)
            {

                item_found = true;

                break;

            }

            index += 1;

        }

        if item_found
        {

            let res = *self.contents[index].get_value_ref();

            self.contents.remove_at(index);

            return Some(res);

        }

        None


    }

}

impl<K, V> Dictionary<K, V>
    where K: PartialEq, V: Clone
{

    pub fn add_or_replace_clone(&mut self, key: K, value: V) -> Option<V>
    {

        let kvp_option = self.get_kvp_mut(&key);

        if let Some(kvp) = kvp_option
        {

            let res = kvp.get_value_ref().clone();

            let v_mut = kvp.get_value_mut();

            *v_mut = value;

            return Some(res);

        }

        self.add(key, value);

        None

    }

    pub fn remove_get_clone(&mut self, key: K) -> Option<V>
    {

        let mut index = 0;

        let mut item_found = false;

        for item in self.contents.iter()
        {

            if item.get_key_ref().eq(&key)
            {

                item_found = true;

                break;

            }

            index += 1;

        }

        if item_found
        {

            let res = self.contents[index].get_value_ref().clone();

            self.contents.remove_at(index);

            return Some(res);

        }

        None


    }


}

impl<K, V> Debug for Dictionary<K, V>
    where K: PartialEq + Debug, V: Debug
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Dictionary").field("contents", &self.contents).finish()
    }
    
}


