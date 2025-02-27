//!
//! Functions that help you deal with upgrading reference counted objects.
//! 

use std::{future::Future, rc::{Rc, Weak}, sync::Arc};

static ERROR_MESSAGE: &str = "Error: Upgrade Failed";

///
/// Attempts to upgrade the provided weak Rc reference, calling the provided func object if successful.
/// 
pub fn try_up_rc<T, F>(weak: &Weak<T>, mut func: F) -> bool
    where F: FnMut(Rc<T>)
{

    if let Some(this) = weak.upgrade()
    {

        func(this);

        return true;

    }

    false

}

pub fn up_rc<T, F>(weak: &Weak<T>, func: F)
    where F: FnMut(Rc<T>)
{

    if !try_up_rc(weak, func)
    {

        panic!("{}", ERROR_MESSAGE)

    }

}

///
/// Attempts to upgrade the provided weak Rc reference, calling the provided func object and returning its bool result if successful.
/// 
pub fn try_up_rc_pt<T, F>(weak: &Weak<T>, mut func: F) -> bool
    where F: FnMut(Rc<T>) -> bool
{

    if let Some(this) = weak.upgrade()
    {

        return func(this);

    }

    false

}

pub fn up_rc_pt<T, F>(weak: &Weak<T>, func: F)
    where F: FnMut(Rc<T>) -> bool
{

    if !try_up_rc_pt(weak, func)
    {

        panic!("{}", ERROR_MESSAGE)

    }

}

///
/// Attempts to upgrade the provided weak Arc reference, calling the provided func object if successful.  
/// 
pub fn try_up_arc<T, F>(weak: &std::sync::Weak<T>, mut func: F) -> bool
    where F: FnMut(Arc<T>)
{

    if let Some(this) = weak.upgrade()
    {

        func(this);

        return true;

    }

    false

}

pub fn up_arc<T, F>(weak: &std::sync::Weak<T>, func: F)
    where F: FnMut(Arc<T>)
{

    if !try_up_arc(weak, func)
    {

        panic!("{}", ERROR_MESSAGE)

    }

}

///
/// Attempts to upgrade the provided weak Arc reference, calling the provided func object if successful.  
/// 
pub fn try_up_arc_pt<T, F>(weak: &std::sync::Weak<T>, mut func: F) -> bool
    where F: FnMut(Arc<T>) -> bool
{

    if let Some(this) = weak.upgrade()
    {

        return func(this);

    }

    false

}

pub fn up_arc_pt<T, F>(weak: &std::sync::Weak<T>, func: F)
    where F: FnMut(Arc<T>) -> bool
{

    if !try_up_arc_pt(weak, func)
    {

        panic!("{}", ERROR_MESSAGE)

    }

}

//Async

pub async fn try_up_arc_async<T, F, FUT>(weak: &std::sync::Weak<T>, mut func: F) -> bool
    where F: FnMut(Arc<T>) -> FUT,
          FUT: Future<Output = ()>
{

    if let Some(this) = weak.upgrade()
    {

        func(this).await;

        return true;

    }

    false

}

pub async fn try_up_arc_pt_async<T, F, FUT>(weak: &std::sync::Weak<T>, mut func: F) -> bool
    where F: FnMut(Arc<T>) -> FUT,
          FUT: Future<Output = bool>
{

    if let Some(this) = weak.upgrade()
    {

        return func(this).await;

    }

    false

}

//Returning Values

pub fn try_up_rc_r<T, F, R>(weak: &Weak<T>, mut func: F) -> (bool, R)
    where F: FnMut(Rc<T>) -> R,
          R: Default
{

    if let Some(this) = weak.upgrade()
    {

        return (true, func(this));

    }

    (false, R::default())

}

pub fn try_up_arc_r<T, F, R>(weak: &std::sync::Weak<T>, mut func: F) -> (bool, R)
    where F: FnMut(Arc<T>) -> R,
          R: Default
{

    if let Some(this) = weak.upgrade()
    {

        return (true, func(this));

    }

    (false, R::default())

}

//Async

pub async fn try_up_arc_r_async<T, F, R, FUT>(weak: &std::sync::Weak<T>, mut func: F) -> (bool, R)
    where F: FnMut(Arc<T>) -> FUT,
          R: Default,
          FUT: Future<Output = R>
{

    if let Some(this) = weak.upgrade()
    {

        return (true, func(this).await);

    }

    (false, R::default())

}

//Options

pub fn try_up_rc_opt<T, F, R>(weak: &Weak<T>, mut func: F) -> (bool, Option<R>)
    where F: FnMut(Rc<T>) -> Option<R>
{

    if let Some(this) = weak.upgrade()
    {

        return (true, func(this));

    }

    (false, None)

}

pub fn try_up_arc_opt<T, F, R>(weak: &std::sync::Weak<T>, mut func: F) -> (bool, Option<R>)
    where F: FnMut(Arc<T>) -> Option<R>
{

    if let Some(this) = weak.upgrade()
    {

        return (true, func(this));

    }

    (false, None)

}

//Async

pub async fn try_up_arc_opt_async<T, F, R, FUT>(weak: &std::sync::Weak<T>, mut func: F) -> (bool, Option<R>)
    where F: FnMut(Arc<T>) -> FUT,
          FUT: Future<Output = Option<R>>
{

    if let Some(this) = weak.upgrade()
    {

        return (true, func(this).await);

    }

    (false, None)

}

//Only Options

pub fn try_up_rc_opt_only<T, F, R>(weak: &Weak<T>, mut func: F) -> Option<R>
    where F: FnMut(Rc<T>) -> Option<R>
{

    if let Some(this) = weak.upgrade()
    {

        return func(this);

    }

    None

}

pub fn try_up_arc_opt_only<T, F, R>(weak: &std::sync::Weak<T>, mut func: F) -> Option<R>
    where F: FnMut(Arc<T>) -> Option<R>
{

    if let Some(this) = weak.upgrade()
    {

        return func(this);

    }

    None

}

//Async

pub async fn try_up_arc_opt_only_async<T, F, R, FUT>(weak: &std::sync::Weak<T>, mut func: F) -> Option<R>
    where F: FnMut(Arc<T>) -> FUT,
          FUT: Future<Output = Option<R>>
{

    if let Some(this) = weak.upgrade()
    {

        return func(this).await;

    }

    None

}

//Success Options

pub fn try_up_rc_opt_success<T, F, R>(weak: &Weak<T>, mut func: F) -> Option<R>
    where F: FnMut(Rc<T>) -> R
{

    if let Some(this) = weak.upgrade()
    {

        return Some(func(this));

    }

    None

}

pub fn try_up_arc_opt_success<T, F, R>(weak: &std::sync::Weak<T>, mut func: F) -> Option<R>
    where F: FnMut(Arc<T>) -> R
{

    if let Some(this) = weak.upgrade()
    {

        return Some(func(this));

    }

    None

}

//Async

pub async fn try_up_arc_opt_success_async<T, F, R, FUT>(weak: &std::sync::Weak<T>, mut func: F) -> Option<R>
    where F: FnMut(Arc<T>) -> FUT,
          FUT: Future<Output = R>
{

    if let Some(this) = weak.upgrade()
    {

        return Some(func(this).await);

    }

    None

}

