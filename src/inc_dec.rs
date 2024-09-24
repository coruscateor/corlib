//!
//! This module enables you to do ++ and -- like operations on standard numeric primitives.
//! 
//! “pp” stands for “plus plus” and “mm” stands for “minus minus”, obviously.
//! 

//Incrementation

/// Increments an integer by one.
#[macro_export]
macro_rules! pp
{

    ($integer:ident) =>
    {

        $integer += 1;

    }

}

#[macro_export]
macro_rules! pp_mut
{

    ($integer:ident) =>
    {

        {

            *$integer += 1;

            *$integer

        }

    }

}

#[macro_export]
macro_rules! checked_pp_mut
{

    ($integer:ident) =>
    {

        if let Some(res) = $integer.checked_add(1)
        {

            *$integer = res;

            Some(*$integer)

        }
        else
        {

            None
            
        }

    }

}

/// Increments a floating point number by one.
#[macro_export]
macro_rules! ppf
{

    ($float:ident) =>
    {

        $float += 1.0;

    }

}

#[macro_export]
macro_rules! ppf_mut
{

    ($float:ident) =>
    {

        {

            *$float += 1.0;

            *$float

        }

    }

}

/*
macro expansion ignores token `*` and any following
the usage of `ppf_mut!` is likely invalid in expression contextrustcClick for full compiler diagnostic
inc_dec.rs(205, 9): caused by the macro expansion here
inc_dec.rs(205, 23): you might be missing a semicolon here: `;`
No quick fixes available
*/

//Decrementation

/// Decrements an integer by one.
#[macro_export]
macro_rules! mm
{

    ($integer:ident) =>
    {

        $integer -= 1;

    }

}

#[macro_export]
macro_rules! mm_mut
{

    ($integer:ident) =>
    {

        {

            *$integer -= 1;

            *$integer

        }

    }

}

/// Decrements a floating point number by one.
#[macro_export]
macro_rules! mmf
{

    ($float:ident) =>
    {

        $float -= 1.0;

    }

}

#[macro_export]
macro_rules! mmf_mut
{

    ($float:ident) =>
    {

        {

            *$float -= 1.0;

            *$float

        }

    }

}

#[macro_export]
macro_rules! checked_mm_mut
{

    ($integer:ident) =>
    {

        if let Some(res) = $integer.checked_sub(1)
        {

            *$integer = res;

            Some(*$integer)

        }
        else
        {

            None
            
        }

    }

}

pub trait IncDecSelf
    where Self: Sized + Copy
{

    fn pp(&mut self) -> Self;

    fn try_pp(&mut self) -> Option<Self>;

    fn mm(&mut self) -> Self;

    fn try_mm(&mut self) -> Option<Self>;

}

//Integers and floating-point intergers.

//f32

impl IncDecSelf for f32
{

    fn pp(&mut self) -> Self
    {

        ppf_mut!(self) //;

        //*self

        /*
        let mut val = *self;

        ppf!(val);

        *self = val;

        val
        */

    }

    fn try_pp(&mut self) -> Option<Self>
    {
        
        if *self < (f32::MAX - 0.9)
        {

            Some(ppf_mut!(self)) //;

            //Some(*self)

        }
        else
        {

            None
            
        }

    }

    fn mm(&mut self) -> Self
    {

        mmf_mut!(self) //;

        //*self

    }

    fn try_mm(&mut self) -> Option<Self>
    {
        
        if *self > (f32::MIN + 0.9)
        {

            Some(mmf_mut!(self)) //;

            //Some(*self)

        }
        else
        {

            None
            
        }

    }

}

//f64

impl IncDecSelf for f64
{

    fn pp(&mut self) -> Self
    {

        ppf_mut!(self) //;

        //*self

    }

    fn try_pp(&mut self) -> Option<Self>
    {
        
        if *self < (f64::MAX - 0.9)
        {

            Some(ppf_mut!(self)) //;

            //Some(*self)

        }
        else
        {

            None
            
        }

    }

    fn mm(&mut self) -> Self
    {

        mmf_mut!(self)  //;

        //*self

    }

    fn try_mm(&mut self) -> Option<Self>
    {
        
        if *self > (f64::MIN + 0.9)
        {

            Some(mmf_mut!(self)) //;

            //Some(*self)

        }
        else
        {

            None
            
        }
        
    }

}

//i8

impl IncDecSelf for i8
{

    fn pp(&mut self) -> Self
    {

        pp_mut!(self) //;

        //*self

    }

    fn try_pp(&mut self) -> Option<Self>
    {
        
        checked_pp_mut!(self)

        /*
        if let Some(res) = self.checked_add(1)
        {

            *self = res;

            Some(*self)

        }
        else
        {

            None
            
        }
        */

    }

    fn mm(&mut self) -> Self
    {

        mm_mut!(self) //;

        //*self

    }

    fn try_mm(&mut self) -> Option<Self>
    {
        
        checked_mm_mut!(self)

    }

}

//i16

impl IncDecSelf for i16
{

    fn pp(&mut self) -> Self
    {

        pp_mut!(self) //;

        //*self

    }

    fn try_pp(&mut self) -> Option<Self>
    {
        
        checked_pp_mut!(self)

    }

    fn mm(&mut self) -> Self
    {

        mm_mut!(self) //;

        //*self

    }

    fn try_mm(&mut self) -> Option<Self>
    {
        
        checked_mm_mut!(self)

    }


}

//i32

impl IncDecSelf for i32
{

    fn pp(&mut self) -> Self
    {

        pp_mut!(self) //;

        //*self

    }

    fn try_pp(&mut self) -> Option<Self>
    {
        
        checked_pp_mut!(self)

    }

    fn mm(&mut self) -> Self
    {

        mm_mut!(self) //;

        //*self

    }

    fn try_mm(&mut self) -> Option<Self>
    {
        
        checked_mm_mut!(self)

    }

}

//i64

impl IncDecSelf for i64
{

    fn pp(&mut self) -> Self
    {

        pp_mut!(self) //;

        //*self

    }

    fn try_pp(&mut self) -> Option<Self>
    {
        
        checked_pp_mut!(self)

    }

    fn mm(&mut self) -> Self
    {

        mm_mut!(self) //;

        //*self

    }

    fn try_mm(&mut self) -> Option<Self>
    {
        
        checked_mm_mut!(self)

    }

}

//i128

impl IncDecSelf for i128
{

    fn pp(&mut self) -> Self
    {

        pp_mut!(self) //;

        //*self

    }

    fn try_pp(&mut self) -> Option<Self>
    {
        
        checked_pp_mut!(self)

    }

    fn mm(&mut self) -> Self
    {

        mm_mut!(self) //;

        //*self

    }

    fn try_mm(&mut self) -> Option<Self>
    {
        
        checked_mm_mut!(self)

    }

}

//isize

impl IncDecSelf for isize
{

    fn pp(&mut self) -> Self
    {

        pp_mut!(self) //;

        //*self

    }

    fn try_pp(&mut self) -> Option<Self>
    {
        
        checked_pp_mut!(self)

    }

    fn mm(&mut self) -> Self
    {

        mm_mut!(self) //;

        //*self

    }

    fn try_mm(&mut self) -> Option<Self>
    {
        
        checked_mm_mut!(self)

    }

}

//u8

impl IncDecSelf for u8
{

    fn pp(&mut self) -> Self
    {

        pp_mut!(self) //;

        //*self

    }

    fn try_pp(&mut self) -> Option<Self>
    {
        
        checked_pp_mut!(self)

    }

    fn mm(&mut self) -> Self
    {

        mm_mut!(self) //;

        //*self

    }

    fn try_mm(&mut self) -> Option<Self>
    {
        
        checked_mm_mut!(self)

    }

}

//u16

impl IncDecSelf for u16
{

    fn pp(&mut self) -> Self
    {

        pp_mut!(self) //;

        //*self

    }

    fn try_pp(&mut self) -> Option<Self>
    {
        
        checked_pp_mut!(self)

    }

    fn mm(&mut self) -> Self
    {

        mm_mut!(self) //;

        //*self

    }

    fn try_mm(&mut self) -> Option<Self>
    {
        
        checked_mm_mut!(self)

    }

}

//u32

impl IncDecSelf for u32
{

    fn pp(&mut self) -> Self
    {

        pp_mut!(self) //;

        //*self

    }

    fn try_pp(&mut self) -> Option<Self>
    {
        
        checked_pp_mut!(self)

    }

    fn mm(&mut self) -> Self
    {

        mm_mut!(self) //;

        //*self

    }

    fn try_mm(&mut self) -> Option<Self>
    {
        
        checked_mm_mut!(self)

    }

}

//u64

impl IncDecSelf for u64
{

    fn pp(&mut self) -> Self
    {

        pp_mut!(self) //;

        //*self

    }

    fn try_pp(&mut self) -> Option<Self>
    {
        
        checked_pp_mut!(self)

    }

    fn mm(&mut self) -> Self
    {

        mm_mut!(self) //;

        //*self

    }

    fn try_mm(&mut self) -> Option<Self>
    {
        
        checked_mm_mut!(self)

    }

}

//u128

impl IncDecSelf for u128
{

    fn pp(&mut self) -> Self
    {

        pp_mut!(self) //;

        //*self

    }

    fn try_pp(&mut self) -> Option<Self>
    {
        
        checked_pp_mut!(self)

    }

    fn mm(&mut self) -> Self
    {

        mm_mut!(self) //;

        //*self

    }

    fn try_mm(&mut self) -> Option<Self>
    {
        
        checked_mm_mut!(self)

    }

}

//usize

impl IncDecSelf for usize
{

    fn pp(&mut self) -> Self
    {

        pp_mut!(self) //;

        //*self

    }

    fn try_pp(&mut self) -> Option<Self>
    {
        
        checked_pp_mut!(self)

    }

    fn mm(&mut self) -> Self
    {

        mm_mut!(self) //;

        //*self

    }

    fn try_mm(&mut self) -> Option<Self>
    {
        
        checked_mm_mut!(self)

    }

}

//Integers Only

pub trait IntIncDecSelf
    where Self: Sized + Copy
{

    fn overflowing_pp(&mut self) -> (Self, bool);

    fn overflowing_mm(&mut self) -> (Self, bool);

}

#[macro_export]
macro_rules! overflowing_pp_mut
{

    ($integer:ident) =>
    {

        {

            let res = $integer.overflowing_add(1);

            *$integer = res.0;

            res

        }

    }

}

#[macro_export]
macro_rules! overflowing_mm_mut
{

    ($integer:ident) =>
    {

        {

            let res = $integer.overflowing_sub(1);

            *$integer = res.0;

            res

        }

    }

}


//i8

impl IntIncDecSelf for i8
{

    fn overflowing_pp(&mut self) -> (Self, bool)
    {

        overflowing_pp_mut!(self)

        /*
        let res = self.overflowing_add(1);

        *self = res.0;

        res
        */
        
    }

    fn overflowing_mm(&mut self) -> (Self, bool)
    {

        overflowing_mm_mut!(self)

        /*
        let res =self.overflowing_sub(1);

        *self = res.0;

        res
        */

    }

}

//i16

impl IntIncDecSelf for i16
{

    fn overflowing_pp(&mut self) -> (Self, bool)
    {

        overflowing_pp_mut!(self)
        
    }

    fn overflowing_mm(&mut self) -> (Self, bool)
    {

        overflowing_mm_mut!(self)

    }

}

//i32

impl IntIncDecSelf for i32
{

    fn overflowing_pp(&mut self) -> (Self, bool)
    {

        overflowing_pp_mut!(self)
        
    }

    fn overflowing_mm(&mut self) -> (Self, bool)
    {

        overflowing_mm_mut!(self)

    }

}

//i64

impl IntIncDecSelf for i64
{

    fn overflowing_pp(&mut self) -> (Self, bool)
    {

        overflowing_pp_mut!(self)
        
    }

    fn overflowing_mm(&mut self) -> (Self, bool)
    {

        overflowing_mm_mut!(self)

    }

}

//i128

impl IntIncDecSelf for i128
{

    fn overflowing_pp(&mut self) -> (Self, bool)
    {

        overflowing_pp_mut!(self)
        
    }

    fn overflowing_mm(&mut self) -> (Self, bool)
    {

        overflowing_mm_mut!(self)

    }

}

//isize

impl IntIncDecSelf for isize
{

    fn overflowing_pp(&mut self) -> (Self, bool)
    {

        overflowing_pp_mut!(self)
        
    }

    fn overflowing_mm(&mut self) -> (Self, bool)
    {

        overflowing_mm_mut!(self)

    }

}

//u8

impl IntIncDecSelf for u8
{

    fn overflowing_pp(&mut self) -> (Self, bool)
    {

        overflowing_pp_mut!(self)
        
    }

    fn overflowing_mm(&mut self) -> (Self, bool)
    {

        overflowing_mm_mut!(self)

    }

}

//u16

impl IntIncDecSelf for u16
{

    fn overflowing_pp(&mut self) -> (Self, bool)
    {

        overflowing_pp_mut!(self)
        
    }

    fn overflowing_mm(&mut self) -> (Self, bool)
    {

        overflowing_mm_mut!(self)

    }

}

//u32

impl IntIncDecSelf for u32
{

    fn overflowing_pp(&mut self) -> (Self, bool)
    {

        overflowing_pp_mut!(self)
        
    }

    fn overflowing_mm(&mut self) -> (Self, bool)
    {

        overflowing_mm_mut!(self)

    }

}

//u64

impl IntIncDecSelf for u64
{

    fn overflowing_pp(&mut self) -> (Self, bool)
    {

        overflowing_pp_mut!(self)
        
    }

    fn overflowing_mm(&mut self) -> (Self, bool)
    {

        overflowing_mm_mut!(self)

    }

}

//u128

impl IntIncDecSelf for u128
{

    fn overflowing_pp(&mut self) -> (Self, bool)
    {

        overflowing_pp_mut!(self)
        
    }

    fn overflowing_mm(&mut self) -> (Self, bool)
    {

        overflowing_mm_mut!(self)

    }

}

//usize

impl IntIncDecSelf for usize
{

    fn overflowing_pp(&mut self) -> (Self, bool)
    {

        overflowing_pp_mut!(self)
        
    }

    fn overflowing_mm(&mut self) -> (Self, bool)
    {

        overflowing_mm_mut!(self)

    }

}

#[cfg(test)]
mod tests
{

    //use super::*;

    #[test]
    fn try_pp()
    {

        let mut int_val = 1;

        pp!(int_val);

        assert_eq!(2, int_val);

    }

    #[test]
    fn try_ppf()
    {

        let mut f32_val: f32 = 1.0;

        ppf!(f32_val);

        assert_eq!(2.0, f32_val);

        let mut f64_val = 1.0;

        ppf!(f64_val);

        assert_eq!(2.0, f64_val);

    }

    #[test]
    fn try_mm()
    {

        let mut int_val = 2;

        mm!(int_val);

        assert_eq!(1, int_val);

    }

    #[test]
    fn try_mmf()
    {

        let mut f32_val: f32 = 2.0;

        mmf!(f32_val);

        assert_eq!(1.0, f32_val);

        let mut f64_val = 2.0;

        mmf!(f64_val);

        assert_eq!(1.0, f64_val);

    }

}
