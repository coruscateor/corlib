use std::{rc::{Rc, Weak}, sync::Arc};

pub fn up_rc<T, F>(weak: &Weak<T>, mut func: F) -> bool
    where F: FnMut(Rc<T>)
{

    if let Some(this) = weak.upgrade()
    {

        func(this);

        return true;

    }

    false

}

pub fn up_arc<T, F>(weak: &std::sync::Weak<T>, mut func: F) -> bool
    where F: FnMut(Arc<T>)
{

    if let Some(this) = weak.upgrade()
    {

        func(this);

        return true;

    }

    false

}

//Returning Values

pub fn up_rc_r<T, F, R>(weak: &Weak<T>, mut func: F) -> (bool, R)
    where F: FnMut(Rc<T>) -> R,
          R: Default
{

    if let Some(this) = weak.upgrade()
    {

        return (true, func(this));

    }

    (false, R::default())

}

pub fn up_arc_r<T, F, R>(weak: &std::sync::Weak<T>, mut func: F) -> (bool, R)
    where F: FnMut(Arc<T>) -> R,
          R: Default
{

    if let Some(this) = weak.upgrade()
    {

        return (true, func(this));

    }

    (false, R::default())

}

//Options

pub fn up_rc_opt<T, F, R>(weak: &Weak<T>, mut func: F) -> (bool, Option<R>)
    where F: FnMut(Rc<T>) -> Option<R>,
          R: Default
{

    if let Some(this) = weak.upgrade()
    {

        return (true, func(this));

    }

    (false, None)

}


pub fn up_arc_opt<T, F, R>(weak: &std::sync::Weak<T>, mut func: F) -> (bool, Option<R>)
    where F: FnMut(Arc<T>) -> Option<R>,
          R: Default
{

    if let Some(this) = weak.upgrade()
    {

        return (true, func(this));

    }

    (false, None)

}
