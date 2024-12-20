//!
//! A fun way to crash your programme.
//! 

///
/// Use this to simulate an unexpected programme termination.
/// 
pub struct DropPanic();

impl Drop for DropPanic
{

    fn drop(&mut self)
    {

        panic!("OH SHI...");

    }

}
