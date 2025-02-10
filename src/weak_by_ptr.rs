use std::{fmt::Debug, rc::{Rc, Weak}};

use crate::rc_by_ptr::RcByPtr;

use std::hash::{Hash, Hasher};

/// A Container for comparing and hashing weak reference counted values using their pointers.
pub struct WeakByPtr<T: ?Sized>
{

    contents: Weak<T>

}

impl<T: ?Sized> WeakByPtr<T>
{

    pub fn new(contents: &Weak<T>) -> Self
    {

        Self
        {

            contents: contents.clone()

        }

    }

    pub fn from_rc(rc_contents: &Rc<T>) -> Self
    {

        Self
        {

            contents: Rc::downgrade(rc_contents)

        }

    }

    pub fn from_clone(contents: &Weak<T>) -> Self
    {

        Self
        {

            contents: contents.clone()

        }

    }

    pub fn contents(&self) -> Weak<T>
    {

        self.contents.clone()

    }
    
    pub fn contents_ref(&self) -> &Weak<T>
    {

        &self.contents

    }

    pub fn upgrade_contents(&self) -> Option<Rc<T>>
    {

        self.contents.upgrade()

    }

    pub fn upgrade(&self) -> Option<RcByPtr<T>>
    {

        if let Some(rc) = self.contents.upgrade()
        {

            Some(RcByPtr::new(&rc))

        }
        else
        {

            None
            
        }

    }

    pub fn strong_count(&self) -> usize
    {

        self.contents.strong_count()

    }

    pub fn weak_count(&self) -> usize
    {

        self.contents.weak_count()

    }

    pub fn take(self) -> Weak<T>
    {

        self.contents

    }

}

impl<T: ?Sized> PartialEq for WeakByPtr<T>
{

    fn eq(&self, other: &Self) -> bool
    {

        self.contents.ptr_eq(other.contents_ref())

    }

}

impl<T: ?Sized> Eq for WeakByPtr<T> {}

impl<T: ?Sized> Hash for WeakByPtr<T>
{

    fn hash<H: Hasher>(&self, state: &mut H)
    {

        Weak::as_ptr(&self.contents).hash(state);

    }

}

//Must do it this way or Clone might not get implemented.

impl<T: ?Sized> Clone for WeakByPtr<T>
{

    fn clone(&self) -> Self
    {

        Self
        {

            contents: self.contents.clone()
        
        }

    }

}

impl<T: ?Sized> PartialOrd for WeakByPtr<T>
{

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering>
    {

        let left = Weak::as_ptr(&self.contents).cast::<()>();

        let right = Weak::as_ptr(other.contents_ref()).cast::<()>();

        left.partial_cmp(&right)

    }

}

impl<T: ?Sized> Ord for WeakByPtr<T>
{

    fn cmp(&self, other: &Self) -> std::cmp::Ordering
    {
        
        let left = Weak::as_ptr(&self.contents).cast::<()>();

        let right = Weak::as_ptr(other.contents_ref()).cast::<()>();

        left.cmp(&right)
        
    }

}

impl<T> Debug for WeakByPtr<T>
    where T: ?Sized + Debug
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WeakByPtr").field("contents", &self.contents).finish()
    }
    
}