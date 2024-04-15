//use std::borrow::{Borrow, BorrowMut};

use crate::{MutSelfGetRef, GetMut, TryGetRef, TryGetMut};

use std::ops::Fn;

///
/// A lazily constructed object that does not contain an initaising function pointer.
/// 
#[derive(Default)]
pub struct Lazy<T> //ByDefault //: Default
{

    value: Option<T>

}

impl<T> Lazy<T>  //: Default
{

    pub const fn new() -> Self
    {

        //Self::default()

        Self
        {

            value: None

        }

    }

    pub fn drop_value(&mut self)
    {

        self.value = None;

    }

    pub fn try_take(&mut self) -> Option<T>
    {

        self.value.take()        

    }

    pub fn get_ref_fn<F>(&mut self, func: &F) -> &T
        where F: Fn() -> T
    {

        if self.value.is_none()
        {

            self.value = Some(func());

        }

        self.value.as_ref().unwrap()

    }

    pub fn get_ref_fn_param<F, P>(&mut self, func: &F, param: &P) -> &T
        where F: Fn(&P) -> T
    {

        if self.value.is_none()
        {

            self.value = Some(func(param));

        }

        self.value.as_ref().unwrap()

    }

    pub fn get_mut_fn<F>(&mut self, func: &F) -> &T
        where F: Fn() -> T
    {

        if self.value.is_none()
        {

            self.value = Some(func());

        }

        self.value.as_mut().unwrap()

    }

    pub fn get_mut_fn_param<F, P>(&mut self, func: &F, param: &P) -> &T
        where F: Fn(&P) -> T
    {

        if self.value.is_none()
        {

            self.value = Some(func(param));

        }

        self.value.as_mut().unwrap()

    }

    /*
    fn get_ref_call(&mut self) -> &T
    {


    }
    */
    
}

impl<T: Default> Lazy<T>
{

    pub fn init_value(&mut self)
    {

        self.value = Some(T::default());

    }

    fn check_is_init(&mut self) //-> bool
    {

        if self.value.is_none()
        {

            self.init_value();

            //return true;

        }

        //false

    }

    pub fn try_re_init_value(&mut self) -> bool
    {

        if self.value.is_some()
        {

            self.init_value();

            true

        }
        else
        {
            
            false

        }

    }

    pub fn take(&mut self) -> T
    {

        self.check_is_init();

        self.value.take().unwrap()

    }

}

impl<T: Default> MutSelfGetRef<T> for Lazy<T> //ByDefault
{

    fn get_ref(&mut self) -> &T
    {
        
        self.check_is_init();
    
        self.value.as_ref().unwrap()

    }

}

impl<T: Default> GetMut<T> for Lazy<T> //ByDefault
{

    fn get_mut(&mut self) -> &mut T
    {

        self.check_is_init();
    
        self.value.as_mut().unwrap()

    }

}

impl<T: Default> TryGetRef<T> for Lazy<T> //ByDefault
{

    fn try_get_ref(&self) -> Option<&T>
    {
    
        self.value.as_ref()

    }

}

impl<T: Default> TryGetMut<T> for Lazy<T> //ByDefault
{

    fn try_get_mut(&mut self) -> Option<&mut T>
    {
    
        self.value.as_mut()

    }

}
