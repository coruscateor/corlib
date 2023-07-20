use std::rc::Rc;

use std::sync::Arc;

///Intended to be used to return a default reference counted object containing Self.
pub trait RcDefault
{

    fn rc_default() -> Rc<Self>;
    
}

///Intended to be used to return a default atomically reference counted object containing Self.
pub trait ArcDefault
{

    fn arc_default() -> Arc<Self>;
    
}
