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

mod getters_setters_callers;

pub use getters_setters_callers::*;

mod rc_by_ptr;

pub use rc_by_ptr::*;

pub mod rc_default;

mod lazy;

pub use lazy::*;

mod text_enums;

pub use text_enums::*;

pub mod get_some;

pub mod inc_dec;

pub mod as_any;

pub use as_any::*;

pub mod rfc;

pub mod upgrading;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
