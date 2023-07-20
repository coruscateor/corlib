//use std::alloc::Global; //https://doc.rust-lang.org/std/alloc/struct.Global.html

//pub struct List<T, A = Global>

//use std::result::Result;

use std::ops::{Index, IndexMut, RangeBounds};

use std::slice::{Iter, IterMut, SliceIndex};

use std::vec::Drain;


//More Result results to avoid panicking

//https://doc.rust-lang.org/std/primitive.isize.html

//https://doc.rust-lang.org/std/primitive.isize.html#associatedconstant.MAX

//https://doc.rust-lang.org/std/panic/fn.catch_unwind.html

///
/// A .net style list.
/// 
#[derive(Default)]
pub struct List<T>
    where T: PartialEq
{

    contents: Vec<T> //Vec<T, A>

}

impl<T> List<T>
    where T: PartialEq
{

    //const

    pub const fn new() -> Self
    {

        Self
        {

            contents: Vec::new()

        }

    }

    pub fn with_capacity(capacity: usize) -> Self
    {

        Self
        {

            contents: Vec::with_capacity(capacity)

        }

    }

    pub fn add(&mut self, value: T)
    {

        self.contents.push(value);

        //let index = self.contents.len();

        //self.contents.insert(index, value);

        /*
        if self.contents.len() < 1
        {

            self.contents.push(value);

        }
        else {
            
            let index = self.contents.len();

            self.contents.insert(index, value);

        }
        */

    }

    pub fn add_copy(&mut self, value: &T)
        where T: Copy
    {

        self.contents.push(*value);

    }

    pub fn add_clone(&mut self, value: &T)
        where T: Clone
    {

        self.contents.push(value.clone());

    }

    pub fn add_or_repace(&mut self, value: T) //-> Option<T>
    {

        let mut index = 0;

        let mut  has_found_index = false;

        for item in self.contents.iter()
        {

            if value.eq(item)
            {

                has_found_index = true;

                break;

            }

            index += 1;

        }

        if has_found_index
        {

            //let return_value = self.contents[index];

            self.contents[index] = value;

            //return Some(return_value);

        }
        else 
        {
       
            self.contents.push(value);   

        }

        //None

        //return something?
        
    }

    //panic safe

    /*
    pub fn add_ps(&mut self, value: T) -> Result<(), String>
    {

        //might not be correct...

        if self.contents.len() == isize::MAX.try_into().unwrap()
        {

           return Err("Max reached".to_string());

        }

        //check capacity

        self.contents.push(value);

        Ok(())

    }
    */

    pub fn remove(&mut self, value: T) ->  bool//Result<T, String>
    {

        //let removal_index: Option<usize> = None;

        let mut removal_index: usize = 0;

        let mut has_found_item = false;

        for item in self.contents.iter()
        {

            if value.eq(item)
            {

                has_found_item = true;

                break;

            }

            removal_index += 1;

        }

        has_found_item

        /*
        if has_found_item
        {

            let removed = self.contents.remove(removal_index);

            Ok(removed)

        }
        else
        {
        
            Err("Item not found".to_string())

        }
        */

    }

    pub fn remove_ref(&mut self, value: &T) -> bool //Result<T, String>
    {

        //let removal_index: Option<usize> = None;

        let mut removal_index: usize = 0;

        let mut has_found_item = false;

        for item in self.contents.iter()
        {

            if value == item
            {

                has_found_item = true;

                break;

            }

            removal_index += 1;

        }

        has_found_item

        /*
        if has_found_item
        {

            let removed = self.contents.remove(removal_index);

            Ok(removed)

        }
        else
        {
        
            Err("Item not found".to_string())

        }
        */

    }

    /*
    pub fn remove_and_drop(&mut self, value: T) -> Result<(), String>
    {

        //let removal_index: Option<usize> = None;

        let mut removal_index: usize = 0;

        let mut has_found_item = false;

        for item in self.contents.iter()
        {

            if value.eq(item)
            {

                has_found_item = true;

                break;

            }

            removal_index += 1;

        }

        if has_found_item
        {

            self.contents.remove(removal_index);

            Ok(())

        }
        else
        {
        
            Err("Item not found".to_string())

        }

    }

    pub fn remove_and_drop_ref(&mut self, value: &T) -> Result<(), String>
    {

        //let removal_index: Option<usize> = None;

        let mut removal_index: usize = 0;

        let mut has_found_item = false;

        for item in self.contents.iter()
        {

            if value == item
            {

                has_found_item = true;

                break;

            }

            removal_index += 1;

        }

        if has_found_item
        {

            self.contents.remove(removal_index);

            Ok(())

        }
        else
        {
        
            Err("Item not found".to_string())

        }

    }
    */

    //remove_last

    //remove_last_and_drop

    pub fn contains(&self, x: &T) -> bool
    {

        self.contents.contains(x)

    }

    pub fn len(&self) -> usize
    {

        self.contents.len()

    }

    pub fn is_empty(&self) -> bool
    {

        self.contents.is_empty()

    }

    pub fn to_vec(&self) -> Vec<T> where
        T: Clone
    {

        self.contents.to_vec()

    }

    //move to vec
    //

    pub fn clear(&mut self)
    {

        self.contents.clear();

    }

    pub fn push(&mut self, value: T) //where
        //T: Default + (Copy | Clone)
    {

        if self.contents.len() < 1
        {

            self.contents.push(value);

        }
        else
        {

            self.contents.insert(0, value);
            
        }

        /*
        let len = self.contents.len();

        if(len < 1)
        {

            self.contents.push(value);

        }
        else
        {
            
            let last_item_index = len -1;

            self.contents.push(Default::default());


            
        }
        */

    }

    pub fn push_copy(&mut self, value: &T) where
        T: Copy
    {

        if self.contents.len() < 1
        {

            self.contents.push(*value);

        }
        else
        {

            self.contents.insert(0, *value);
            
        }


    }

    pub fn push_clone(&mut self, value: &T) where
        T: Clone //+ Copy
    {

        if self.contents.len() < 1
        {

            self.contents.push(value.clone());

        }
        else
        {

            let val = value.clone();

            self.contents.insert(0, val);
            
        }


    }

    /*
    pub fn end_push(&mut self, value: T)
    {

        self.contents.push(value);

    }
    */

    pub fn pop(&mut self) -> Option<T>
    {

        self.contents.pop()

    }

    pub fn insert(&mut self, index: usize, element: T)
    {

        self.contents.insert(index, element);

    }

    pub fn remove_at(&mut self, index: usize) -> T
    {

        self.contents.remove(index)

    }

    pub fn first(&self) -> Option<&T>
    {

        self.contents.first()

    }

    pub fn first_mut(&mut self) -> Option<&mut T>
    {

        self.contents.first_mut()

    }

    pub fn last(&self) -> Option<&T>
    {

        self.contents.last()

    }

    pub fn last_mut(&mut self) -> Option<&mut T>
    {

        self.contents.last_mut()

    }

    pub fn iter(&self) -> Iter<'_, T>
    {

        self.contents.iter()

    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T>
    {

        self.contents.iter_mut()

    }

    pub fn get_last_index(&self) -> Option<usize>
    {

        if self.contents.is_empty()
        {

            return None;

        }

        Some(self.contents.len() - 1)

    }

    pub fn index_of(&self, value: &T) -> Option<usize>
    {

        let mut index: usize = 0;

        for item in self.contents.iter()
        {

            if item == value
            {

                return Some(index);

            }

            index += 1;

        }

        None

    }

    pub fn reverse(&mut self)
    {

        self.contents.reverse()

    }

    pub fn get_item(&self, index: usize) -> Option<&T>
    {

        let mut current_index: usize = 0;

        for item in self.contents.iter()
        {

            if current_index == index
            {

                return Some(item);

            }

            current_index += 1;

        }

        None

    }

    pub fn get_item_mut(&mut self, index: usize) -> Option<&mut T>
    {

        let mut current_index: usize = 0;

        for item in self.contents.iter_mut()
        {

            if current_index == index
            {

                return Some(item);

            }

            current_index += 1;

        }

        None

    }

    pub fn get<I>(&self, index: I) -> Option<&<I as SliceIndex<[T]>>::Output>
        where I: SliceIndex<[T]>
    {

        self.contents.get(index)

    }

    pub fn get_mut<I>(&mut self, index: I) -> Option<&mut <I as SliceIndex<[T]>>::Output>
        where I: SliceIndex<[T]>
    {

        self.contents.get_mut(index)

    }

    /*
    pub fn index_of_value(&self, value: T) -> Option<usize>
    {

        let mut index: usize = 0;

        for item in self.contents.iter()
        {

            if item.eq(value)
            {

                return Some(index);

            }

            index += 1;

        }

        None

    }
    */

    pub fn drain<R>(&mut self, range: R) -> Drain<'_, T> //, A>
        where R: RangeBounds<usize>
    {

        self.contents.drain(range)

    }

    pub fn drain_all(&mut self) -> Drain<'_, T>
    {

        self.contents.drain(..)

    }

}

impl<T> List<T> where
    T: Copy + PartialEq
{

    pub fn add_or_repace_copy(&mut self, value: T) -> Option<T>
    {

        let mut index = 0;

        let mut  has_found_index = false;

        for item in self.contents.iter()
        {

            if value.eq(item)
            {

                has_found_index = true;

                break;

            }

            index += 1;

        }

        if has_found_index
        {

            let return_value = self.contents[index];

            self.contents[index] = value;

            return Some(return_value);

        }
        else 
        {
       
            self.contents.push(value);   

        }

        None
        
    }

}

impl<T> List<T> where
    T: Clone + PartialEq
{

    pub fn resize(&mut self, new_len: usize, value: T)
    {

        self.contents.resize(new_len, value)

    }

    /*
    pub fn reverse(&mut self)
    {

        self.contents.reverse()

    }
    */

    pub fn add_or_repace_clone(&mut self, value: T) -> Option<T>
    {

        let mut index = 0;

        let mut  has_found_index = false;

        for item in self.contents.iter()
        {

            if value.eq(item)
            {

                has_found_index = true;

                break;

            }

            index += 1;

        }

        if has_found_index
        {

            let return_value = self.contents[index].clone();

            self.contents[index] = value;

            return Some(return_value);

        }
        else 
        {
       
            self.contents.push(value);   

        }

        None

        //return something?
        
    }

}

impl<T> Index<usize> for List<T> where
    T: PartialEq
{

    type Output = T;

    fn index(&self, index: usize) -> &Self::Output
    {
        
        self.contents.index(index)

    }

}

impl<T> IndexMut<usize> for List<T> where
    T: PartialEq
{

    fn index_mut(&mut self, index: usize) -> &mut Self::Output
    {
        
        self.contents.index_mut(index)

    }

}


impl<T> Clone for List<T> where
    T: PartialEq + Clone
{

    fn clone(&self) -> Self
    {

        Self
        { 
            
            contents: self.contents.clone()
        
        }

    }

}



