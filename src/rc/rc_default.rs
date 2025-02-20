//!
//! Traits for getting reference counted default objects.
//! 

use std::rc::Rc;

use std::sync::Arc;

///
/// A trait for giving a default value in a reference counted object.
/// 
pub trait RcDefault
{

    fn rc_default() -> Rc<Self>;
    
}

///
/// A trait for giving a default value in an atomically reference counted object.
/// 
pub trait ArcDefault
{

    fn arc_default() -> Arc<Self>;
    
}
