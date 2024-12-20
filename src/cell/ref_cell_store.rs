use std::cell::{BorrowError, BorrowMutError, Ref, RefCell, RefMut};

/// 
/// RefCellStore is intended to help you automatically manage RefCell references with closures.
/// 
pub struct RefCellStore<T>
{

    ref_cell: RefCell<T>

}

impl<T> RefCellStore<T>
{

    pub fn new(state: T) -> Self
    {

        Self
        {

            ref_cell: RefCell::new(state)

        }

    }

    pub fn ref_cell_ref(&self) -> &RefCell<T>
    {

        &self.ref_cell

    }

    //borrow

    pub fn borrow<F, R>(&self, mut func: F) -> R
        where F: FnMut(Ref<T>) -> R
    {

        let rfc_ref = self.ref_cell.borrow();

        func(rfc_ref)

    }

    pub fn borrow_with_param<P, F, R>(&self, param: P, mut func: F) -> R
        where F: FnMut(Ref<T>, P) -> R
    {

        let rfc_ref = self.ref_cell.borrow();

        func(rfc_ref, param)

    }

    pub fn borrow_with_ref<P, F, R>(&self, param: &P, mut func: F) -> R
        where F: FnMut(Ref<T>, &P) -> R
    {

        let rfc_ref = self.ref_cell.borrow();

        func(rfc_ref, param)

    }

    pub fn borrow_with_mut<P, F, R>(&self, param: &mut P, mut func: F) -> R
        where F: FnMut(Ref<T>, &mut P) -> R
    {

        let rfc_ref = self.ref_cell.borrow();

        func(rfc_ref, param)

    }

    //borrow_mut

    pub fn borrow_mut<F, R>(&self, mut func: F) -> R
        where F: FnMut(RefMut<T>) -> R
    {

        let rfc_mut = self.ref_cell.borrow_mut();

        func(rfc_mut)

    }

    pub fn borrow_mut_with_param<P, F, R>(&self, param: P, mut func: F) -> R
        where F: FnMut(RefMut<T>, P) -> R
    {

        let rfc_mut = self.ref_cell.borrow_mut();

        func(rfc_mut, param)

    }

    pub fn borrow_mut_with_ref<P, F, R>(&self, param: &P, mut func: F) -> R
        where F: FnMut(RefMut<T>, &P) -> R
    {

        let rfc_mut = self.ref_cell.borrow_mut();

        func(rfc_mut, param)

    }

    pub fn borrow_mut_with_mut<P, F, R>(&self, param: &mut P, mut func: F) -> R
        where F: FnMut(RefMut<T>, &mut P) -> R
    {

        let rfc_mut = self.ref_cell.borrow_mut();

        func(rfc_mut, param)

    }

    //try_borrow

    pub fn try_borrow<F, R>(&self, mut func: F) -> Result<R, BorrowError>
        where F: FnMut(Ref<T>) -> R
    {

        let ref_res = self.ref_cell.try_borrow();

        match ref_res
        {

            Ok(res) =>
            {

                Ok(func(res))

            }
            Err(err) =>
            {

                Err(err)

            }

        }

    }

    pub fn try_borrow_with_param<P, F, R>(&self, param: P, mut func: F) -> Result<R, BorrowError>
        where F: FnMut(Ref<T>, P) -> R
    {

        let ref_res = self.ref_cell.try_borrow();

        match ref_res
        {

            Ok(res) =>
            {

                Ok(func(res, param))

            }
            Err(err) =>
            {

                Err(err)

            }

        }
    }

    pub fn try_borrow_with_ref<P, F, R>(&self, param: &P, mut func: F) -> Result<R, BorrowError>
        where F: FnMut(Ref<T>, &P) -> R
    {

        let ref_res = self.ref_cell.try_borrow();

        match ref_res
        {

            Ok(res) =>
            {

                Ok(func(res, param))

            }
            Err(err) =>
            {

                Err(err)

            }

        }

    }

    pub fn try_borrow_with_mut<P, F, R>(&self, param: &mut P, mut func: F) -> Result<R, BorrowError>
        where F: FnMut(Ref<T>, &mut P) -> R
    {

        let ref_res = self.ref_cell.try_borrow();

        match ref_res
        {

            Ok(res) =>
            {

                Ok(func(res, param))

            }
            Err(err) =>
            {

                Err(err)

            }

        }

    }

    //try_borrow_mut

    pub fn try_borrow_mut<F, R>(&self, mut func: F) -> Result<R, BorrowMutError>
        where F: FnMut(RefMut<T>) -> R
    {

        let ref_res = self.ref_cell.try_borrow_mut();

        match ref_res
        {

            Ok(res) =>
            {

                Ok(func(res))

            }
            Err(err) =>
            {

                Err(err)

            }

        }

    }

    pub fn try_borrow_mut_with_param<P, F, R>(&self, param: P, mut func: F) -> Result<R, BorrowMutError>
        where F: FnMut(RefMut<T>, P) -> R
    {

        let ref_res = self.ref_cell.try_borrow_mut();

        match ref_res
        {

            Ok(res) =>
            {

                Ok(func(res, param))

            }
            Err(err) =>
            {

                Err(err)

            }

        }
    }

    pub fn try_borrow_mut_with_ref<P, F, R>(&self, param: &P, mut func: F) -> Result<R, BorrowMutError>
        where F: FnMut(RefMut<T>, &P) -> R
    {

        let ref_res = self.ref_cell.try_borrow_mut();

        match ref_res
        {

            Ok(res) =>
            {

                Ok(func(res, param))

            }
            Err(err) =>
            {

                Err(err)

            }

        }

    }

    pub fn try_borrow_mut_with_mut<P, F, R>(&self, param: &mut P, mut func: F) -> Result<R, BorrowMutError>
        where F: FnMut(RefMut<T>, &mut P) -> R
    {

        let ref_res = self.ref_cell.try_borrow_mut();

        match ref_res
        {

            Ok(res) =>
            {

                Ok(func(res, param))

            }
            Err(err) =>
            {

                Err(err)

            }

        }

    }

}
