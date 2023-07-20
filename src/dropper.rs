use std::ops::{FnMut, Drop};

//pass container objects

///
/// Contains a closure that is called when dropped.
/// 
pub struct Dropper<T>
    where T: FnMut()
{

    f: T

}

impl<T> Dropper<T>
    where T: FnMut()
{

    pub fn new(f: T) -> Self
    {

        Self
        {

            f

        }

    }

}

impl<T> Drop for Dropper<T>
    where T: FnMut()
{

    fn drop(&mut self) {
        
        (self.f)()

    }

}

//Parameterised Dropper

///
/// Contains a closure and a paramter. The closure is called with parameter being passed to it when dropped.
/// 
pub struct ParamDropper<T, P>
    where T: FnMut(&P)
{

    f: T,
    p: P

}

impl<T, P> ParamDropper<T, P>
    where T: FnMut(&P)
{

    pub fn new(f: T, p: P) -> Self
    {

        Self
        {

            f,
            p

        }

    }

    pub fn param(&self) -> &P
    {

        &self.p

    }

    pub fn param_mut(&mut self) -> &mut P
    {

        &mut self.p

    }

}

impl<T, P> Drop for ParamDropper<T, P>
    where T: FnMut(&P)
{

    fn drop(&mut self) {
        
        (self.f)(&self.p)

    }

}

//

///
/// Contains a closure and a container reference parameter. The closure is called with the container parameter being passed to it when dropped.
/// The container parameter would probably be referening to a containing object.
/// 
pub struct ContainerDropper<'a, C, T>
    where T: FnMut(&C)
{

    f: T,
    c: &'a C

}

impl<'a, C, T> ContainerDropper<'a, C, T>
    where T: FnMut(&C)
{

    pub fn new(f: T, c: &'a C) -> Self
    {

        Self
        {

            f,
            c

        }

    }

}

///
/// Contains a closure, a value parameter and a container reference parameter. The closure is called with the container and value parameters being passed to it when dropped.
/// The container parameter would probably be referening to a containing object.
/// 
impl<'a, C, T> Drop for ContainerDropper<'a, C, T>
    where T: FnMut(&C)
{

    fn drop(&mut self) {
        
        (self.f)(self.c)

    }

}

//

pub struct  ContainerParamDropper<'a, C, T, P>
    where T: FnMut(&C, &P)
{

    f: T,
    c: &'a C,
    p: P

}

impl<'a, C, T, P> ContainerParamDropper<'a, C, T, P>
    where T: FnMut(&C, &P)
{

    pub fn new(f: T, c: &'a C, p: P) -> Self
    {

        Self
        {

            f,
            c,
            p

        }

    }

}

impl<'a, C, T, P> Drop for ContainerParamDropper<'a, C, T, P>
    where T: FnMut(&C, &P)
{

    fn drop(&mut self) {
        
        (self.f)(self.c, &self.p)

    }

}

