#![doc = include_str!("../README.md")] 

use cfg_if::cfg_if;

mod non_option;

pub use non_option::*;

pub mod collections;

pub mod events;

mod weak_by_ptr;

pub use weak_by_ptr::*;

mod gap_filling_counter;

pub use gap_filling_counter::*;

pub mod has_one;

mod dropper;

pub use dropper::*;

mod getters_setters_callers;

//pub use getters_setters_callers::*;

mod rc_by_ptr;

pub use rc_by_ptr::*;

pub mod rc_default;

pub mod text;

//pub use text::*;

pub mod get_some;

pub mod inc_dec;

pub mod as_any;

pub use as_any::*;

pub mod rfc;

pub mod upgrading;

mod arc_by_ptr;

pub use arc_by_ptr::*;

mod sync_weak_by_ptr;

pub use sync_weak_by_ptr::*;

mod immut;

pub use immut::*;

mod work_in_progress_result;

pub use work_in_progress_result::*;

cfg_if!
{

    if #[cfg(feature = "drop_panic")]
    {
        
        mod drop_panic;

        pub use drop_panic::*;

    }

}


