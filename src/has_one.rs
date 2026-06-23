//!
//! Trait implenenations for getting the value of one for each numeric type.
//! 

///
/// For helping you get the number 1 or 1.0 values of numeric types in situations that require it.
/// 
pub trait HasOne<T>
{

    fn one() -> T;

}

//f32

///
/// A HasOne\<T\> implementation for the f32 type.
/// 
#[derive(Default, Debug)]
pub struct F32HasOne();

impl HasOne<f32> for F32HasOne {
    
    fn one() -> f32
    {

        1.0

    }

}

//f64

///
/// A HasOne\<T\> implementation for the f64 type.
/// 
#[derive(Default, Debug)]
pub struct F64HasOne();

impl HasOne<f64> for F64HasOne {
    
    fn one() -> f64
    {

        1.0

    }

}

//i8

///
/// A HasOne\<T\> implementation for the i8 type.
/// 
#[derive(Default, Debug)]
pub struct I8HasOne();

impl HasOne<i8> for I8HasOne {
    
    fn one() -> i8
    {

        1

    }

}

//i16

///
/// A HasOne\<T\> implementation for the i16 type.
/// 
#[derive(Default, Debug)]
pub struct I16HasOne();

impl HasOne<i16> for I16HasOne {
    
    fn one() -> i16
    {

        1

    }

}

//i32

///
/// A HasOne\<T\> implementation for the i32 type.
/// 
#[derive(Default, Debug)]
pub struct I32HasOne();

impl HasOne<i32> for I32HasOne {
    
    fn one() -> i32
    {

        1

    }

}


//i64

///
/// A HasOne\<T\> implementation for the i64 type.
/// 
#[derive(Default, Debug)]
pub struct I64HasOne();

impl HasOne<i64> for I64HasOne {
    
    fn one() -> i64
    {

        1

    }

}

//i128

///
/// A HasOne\<T\> implementation for the i128 type.
/// 
#[derive(Default, Debug)]
pub struct I128HasOne();

impl HasOne<i128> for I128HasOne {
    
    fn one() -> i128
    {

        1

    }

}

//isize

///
/// A HasOne\<T\> implementation for the isize type.
/// 
#[derive(Default, Debug)]
pub struct ISizeHasOne();

impl HasOne<isize> for ISizeHasOne {
    
    fn one() -> isize
    {

        1

    }

}

///
/// A HasOne\<T\> implementation for the u8 type.
/// 
#[derive(Default, Debug)]
pub struct U8HasOne();

impl HasOne<u8> for U8HasOne {
    
    fn one() -> u8
    {

        1

    }

}

///
/// A HasOne\<T\> implementation for the u16 type.
/// 
#[derive(Default, Debug)]
pub struct U16HasOne();

impl HasOne<u16> for U16HasOne {
    
    fn one() -> u16
    {

        1

    }

}

///
/// A HasOne\<T\> implementation for the u32 type.
/// 
#[derive(Default, Debug)]
pub struct U32HasOne();

impl HasOne<u32> for U32HasOne {
    
    fn one() -> u32
    {

        1

    }

}

///
/// A HasOne\<T\> implementation for the u64 type.
/// 
#[derive(Default, Debug)]
pub struct U64HasOne();

impl HasOne<u64> for U64HasOne {
    
    fn one() -> u64
    {

        1

    }

}

///
/// A HasOne\<T\> implementation for the u128 type.
/// 
#[derive(Default, Debug)]
pub struct U128HasOne();

impl HasOne<u128> for U128HasOne {
    
    fn one() -> u128
    {

        1

    }

}

///
/// A HasOne\<T\> implementation for the usize type.
/// 
#[derive(Default, Debug)]
pub struct USizeHasOne();

impl HasOne<usize> for USizeHasOne {
    
    fn one() -> usize
    {

        1

    }

}

