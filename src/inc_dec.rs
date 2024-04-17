//!
//! This module has ++ and -- like operations in the form of macros.
//! 
//! If you miss these convenient unary operators here is the place to look.
//! 

/// Increments an integer by one.
#[macro_export]
macro_rules! pp
{
    ($integer:ident) =>
    {

        $integer += 1;

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

/// Decrements an integer by one.
#[macro_export]
macro_rules! mm
{
    ($integer:ident) =>
    {

        $integer -= 1;

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
