use std::sync::{Arc, Weak};

use super::SyncWeakByPtr;

use std::hash::{Hash, Hasher};

use std::fmt::Debug;

/// A Container for comparing and hashing reference counted values using their pointers.
pub struct ArcByPtr<T: ?Sized>
{

    contents: Arc<T>

}

impl<T: ?Sized> ArcByPtr<T>
{

    pub fn new(contents: &Arc<T>) -> Self
    {

        Self
        {

            contents: contents.clone()

        }

    }

    pub fn from_weak(arc_contents: &Weak<T>) -> Option<Self>
    {

        if let Some(arc) = arc_contents.upgrade()
        {

            Some(Self
            {
    
                contents: arc
    
            })

        }
        else
        {
         
            None
            
        }

    }

    pub fn from_clone(contents: &Arc<T>) -> Self
    {

        Self
        {

            contents: contents.clone()

        }

    }

    pub fn contents(&self) -> Arc<T>
    {

        self.contents.clone()

    }

    pub fn contents_ref(&self) -> &Arc<T>
    {

        &self.contents

    }

    pub fn downgrade_contents(&self) -> Weak<T>
    {

        Arc::downgrade(&self.contents)

    }

    pub fn downgrade(&self) -> SyncWeakByPtr<T>
    {

        SyncWeakByPtr::new(&Arc::downgrade(&self.contents))

    }

    pub fn take(self) -> Arc<T>
    {

        self.contents

    }

}

impl<T: ?Sized> PartialEq for ArcByPtr<T>
{

    fn eq(&self, other: &Self) -> bool
    {

        Arc::ptr_eq(&self.contents, other.contents_ref())

    }

}

impl<T: ?Sized> Eq for ArcByPtr<T> {}

impl<T: ?Sized> Hash for ArcByPtr<T>
{

    fn hash<H: Hasher>(&self, state: &mut H)
    {

        Arc::as_ptr(&self.contents).hash(state);

    }

}

//#[derive(Clone)] makes it so every generic pramerter down must implement clone for the clone trait implementation to be part of the compiled object. Weird

impl<T: ?Sized> Clone for ArcByPtr<T>
{

    fn clone(&self) -> Self
    {

        Self
        {

            contents: self.contents.clone()
        
        }

    }

}

impl<T: ?Sized> PartialOrd for ArcByPtr<T>
{

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering>
    {

        //self.contents.partial_cmp(&other.contents)
    
        //Rc::partial_cmp(&self, other)

        let left = Arc::as_ptr(&self.contents).cast::<()>(); //.partial_cmp(other)

        let right = Arc::as_ptr(other.contents_ref()).cast::<()>();

        /*
        ambiguous wide pointer comparison, the comparison includes metadata which may not be expectedrustcClick for full compiler diagnostic
        arc_by_ptr.rs(154, 13): use `std::ptr::addr_eq` or untyped pointers to only compare their addresses: `.cast::<()>()`, `.cast::<()>()`
         */

        left.partial_cmp(&right)

    }

}

impl<T: ?Sized> Ord for ArcByPtr<T>
{

    fn cmp(&self, other: &Self) -> std::cmp::Ordering
    {
        
        let left = Arc::as_ptr(&self.contents).cast::<()>();

        let right = Arc::as_ptr(other.contents_ref()).cast::<()>();

        /*
        ambiguous wide pointer comparison, the comparison includes metadata which may not be expectedrustcClick for full compiler diagnostic
        arc_by_ptr.rs(175, 13): use `std::ptr::addr_eq` or untyped pointers to only compare their addresses: `.cast::<()>()`, `.cast::<()>()`
         */

        left.cmp(&right)
        
    }

}

impl<T> Debug for ArcByPtr<T>
    where T: ?Sized + Debug
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ArcByPtr").field("contents", &self.contents).finish()
    }
    
}