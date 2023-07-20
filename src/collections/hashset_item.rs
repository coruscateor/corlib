use std::hash::{Hash, Hasher};

use std::rc::{Rc, Weak};

use std::cell::RefCell;

//deref/drefmut

pub struct HashsetItem<T>
{

    item: T

}

impl<T> HashsetItem<T>
{

    pub fn new(item: T) -> Self
    {

        Self
        {

            item

        }

    }

}

impl<T> PartialEq for HashsetItem<Rc<T>>
    where T: PartialEq
{

    fn eq(&self, other: &Self) -> bool
    {

        self == other

    }

}

impl<T> Eq for HashsetItem<Rc<T>>
    where T: PartialEq
{}


impl<'a, T> Hash for HashsetItem<Rc<T>>
    where T: Hash + HasStrId,

{

    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher
    {

        self.item.get_id().hash(state);

    }

}

/*
impl<T> PartialEq for HashsetItem<Rc<RefCell<T>>>
where T: PartialEq
{

    fn eq(&self, other: &Self) -> bool
    {

        self == other

    }

}

impl<T> Eq for HashsetItem<Rc<RefCell<T>>>
    where T: PartialEq
{}
*/

impl<'a, T> Hash for HashsetItem<Rc<RefCell<T>>>
    where T: Hash + HasStrId,

{

    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher
    {

        self.item.borrow().get_id().hash(state);

        //self.hash(state);

    }

}

pub trait HasStrId //<'a>
{

    fn get_id(&self) -> &str; //'a str;

}

/*
impl<T> PartialEq for HashsetItem<T>
{

    fn eq(&self, other: &Self) -> bool
    {

        &self.window == &other.window

    }

}

impl<T> Hash for HashsetItem<T>
{

    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher
    {

        self.id.hash(state);

    }

}
*/

