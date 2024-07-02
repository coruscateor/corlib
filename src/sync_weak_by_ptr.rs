use std::sync::{Arc, Weak};

use crate::ArcByPtr;

use std::hash::{Hash, Hasher};

use std::fmt::Debug;

/// A Container for comparing and hashing weak reference counted values using their pointers.
pub struct SyncWeakByPtr<T: ?Sized>
{

    contents: Weak<T>

}

impl<T: ?Sized> SyncWeakByPtr<T>
{

    pub fn new(contents: &Weak<T>) -> Self
    {

        Self
        {

            contents: contents.clone()

        }

    }

    pub fn from_arc(rc_contents: &Arc<T>) -> Self
    {

        Self
        {

            contents: Arc::downgrade(rc_contents)

        }

    }

    pub fn from_clone(contents: &Weak<T>) -> Self
    {

        Self
        {

            contents: contents.clone()

        }

    }

    pub fn contents(&self) -> &Weak<T>
    {

        &self.contents

    }

    pub fn upgrade_contents(&self) -> Option<Arc<T>>
    {

        self.contents.upgrade()

    }

    pub fn upgrade(&self) -> Option<ArcByPtr<T>>
    {

        if let Some(rc) = self.contents.upgrade()
        {

            Some(ArcByPtr::new(&rc))

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

impl<T: ?Sized> PartialEq for SyncWeakByPtr<T>
{

    fn eq(&self, other: &Self) -> bool
    {

        self.contents.ptr_eq(other.contents())

    }

}

impl<T: ?Sized> Eq for SyncWeakByPtr<T> {}

impl<T: ?Sized> Hash for SyncWeakByPtr<T>
{

    fn hash<H: Hasher>(&self, state: &mut H)
    {

        Weak::as_ptr(&self.contents).hash(state);

    }

}

//Must do it this way or Clone might not get implemented.

impl<T: ?Sized> Clone for SyncWeakByPtr<T>
{

    fn clone(&self) -> Self
    {

        Self
        {

            contents: self.contents.clone()
        
        }

    }

}

impl<T: ?Sized> PartialOrd for SyncWeakByPtr<T>
{

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering>
    {

        let left = Weak::as_ptr(&self.contents).cast::<()>();

        let right = Weak::as_ptr(other.contents()).cast::<()>();

        left.partial_cmp(&right)

    }

}

impl<T: ?Sized> Ord for SyncWeakByPtr<T>
{

    fn cmp(&self, other: &Self) -> std::cmp::Ordering
    {
        
        let left = Weak::as_ptr(&self.contents).cast::<()>();

        let right = Weak::as_ptr(other.contents()).cast::<()>();

        left.cmp(&right)
        
    }

}

impl<T> Debug for SyncWeakByPtr<T>
    where T: ?Sized + Debug
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SyncWeakByPtr").field("contents", &self.contents).finish()
    }
    
}
