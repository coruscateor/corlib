use std::cell::{Ref, RefCell, RefMut};

/// 
/// MutState is intended to help you automatically manage RefCell reference lifetimes with closures.
/// 
pub struct MutState<T>
{

    mut_state: RefCell<T>

}

impl<T> MutState<T>
{

    pub fn new(mut_state: T) -> Self
    {

        Self
        {

            mut_state: RefCell::new(mut_state)

        }

    }

    pub fn mut_state_ref(&self) -> &RefCell<T>
    {

        &self.mut_state

    }

    //borrow

    pub fn borrow<F, R>(&self, mut func: F) -> R
        where F: FnMut(Ref<T>) -> R
    {

        let rfc_ref = self.mut_state.borrow();

        func(rfc_ref)

    }

    pub fn borrow_with<P, F, R>(rfc: &RefCell<T>, param: P, mut func: F) -> R
        where F: FnMut(Ref<T>, P) -> R
    {

        let rfc_ref = rfc.borrow();

        func(rfc_ref, param)

    }

    pub fn borrow_with_ref<P, F, R>(rfc: &RefCell<T>, param: &P, mut func: F) -> R
        where F: FnMut(Ref<T>, &P) -> R
    {

        let rfc_ref = rfc.borrow();

        func(rfc_ref, param)

    }

    pub fn borrow_with_mut<P, F, R>(rfc: &RefCell<T>, param: &mut P, mut func: F) -> R
        where F: FnMut(Ref<T>, &mut P) -> R
    {

        let rfc_ref = rfc.borrow();

        func(rfc_ref, param)

    }

    //borrow_mut

    pub fn borrow_mut<F, R>(&self, mut func: F) -> R
        where F: FnMut(RefMut<T>) -> R
    {

        let rfc_mut = self.mut_state.borrow_mut();

        func(rfc_mut)

    }

    pub fn borrow_mut_with<P, F, R>(rfc: &RefCell<T>, param: P, mut func: F) -> R
        where F: FnMut(RefMut<T>, P) -> R
    {

        let rfc_mut = rfc.borrow_mut();

        func(rfc_mut, param)

    }

    pub fn borrow_mut_with_ref<P, F, R>(rfc: &RefCell<T>, param: &P, mut func: F) -> R
        where F: FnMut(RefMut<T>, &P) -> R
    {

        let rfc_mut = rfc.borrow_mut();

        func(rfc_mut, param)

    }

    pub fn borrow_mut_with_mut<P, F, R>(rfc: &RefCell<T>, param: &mut P, mut func: F) -> R
        where F: FnMut(RefMut<T>, &mut P) -> R
    {

        let rfc_mut = rfc.borrow_mut();

        func(rfc_mut, param)

    }

}
