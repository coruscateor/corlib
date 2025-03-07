use std::cell::{RefCell, Ref, RefMut};

///
/// Intended to be used with a std::cell:RefCell that is a field of $this.
///
/// Calls “borrow” on the RefCell field and passes the result and a reference to $this to the provided $func expression.
///
/// If no $rfc_field is provided (first case) then the name “mut_state” is used for the $rfc_field.
/// 
#[macro_export]
macro_rules! rfc_borrow
{

    ($this:ident, $func:expr) =>
    {

        rfc_borrow!($this, mut_state, $func)

    };
    ($this:ident, $rfc_field:ident, $func:expr) =>
    {

        {

            let mut_state_ref = $this.$rfc_field.borrow();

            $func(mut_state_ref, &$this)

        }

    }

}

///
/// Intended to be used with a std::cell:RefCell that is a field of $this.
///
/// Calls “borrow_mut” on the RefCell field and passes the result and a reference to $this to the provided $func expression.
///
/// If no $rfc_field is provided (first case) then the name “mut_state” is used for the $rfc_field.
/// 
#[macro_export]
macro_rules! rfc_borrow_mut
{

    ($this:ident, $func:expr) =>
    {

        rfc_borrow_mut!($this, mut_state, $func)

    };
    ($this:ident, $rfc_field:ident, $func:expr) =>
    {

        {

            let mut_state_mut = $this.$rfc_field.borrow_mut();

            $func(mut_state_mut, &$this)

        }

    }

}

//Pass this not by ref

///
/// Like rfc_borrow but for situations where it is intended that $this be copied i.e. where $this is already a reference.
/// 
#[macro_export]
macro_rules! rfc_borrow_2
{

    ($this:ident, $func:expr) =>
    {

        rfc_borrow_2!($this, mut_state, $func)

    };
    ($this:ident, $rfc_field:ident, $func:expr) =>
    {

        {

            let mut_state_ref = $this.$rfc_field.borrow();

            $func(mut_state_ref, $this)

        }

    }

}

///
/// Like rfc_borrow_mut but for situations where it is intended that $this be copied i.e. where $this is already a reference.
/// 
#[macro_export]
macro_rules! rfc_borrow_mut_2
{

    ($this:ident, $func:expr) =>
    {

        rfc_borrow_mut_2!($this, mut_state, $func)

    };
    ($this:ident, $rfc_field:ident, $func:expr) =>
    {

        {

            let mut_state_mut = $this.$rfc_field.borrow_mut();

            $func(mut_state_mut, $this)

        }

    }

}

//Param by ref

///
/// Calls “borrow” on the provided RefCell, passing the resultant Ref object to the provided func object, returning the result.
/// 
pub fn borrow<T, F, R>(rfc: &RefCell<T>, mut func: F) -> R
    where F: FnMut(Ref<T>) -> R
{

   let rfc_ref = rfc.borrow();

   func(rfc_ref)

}

///
/// Calls “borrow” on the provided RefCell, passing the resultant Ref object and the provided param reference to the provided func object, returning the result.
/// 
pub fn borrow_param<T, P, F, R>(rfc: &RefCell<T>, param: &P, mut func: F) -> R
    where F: FnMut(Ref<T>, &P) -> R
{

   let rfc_ref = rfc.borrow();

   func(rfc_ref, param)

}

///
/// Calls “borrow_mut” on the provided RefCell, passing the resultant RefMut object to the provided func object, returning the result.
/// 
pub fn borrow_mut<T, F, R>(rfc: &RefCell<T>, mut func: F) -> R
    where F: FnMut(RefMut<T>) -> R
{

   let rfc_mut = rfc.borrow_mut();

   func(rfc_mut)

}

///
/// Calls “borrow_mut” on the provided RefCell, passing the resultant RefMut object and the provided param reference to the provided func object, returning the result.
/// 
pub fn borrow_mut_param<T, P, F, R>(rfc: &RefCell<T>, param: &P, mut func: F) -> R
    where F: FnMut(RefMut<T>, &P) -> R
{

   let rfc_mut = rfc.borrow_mut();

   func(rfc_mut, param)

}

//Param by mut ref

pub fn borrow_param_2<T, P, F, R>(rfc: &RefCell<T>, param: &mut P, mut func: F) -> R
    where F: FnMut(Ref<T>, &mut P) -> R
{

   let rfc_ref = rfc.borrow();

   func(rfc_ref, param)

}

pub fn borrow_mut_param_2<T, P, F, R>(rfc: &RefCell<T>, param: &mut P, mut func: F) -> R
    where F: FnMut(RefMut<T>, &mut P) -> R
{

   let rfc_mut = rfc.borrow_mut();

   func(rfc_mut, param)

}

//Clone param

pub fn borrow_param_3<T, P, F, R>(rfc: &RefCell<T>, param: &P, mut func: F) -> R
    where F: FnMut(Ref<T>, P) -> R,
          P: Clone
{

   let rfc_ref = rfc.borrow();

   func(rfc_ref, param.clone())

}

pub fn borrow_mut_param_3<T, P, F, R>(rfc: &RefCell<T>, param: &P, mut func: F) -> R
    where F: FnMut(RefMut<T>, P) -> R,
          P: Clone
{

   let rfc_mut = rfc.borrow_mut();

   func(rfc_mut, param.clone())

}

//Copy or move param

pub fn borrow_param_4<T, P, F, R>(rfc: &RefCell<T>, param: P, mut func: F) -> R
    where F: FnMut(Ref<T>, P) -> R
{

   let rfc_ref = rfc.borrow();

   func(rfc_ref, param)

}

pub fn borrow_mut_param_4<T, P, F, R>(rfc: &RefCell<T>, param: P, mut func: F) -> R
    where F: FnMut(RefMut<T>, P) -> R
{

   let rfc_mut = rfc.borrow_mut();

   func(rfc_mut, param)

}
