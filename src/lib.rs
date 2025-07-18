#![doc = include_str!("../README.md")] 

#![cfg_attr(docsrs, feature(doc_auto_cfg))]

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

//To be removed - use accessorise:

//mod getters_setters_callers;

//pub use getters_setters_callers::*;

//mod rc_by_ptr;

//pub use rc_by_ptr::*;

//pub mod rc_default;

pub mod text;

//pub use text::*;

mod get_some;

//pub use get_some::*;

//To be removed - moved to inc_dec:

//pub mod inc_dec;

pub mod upgrading;

//mod arc_by_ptr;

//pub use arc_by_ptr::*;

//mod sync_weak_by_ptr;

//pub use sync_weak_by_ptr::*;

mod immut;

pub use immut::*;

//Disabled - to be removed - moved to highly_sendable:

//mod work_in_progress_result;

//pub use work_in_progress_result::*;

pub mod convert;

pub mod cell;

pub mod value;

//Disabled - to be removed - moved to env_var_helpers:

//pub mod cargo_env;

#[cfg(feature = "drop_panic")]
mod drop_panic;

#[cfg(feature = "drop_panic")]
pub use drop_panic::*;

//Disabled - to be removed:

/*
cfg_if!
{

    if #[cfg(feature = "drop_panic")]
    {
        
        pub mod drop_panic;

        //pub use drop_panic::*;

    }

}
*/

mod weak_self;

pub use weak_self::*;

pub mod rc;

mod enums;

pub use enums::*;

mod may_retry;

pub use may_retry::*;

#[cfg(test)]
mod may_retry_tests;

#[cfg(feature = "highly_sendable")]
pub extern crate highly_sendable;

#[cfg(feature = "env_var_helpers")]
pub extern crate env_var_helpers;

#[cfg(feature = "inc_dec")]
pub extern crate inc_dec;

#[cfg(feature = "capped_collections")]
pub extern crate capped_collections;

#[cfg(feature = "accessorise")]
pub extern crate accessorise;

