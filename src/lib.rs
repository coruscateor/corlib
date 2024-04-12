#![doc = include_str!("../README.md")] 

mod non_option;

pub use non_option::*; //crate::

//Directories

pub mod collections;

//use crate::collections::*;

pub mod events;

//pub use crate::events::*;

//

//pub use crate::events::event_dictionary::*; 

mod weak_by_ptr;

pub use weak_by_ptr::*; //crate::

//mod mut_borrowing;

//pub use mut_borrowing::*; //crate::

//mod var_non_option;

//pub use var_non_option::*; //crate::

mod gap_filling_counter;

pub use gap_filling_counter::*; //crate::

mod get_rc_or_weak_self;

pub use get_rc_or_weak_self::*; //crate::

//Directories

//pub use crate::events::event_dictionary::*;

//pub mod events;

//mod self_has_parent;

//pub use self_has_parent::*;

pub mod has_one;

mod dropper;

pub use dropper::*;

//pub mod macros;

//pub use has_one::*;

mod getters_setters_callers;

pub use getters_setters_callers::*;

mod rc_by_ptr;

pub use rc_by_ptr::*;

pub mod rc_default;

mod invalid;

pub use invalid::*;

mod lazy;

pub use lazy::*;

mod get_ref_mut;

pub use get_ref_mut::*;

//mod default_abuser;

//pub use default_abuser::*;

mod text_enums;

pub use text_enums::*;

mod sync;

pub use sync::*;

mod refcell_borrowing;

pub use refcell_borrowing::*;

pub mod get_some;

pub mod inc_dec;

pub mod as_any;

pub use as_any::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
