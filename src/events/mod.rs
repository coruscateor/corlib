//!
//! Event structs inspired by .NET events.
//! 
//! You basically instantiate a non-base event struct, expose the subscribe and unsubscribe methods and call the raise method when it's time to raise an event.
//!  

//mod event_dictionary;

//pub use event_dictionary::*; //crate::events:: event_dictionary::*;

mod dictionary_weak_rc_event;

pub use dictionary_weak_rc_event::*;

mod list_event;

pub use list_event::*;

mod base_list_event;

mod list_args_event;

mod base_dictionary_weak_rc_event;

mod dictionary_weak_rc_args_event;

mod skip_once;

mod single_sub_event;

pub use single_sub_event::*;

mod single_sub_args_event;

pub use single_sub_args_event::*;


