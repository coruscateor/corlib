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

        *$integer += 1;

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

        *$float += 1.0;

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

        *$integer -= 1;

        //*$integer

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

        $float -= 1.0;

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

pub trait IncrementsSelf
    where Self: Sized + Copy
{

    fn pp(&mut self) -> Self;

    fn try_pp(&mut self) -> Option<Self>;

}

pub trait DecrementsSelf
    where Self: Sized + Copy
{

    fn mm(&mut self) -> Self;

    fn try_mm(&mut self) -> Option<Self>;

}

//Incrementation

//f32

impl IncrementsSelf for f32
{

    fn pp(&mut self) -> Self
    {

        ppf_mut!(self);

        *self

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

            ppf_mut!(self);

            Some(*self)

        }
        else
        {

            None
            
        }


    }

}

//f64

impl IncrementsSelf for f64
{

    fn pp(&mut self) -> Self
    {

        ppf_mut!(self);

        *self

    }

    fn try_pp(&mut self) -> Option<Self>
    {
        
        if *self < (f64::MAX - 0.9)
        {

            ppf_mut!(self);

            Some(*self)

        }
        else
        {

            None
            
        }

    }

}

//i8

impl IncrementsSelf for i8
{

    fn pp(&mut self) -> Self
    {

        pp_mut!(self);

        *self

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

}

//i16

impl IncrementsSelf for i16
{

    fn pp(&mut self) -> Self
    {

        pp_mut!(self);

        *self

    }

    fn try_pp(&mut self) -> Option<Self>
    {
        
        checked_pp_mut!(self)

    }

}

//i32

impl IncrementsSelf for i32
{

    fn pp(&mut self) -> Self
    {

        pp_mut!(self);

        *self

    }

    fn try_pp(&mut self) -> Option<Self>
    {
        
        checked_pp_mut!(self)

    }

}

//i64

impl IncrementsSelf for i64
{

    fn pp(&mut self) -> Self
    {

        pp_mut!(self);

        *self

    }

    fn try_pp(&mut self) -> Option<Self>
    {
        
        checked_pp_mut!(self)

    }

}

//i128

impl IncrementsSelf for i128
{

    fn pp(&mut self) -> Self
    {

        pp_mut!(self);

        *self

    }

    fn try_pp(&mut self) -> Option<Self>
    {
        
        checked_pp_mut!(self)

    }

}

//isize

impl IncrementsSelf for isize
{

    fn pp(&mut self) -> Self
    {

        pp_mut!(self);

        *self

    }

    fn try_pp(&mut self) -> Option<Self>
    {
        
        checked_pp_mut!(self)

    }

}

//u8

impl IncrementsSelf for u8
{

    fn pp(&mut self) -> Self
    {

        pp_mut!(self);

        *self

    }

    fn try_pp(&mut self) -> Option<Self>
    {
        
        checked_pp_mut!(self)

    }

}

//u16

impl IncrementsSelf for u16
{

    fn pp(&mut self) -> Self
    {

        pp_mut!(self);

        *self

    }

    fn try_pp(&mut self) -> Option<Self>
    {
        
        checked_pp_mut!(self)

    }

}

//u32

impl IncrementsSelf for u32
{

    fn pp(&mut self) -> Self
    {

        pp_mut!(self);

        *self

    }

    fn try_pp(&mut self) -> Option<Self>
    {
        
        checked_pp_mut!(self)

    }

}

//u64

impl IncrementsSelf for u64
{

    fn pp(&mut self) -> Self
    {

        pp_mut!(self);

        *self

    }

    fn try_pp(&mut self) -> Option<Self>
    {
        
        checked_pp_mut!(self)

    }

}

//u128

impl IncrementsSelf for u128
{

    fn pp(&mut self) -> Self
    {

        pp_mut!(self);

        *self

    }

    fn try_pp(&mut self) -> Option<Self>
    {
        
        checked_pp_mut!(self)

    }

}

//usize

impl IncrementsSelf for usize
{

    fn pp(&mut self) -> Self
    {

        pp_mut!(self);

        *self

    }

    fn try_pp(&mut self) -> Option<Self>
    {
        
        checked_pp_mut!(self)

    }

}

//Decrementation

//f32

impl DecrementsSelf for f32
{

    fn mm(&mut self) -> Self
    {

        ppf_mut!(self);

        *self

    }

    fn try_mm(&mut self) -> Option<Self>
    {
        
        if *self > (f32::MIN + 0.9)
        {

            ppf_mut!(self);

            Some(*self)

        }
        else
        {

            None
            
        }

    }

}

//f64

impl DecrementsSelf for f64
{

    fn mm(&mut self) -> Self
    {

        ppf_mut!(self);

        *self

    }

    fn try_mm(&mut self) -> Option<Self>
    {
        
        if *self > (f64::MIN + 0.9)
        {

            ppf_mut!(self);

            Some(*self)

        }
        else
        {

            None
            
        }
        
    }

}

//i8

impl DecrementsSelf for i8
{

    fn mm(&mut self) -> Self
    {

        mm_mut!(self);

        *self

    }

    fn try_mm(&mut self) -> Option<Self>
    {
        
        checked_mm_mut!(self)

    }

}

//i16

impl DecrementsSelf for i16
{

    fn mm(&mut self) -> Self
    {

        mm_mut!(self);

        *self

    }

    fn try_mm(&mut self) -> Option<Self>
    {
        
        checked_mm_mut!(self)

    }

}

//i32

impl DecrementsSelf for i32
{

    fn mm(&mut self) -> Self
    {

        mm_mut!(self);

        *self

    }

    fn try_mm(&mut self) -> Option<Self>
    {
        
        checked_mm_mut!(self)

    }

}

//i64

impl DecrementsSelf for i64
{

    fn mm(&mut self) -> Self
    {

        mm_mut!(self);

        *self

    }

    fn try_mm(&mut self) -> Option<Self>
    {
        
        checked_mm_mut!(self)

    }

}

//i128

impl DecrementsSelf for i128
{

    fn mm(&mut self) -> Self
    {

        mm_mut!(self);

        *self

    }

    fn try_mm(&mut self) -> Option<Self>
    {
        
        checked_mm_mut!(self)

    }

}

//isize

impl DecrementsSelf for isize
{

    fn mm(&mut self) -> Self
    {

        mm_mut!(self);

        *self

    }

    fn try_mm(&mut self) -> Option<Self>
    {
        
        checked_mm_mut!(self)

    }

}

//u8

impl DecrementsSelf for u8
{

    fn mm(&mut self) -> Self
    {

        mm_mut!(self);

        *self

    }

    fn try_mm(&mut self) -> Option<Self>
    {
        
        checked_mm_mut!(self)

    }

}

//u16

impl DecrementsSelf for u16
{

    fn mm(&mut self) -> Self
    {

        mm_mut!(self);

        *self

    }

    fn try_mm(&mut self) -> Option<Self>
    {
        
        checked_mm_mut!(self)

    }

}

//u32

impl DecrementsSelf for u32
{

    fn mm(&mut self) -> Self
    {

        mm_mut!(self);

        *self

    }

    fn try_mm(&mut self) -> Option<Self>
    {
        
        checked_mm_mut!(self)

    }

}

//u64

impl DecrementsSelf for u64
{

    fn mm(&mut self) -> Self
    {

        mm_mut!(self);

        *self

    }

    fn try_mm(&mut self) -> Option<Self>
    {
        
        checked_mm_mut!(self)

    }

}

//u128

impl DecrementsSelf for u128
{

    fn mm(&mut self) -> Self
    {

        mm_mut!(self);

        *self

    }

    fn try_mm(&mut self) -> Option<Self>
    {
        
        checked_mm_mut!(self)

    }

}

//usize

impl DecrementsSelf for usize
{

    fn mm(&mut self) -> Self
    {

        mm_mut!(self);

        *self

    }

    fn try_mm(&mut self) -> Option<Self>
    {
        
        checked_mm_mut!(self)

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
