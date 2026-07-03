#![doc = include_str!("../README.md")] 

#![cfg_attr(docsrs, feature(doc_cfg))]

//Disabled - To be removed:

//use cfg_if::cfg_if;

mod non_option;

pub use non_option::*;

pub mod collections;

pub mod events;

//mod weak_by_ptr;

//pub use weak_by_ptr::*;

mod gap_filling_counter;

pub use gap_filling_counter::*;

pub mod has_one;

//getters_setters_callers - use accessorise

//mod rc_by_ptr;

//pub use rc_by_ptr::*;

//pub mod rc_default;

pub mod text;

//pub use text::*;

mod get_some;

//pub use get_some::*;

//inc_dec - moved to inc_dec

//mod arc_by_ptr;

//pub use arc_by_ptr::*;

//mod sync_weak_by_ptr;

//pub use sync_weak_by_ptr::*;

mod immut;

pub use immut::*;

#[cfg(all(test, feature = "serde"))]
mod immut_tests;

//work_in_progress_result - moved to highly_sendable:

pub mod any;

pub mod cell;

pub mod value;

//cargo_env - moved to env_var_helpers

cfg_if::cfg_if!
{

    if #[cfg(feature = "drop_panic")]
    {
        
        pub mod drop_panic;

    }

}

mod weak_self;

pub use weak_self::*;

pub mod rc;

//enums.rs contents: ThisOrThat, ThisThatOther and DefaultOrValue - see Some More Options

mod may_retry;

#[cfg(test)]
mod may_retry_tests;

mod control_flow;

pub use control_flow::*;

#[cfg(feature = "serde")]
pub mod serde;

#[cfg(feature = "macros")]
pub use corlib_macros::WeakSelf;
