use std::{borrow::Cow, rc::{Rc, Weak}, sync::Arc};

use std::fmt::Debug;

pub enum BoxOrRc<T>
{

    Box(Box<T>),
    Rc(Rc<T>)

}

impl<T> Debug for BoxOrRc<T>
    where T: Debug
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Box(arg0) => f.debug_tuple("Box").field(arg0).finish(),
            Self::Rc(arg0) => f.debug_tuple("Rc").field(arg0).finish(),
        }
    }

}

pub enum BoxOrRcWeak<T>
{

    Box(Box<T>),
    Rc(Rc<T>),
    Weak(Weak<T>)

}

impl<T> Debug for BoxOrRcWeak<T>
    where T: Debug
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Box(arg0) => f.debug_tuple("Box").field(arg0).finish(),
            Self::Rc(arg0) => f.debug_tuple("Rc").field(arg0).finish(),
            Self::Weak(arg0) => f.debug_tuple("Weak").field(arg0).finish(),
        }
    }

}

pub enum BoxOrWeak<T>
{

    Box(Box<T>),
    Weak(Weak<T>)

}

impl<T> Debug for BoxOrWeak<T>
    where T: Debug
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Box(arg0) => f.debug_tuple("Box").field(arg0).finish(),
            Self::Weak(arg0) => f.debug_tuple("Weak").field(arg0).finish(),
        }
    }

}

pub enum BoxOrRcs<T>
{

    Box(Box<T>),
    Rc(Rc<T>),
    Arc(Arc<T>)

}

impl<T> Debug for BoxOrRcs<T>
    where T: Debug
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Box(arg0) => f.debug_tuple("Box").field(arg0).finish(),
            Self::Rc(arg0) => f.debug_tuple("Rc").field(arg0).finish(),
            Self::Arc(arg0) => f.debug_tuple("Arc").field(arg0).finish(),
        }
    }

}

pub enum BoxOrRcsWeaks<T>
{

    Box(Box<T>),
    Rc(Rc<T>),
    RcWeak(Weak<T>),
    Arc(Arc<T>),
    ArcWeak(std::sync::Weak<T>)

}

impl<T> Debug for BoxOrRcsWeaks<T>
    where T: Debug
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Box(arg0) => f.debug_tuple("Box").field(arg0).finish(),
            Self::Rc(arg0) => f.debug_tuple("Rc").field(arg0).finish(),
            Self::RcWeak(arg0) => f.debug_tuple("RcWeak").field(arg0).finish(),
            Self::Arc(arg0) => f.debug_tuple("Arc").field(arg0).finish(),
            Self::ArcWeak(arg0) => f.debug_tuple("ArcWeak").field(arg0).finish(),
        }
    }

}


pub enum Weakness<T>
{

    RcWeak(Weak<T>),
    ArcWeak(std::sync::Weak<T>)

}

impl<T> Debug for Weakness<T>
    where T: Debug
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RcWeak(arg0) => f.debug_tuple("RcWeak").field(arg0).finish(),
            Self::ArcWeak(arg0) => f.debug_tuple("ArcWeak").field(arg0).finish(),
        }
    }

}

pub enum BoxOrRcCow<'a, T: Clone>
{

    Box(Box<T>),
    Rc(Rc<T>),
    Cow(Cow<'a, T>)

}

impl<'a, T> Debug for BoxOrRcCow<'a, T>
    where T: Clone + Debug
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Box(arg0) => f.debug_tuple("Box").field(arg0).finish(),
            Self::Rc(arg0) => f.debug_tuple("Rc").field(arg0).finish(),
            Self::Cow(arg0) => f.debug_tuple("Cow").field(arg0).finish(),
        }
    }

}

pub enum BoxOrRcsCow<'a, T: Clone>
{

    Box(Box<T>),
    Rc(Rc<T>),
    Arc(Arc<T>),
    Cow(Cow<'a, T>)

}

impl<'a, T> Debug for BoxOrRcsCow<'a, T>
    where T: Clone + Debug
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Box(arg0) => f.debug_tuple("Box").field(arg0).finish(),
            Self::Rc(arg0) => f.debug_tuple("Rc").field(arg0).finish(),
            Self::Arc(arg0) => f.debug_tuple("Arc").field(arg0).finish(),
            Self::Cow(arg0) => f.debug_tuple("Cow").field(arg0).finish(),
        }
    }

}

pub enum ArcCow<'a, T: Clone>
{

    Arc(Arc<T>),
    Cow(Cow<'a, T>)

}

impl<'a, T> Debug for ArcCow<'a, T>
    where T: Clone + Debug
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Arc(arg0) => f.debug_tuple("Arc").field(arg0).finish(),
            Self::Cow(arg0) => f.debug_tuple("Cow").field(arg0).finish(),
        }
    }

}


