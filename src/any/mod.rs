//!
//! Cast stuff into &dyn Any and &mut dyn Anys.
//! 

mod any_traits;

pub use any_traits::*;

mod any_functions;

pub use any_functions::*;

cfg_if::cfg_if!
{

    if #[cfg(feature = "macros")]
    {

        pub use corlib_macros::AsAnyRef;

        pub use corlib_macros::AsAnyMut;

    }

}

