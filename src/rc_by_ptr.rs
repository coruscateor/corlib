use std::rc::{Rc, Weak};

use crate::WeakByPtr;

use std::hash::{Hash, Hasher};

/// A Container for comparing and hashing reference counted values by reference.
//#[derive(Clone)]
pub struct RcByPtr<T: ?Sized> //: ?Sized
{

    contents: Rc<T>

}

impl<T: ?Sized> RcByPtr<T> //: ?Sized
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

    /*
    pub fn get_contents_ref(&self) -> &Rc<T>
    {

        &self.contents

    }
    */

    pub fn contents(&self) -> &Rc<T>
    {

        &self.contents

    }

    pub fn downgrade(&self) -> Weak<T>
    {

        Rc::downgrade(&self.contents)

    }

    pub fn downgrade_eq(&self) -> WeakByPtr<T>
    {

        WeakByPtr::new(&Rc::downgrade(&self.contents))

    }

    pub fn take(self) -> Rc<T>
    {

        self.contents

    }

}

impl<T: ?Sized> PartialEq for RcByPtr<T> //: ?Sized>
{

    fn eq(&self, other: &Self) -> bool
    {

        //self.contents.ptr_eq(other.get_contents_ref())

        Rc::ptr_eq(&self.contents, other.contents()) //.get_contents_ref())

    }

}

impl<T: ?Sized> Eq for RcByPtr<T> {} //: ?Sized

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