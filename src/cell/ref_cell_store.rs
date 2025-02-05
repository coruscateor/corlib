use std::{cell::{BorrowError, BorrowMutError, Ref, RefCell, RefMut}, fmt::Debug, rc::{Rc, Weak}};

pub type RcRefCellStore<T> = Rc<RefCellStore<T>>;

pub type WeakRefCellStore<T> = Weak<RefCellStore<T>>;

/// 
/// RefCellStore is intended to help you automatically manage RefCell references with closures.
/// 
pub struct RefCellStore<T>
{

    refcell: RefCell<T>

}

impl<T> RefCellStore<T>
{

    pub fn new(state: T) -> Self
    {

        Self
        {

            refcell: RefCell::new(state)

        }

    }

    pub fn from_refcell(refcell: RefCell<T>) -> Self
    {

        Self
        {

            refcell

        }

    }

    pub fn refcell_ref(&self) -> &RefCell<T>
    {

        &self.refcell

    }

    //borrow

    pub fn borrow<F, R>(&self, mut func: F) -> R
        where F: FnMut(Ref<T>) -> R
    {

        let rfc_ref = self.refcell.borrow();

        func(rfc_ref)

    }

    pub fn borrow_with_param<P, F, R>(&self, param: P, mut func: F) -> R
        where F: FnMut(Ref<T>, P) -> R
    {

        let rfc_ref = self.refcell.borrow();

        func(rfc_ref, param)

    }

    pub fn borrow_with_ref<P, F, R>(&self, param: &P, mut func: F) -> R
        where F: FnMut(Ref<T>, &P) -> R
    {

        let rfc_ref = self.refcell.borrow();

        func(rfc_ref, param)

    }

    pub fn borrow_with_mut<P, F, R>(&self, param: &mut P, mut func: F) -> R
        where F: FnMut(Ref<T>, &mut P) -> R
    {

        let rfc_ref = self.refcell.borrow();

        func(rfc_ref, param)

    }

    pub fn get(&self) -> T
        where T: Clone
    {

        let rfc_ref = self.refcell.borrow();

        rfc_ref.clone()

    }

    //borrow_mut

    pub fn borrow_mut<F, R>(&self, mut func: F) -> R
        where F: FnMut(RefMut<T>) -> R
    {

        let rfc_mut = self.refcell.borrow_mut();

        func(rfc_mut)

    }

    pub fn borrow_mut_with_param<P, F, R>(&self, param: P, mut func: F) -> R
        where F: FnMut(RefMut<T>, P) -> R
    {

        let rfc_mut = self.refcell.borrow_mut();

        func(rfc_mut, param)

    }

    pub fn borrow_mut_with_ref<P, F, R>(&self, param: &P, mut func: F) -> R
        where F: FnMut(RefMut<T>, &P) -> R
    {

        let rfc_mut = self.refcell.borrow_mut();

        func(rfc_mut, param)

    }

    pub fn borrow_mut_with_mut<P, F, R>(&self, param: &mut P, mut func: F) -> R
        where F: FnMut(RefMut<T>, &mut P) -> R
    {

        let rfc_mut = self.refcell.borrow_mut();

        func(rfc_mut, param)

    }

    pub fn set(&self, item: T)
    {

        let mut rfc_mut = self.refcell.borrow_mut();

        *rfc_mut = item;

    }
    
    pub fn set_clone(&self, item: &T)
        where T: Clone
    {

        let mut rfc_mut = self.refcell.borrow_mut();

        *rfc_mut = item.clone();

    }

    //try_borrow

    pub fn try_borrow<F, R>(&self, mut func: F) -> Result<R, BorrowError>
        where F: FnMut(Ref<T>) -> R
    {

        let ref_res = self.refcell.try_borrow();

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

        let ref_res = self.refcell.try_borrow();

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

        let ref_res = self.refcell.try_borrow();

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

        let ref_res = self.refcell.try_borrow();

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

    pub fn try_get(&self) -> Result<T, BorrowError>
        where T: Clone
    {

        let ref_res = self.refcell.try_borrow();

        match ref_res
        {

            Ok(res) =>
            {

                Ok(res.clone())

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

        let mut_res = self.refcell.try_borrow_mut();

        match mut_res
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

        let mut_res = self.refcell.try_borrow_mut();

        match mut_res
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

        let mut_res = self.refcell.try_borrow_mut();

        match mut_res
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

        let mut_res = self.refcell.try_borrow_mut();

        match mut_res
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

    pub fn try_set(&self, item: T) -> Result<(), BorrowMutError>
    {

        let mut_res = self.refcell.try_borrow_mut();

        match mut_res
        {

            Ok(mut res) =>
            {

                *res = item;

                Ok(())

            }
            Err(err) =>
            {

                Err(err)

            }

        }

    }

    pub fn try_set_clone(&self, item: &T) -> Result<(), BorrowMutError>
        where T: Clone
    {

        let mut_res = self.refcell.try_borrow_mut();

        match mut_res
        {

            Ok(mut res) =>
            {

                *res = item.clone();

                Ok(())

            }
            Err(err) =>
            {

                Err(err)

            }

        }

    }

    pub fn take(&self) -> T
        where T: Default
    {

        self.refcell.take()

    }

    pub fn take_refcell(self) -> RefCell<T>
    {

        self.refcell

    }

}

impl<T> Debug for RefCellStore<T>
    where T: Debug
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RefCellStore").field("refcell", &self.refcell).finish()
    }
    
}
