//!
//! The items in this module help you work with RefCells.
//!

use std::{cell::RefCell, rc::{Rc, Weak}};

mod rfc;

pub use rfc::*;

mod ref_cell_store;

pub use ref_cell_store::*;

///
/// The std::rc::Rc<std::cell::RefCell<T>> type
/// 
pub type RcRefCell<T> = Rc<RefCell<T>>;

///
/// The std::rc::Weak<std::cell::RefCell<T>> type
/// 
pub type WeakRefCell<T> = Weak<RefCell<T>>;
