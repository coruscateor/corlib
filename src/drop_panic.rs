//!
//! Panicking made convenient.
//! 

///
/// Use this when you want the programme to panic at the end of a scope.
/// 
pub struct DropPanic();

impl Drop for DropPanic
{

    fn drop(&mut self)
    {

        panic!("OH SHI...");

    }

}
