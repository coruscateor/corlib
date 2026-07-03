//!
//! Cast stuff into &dyn Any and &mut dyn Anys.
//! 

mod any;

pub use any::*;

cfg_if::cfg_if!
{

    if #[cfg(feature = "macros")]
    {

        pub use corlib_macros::AsAnyRef;

        pub use corlib_macros::AsAnyMut;

    }

}

