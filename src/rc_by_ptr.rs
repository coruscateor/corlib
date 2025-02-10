use std::rc::{Rc, Weak};

use crate::WeakByPtr;

use std::hash::{Hash, Hasher};

use std::fmt::Debug;

/// A Container for comparing and hashing reference counted values using their pointers.
pub struct RcByPtr<T: ?Sized>
{

    contents: Rc<T>

}

impl<T: ?Sized> RcByPtr<T>
{

    pub fn new(contents: &Rc<T>) -> Self
    {

        Self
        {

            contents: contents.clone()

        }

    }

    pub fn from_weak(rc_contents: &Weak<T>) -> Option<Self>
    {

        if let Some(rc) = rc_contents.upgrade()
        {

            Some(Self
            {
    
                contents: rc
    
            })

        }
        else
        {
         
            None
            
        }

    }

    pub fn from_clone(contents: &Rc<T>) -> Self
    {

        Self
        {

            contents: contents.clone()

        }

    }

    pub fn contents(&self) -> Rc<T>
    {

        self.contents.clone()

    }

    pub fn contents_ref(&self) -> &Rc<T>
    {

        &self.contents

    }

    pub fn downgrade_contents(&self) -> Weak<T>
    {

        Rc::downgrade(&self.contents)

    }

    pub fn downgrade(&self) -> WeakByPtr<T>
    {

        WeakByPtr::new(&Rc::downgrade(&self.contents))

    }

    pub fn take(self) -> Rc<T>
    {

        self.contents

    }

}

impl<T: ?Sized> PartialEq for RcByPtr<T>
{

    fn eq(&self, other: &Self) -> bool
    {

        Rc::ptr_eq(&self.contents, other.contents_ref())

    }

}

impl<T: ?Sized> Eq for RcByPtr<T> {}

impl<T: ?Sized> Hash for RcByPtr<T>
{

    fn hash<H: Hasher>(&self, state: &mut H)
    {

        Rc::as_ptr(&self.contents).hash(state);

    }

}

//#[derive(Clone)] makes it so every generic pramerter down must implement clone for the clone trait implementation to be part of the compiled object. Weird

impl<T: ?Sized> Clone for RcByPtr<T>
{

    fn clone(&self) -> Self
    {

        Self
        {

            contents: self.contents.clone()
        
        }

    }

}

impl<T: ?Sized> PartialOrd for RcByPtr<T>
{

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering>
    {

        //self.contents.partial_cmp(&other.contents)
    
        //Rc::partial_cmp(&self, other)

        let left = Rc::as_ptr(&self.contents).cast::<()>(); //.partial_cmp(other)

        let right = Rc::as_ptr(other.contents_ref()).cast::<()>();

        left.partial_cmp(&right)

    }

}

impl<T: ?Sized> Ord for RcByPtr<T>
{

    fn cmp(&self, other: &Self) -> std::cmp::Ordering
    {
        
        let left = Rc::as_ptr(&self.contents).cast::<()>();

        let right = Rc::as_ptr(other.contents_ref()).cast::<()>();

        left.cmp(&right)
        
    }

}

impl<T> Debug for RcByPtr<T>
    where T: ?Sized + Debug
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RcByPtr").field("contents", &self.contents).finish()
    }
}
