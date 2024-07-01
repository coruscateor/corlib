use std::{borrow::Cow, rc::{Rc, Weak}, sync::Arc};

pub enum BoxOrRc<T>
{

    Box(Box<T>),
    Rc(Rc<T>)

}

pub enum BoxOrRcWeak<T>
{

    Box(Box<T>),
    Rc(Rc<T>),
    Weak(Weak<T>)

}

pub enum BoxOrWeak<T>
{

    Box(Box<T>),
    Weak(Weak<T>)

}

pub enum BoxOrRcs<T>
{

    Box(Box<T>),
    Rc(Rc<T>),
    Arc(Arc<T>)

}

pub enum BoxOrRcsWeaks<T>
{

    Box(Box<T>),
    Rc(Rc<T>),
    RcWeak(Weak<T>),
    Arc(Arc<T>),
    ArcWeak(std::sync::Weak<T>)

}

pub enum Weakness<T>
{

    RcWeak(Weak<T>),
    ArcWeak(std::sync::Weak<T>)

}

pub enum BoxOrRcCow<'a, T: Clone>
{

    Box(Box<T>),
    Rc(Rc<T>),
    Cow(Cow<'a, T>)

}

pub enum BoxOrRcsCow<'a, T: Clone>
{

    Box(Box<T>),
    Rc(Rc<T>),
    Arc(Arc<T>),
    Cow(Cow<'a, T>)

}

pub enum ArcCow<'a, T: Clone>
{

    Arc(Arc<T>),
    Cow(Cow<'a, T>)

}

