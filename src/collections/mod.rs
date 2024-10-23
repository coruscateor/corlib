//! 
//! Collection related stucts and traits, some of which are inspired by .NET.
//! 

mod list;

pub use list::*; //crate::collections::

mod hashset_item;

pub use hashset_item::*; //crate::collections::

mod key_value_pair;

pub use key_value_pair::*; //crate::collections::

mod dictionary;

pub use dictionary::*; //crate::collections::

mod unique_item_list;

pub use unique_item_list::*;

mod queue;

pub use queue::*;

pub mod dyn_hashing_and_cmp;

//pub use dyn_hashing_and_cmp::*;

mod stacked_vec;

pub use stacked_vec::*;

mod non_default_stacked_vec;

pub use non_default_stacked_vec::*;


