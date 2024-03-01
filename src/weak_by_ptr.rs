use std::rc::{Rc, Weak};

use crate::rc_by_ptr::RcByPtr;

use std::hash::{Hash, Hasher};

/// A Container for comparing and hashing weakly-reference counted values by reference.
//#[derive(Clone)]
pub struct WeakByPtr<T: ?Sized> //: ?Sized
{

    contents: Weak<T>

}

impl<T: ?Sized> WeakByPtr<T> //: ?Sized
{

    pub fn new(contents: Weak<T>) -> Self
    {

        Self
        {

            contents

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

    /*
    pub fn get_contents_ref(&self) -> &Weak<T>
    {

        &self.contents

    }
    */

    pub fn contents(&self) -> &Weak<T>
    {

        &self.contents

    }

    pub fn upgrade(&self) -> Option<Rc<T>>
    {

        self.contents.upgrade()

    }

    pub fn upgrade_eq(&self) -> Option<RcByPtr<T>>
    {

        if let Some(rc) = self.contents.upgrade()
        {

            Some(RcByPtr::new(rc))

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

}

impl<T: ?Sized> PartialEq for WeakByPtr<T> //: ?Sized
{

    fn eq(&self, other: &Self) -> bool
    {

        self.contents.ptr_eq(other.contents()) //get_contents_ref())

    }


}

impl<T: ?Sized> Eq for WeakByPtr<T> {} //: ?Sized

impl<T: ?Sized> Hash for WeakByPtr<T>
{

    fn hash<H: Hasher>(&self, state: &mut H)
    {

        Weak::as_ptr(&self.contents).hash(state);

    }

}

//Must do it this way or Clone mught not get implemented.

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
