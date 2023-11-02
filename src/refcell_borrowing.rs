//use std::rc::Rc;

use std::cell::{RefCell, Ref, RefMut};

use std::ops::{Fn, FnMut, FnOnce};

//Fn

pub fn call_fn_ref<T, F>(rfc_obj: &RefCell<T>, func: F)
    where F: Fn(&Ref<T>)
{

    let obj_ref = rfc_obj.borrow();

    func(&obj_ref)

}


pub fn call_fnrv_ref<T, F, R>(rfc_obj: &RefCell<T>, func: F) -> R
    where F: Fn(&Ref<T>) -> R
{

    let obj_ref = rfc_obj.borrow();

    func(&obj_ref)

}

pub fn call_fn_refmut<T, F>(rfc_obj: &RefCell<T>, func: F)
    where F: Fn(&mut RefMut<T>)
{

    let mut obj_mut = rfc_obj.borrow_mut();

    func(&mut obj_mut)

}


pub fn call_fnrv_refmut<T, F, R>(rfc_obj: &RefCell<T>, func: F) -> R
    where F: Fn(&mut RefMut<T>) -> R
{

    let mut obj_mut = rfc_obj.borrow_mut();

    func(&mut obj_mut)

}

//Plus a Parameter

pub fn call_fn_ref_param<T, F, P>(rfc_obj: &RefCell<T>, func: F, param: P)
    where F: Fn(&Ref<T>, P)
{

    let obj_ref = rfc_obj.borrow();

    func(&obj_ref, param)

}


pub fn call_fnrv_ref_param<T, F, R, P>(rfc_obj: &RefCell<T>, func: F, param: P) -> R
    where F: Fn(&Ref<T>, P) -> R
{

    let obj_ref = rfc_obj.borrow();

    func(&obj_ref, param)

}

pub fn call_fn_refmut_param<T, F, P>(rfc_obj: &RefCell<T>, func: F, param: P)
    where F: Fn(&mut RefMut<T>, P)
{

    let mut obj_mut = rfc_obj.borrow_mut();

    func(&mut obj_mut, param)

}


pub fn call_fnrv_refmut_param<T, F, R, P>(rfc_obj: &RefCell<T>, func: F, param: P) -> R
    where F: Fn(&mut RefMut<T>, P) -> R
{

    let mut obj_mut = rfc_obj.borrow_mut();

    func(&mut obj_mut, param)

}

//FnMut

pub fn call_fnmut_ref<T, F>(rfc_obj: &RefCell<T>, mut func: F)
    where F: FnMut(&Ref<T>)
{

    let obj_ref = rfc_obj.borrow();

    func(&obj_ref)

}


pub fn call_fnmutrv_ref<T, F, R>(rfc_obj: &RefCell<T>, mut func: F) -> R
    where F: FnMut(&Ref<T>) -> R
{

    let obj_ref = rfc_obj.borrow();

    func(&obj_ref)

}

pub fn call_fnmut_refmut<T, F>(rfc_obj: &RefCell<T>, mut func: F)
    where F: FnMut(&mut RefMut<T>)
{

    let mut obj_mut = rfc_obj.borrow_mut();

    func(&mut obj_mut)

}


pub fn call_fnmutrv_refmut<T, F, R>(rfc_obj: &RefCell<T>, mut func: F) -> R
    where F: FnMut(&mut RefMut<T>) -> R
{

    let mut obj_mut = rfc_obj.borrow_mut();

    func(&mut obj_mut)

}

//Plus a Parameter

pub fn call_fnmut_ref_param<T, F, P>(rfc_obj: &RefCell<T>, mut func: F, param: P)
    where F: FnMut(&Ref<T>, P)
{

    let obj_ref = rfc_obj.borrow();

    func(&obj_ref, param)

}

pub fn call_fnmutrv_ref_param<T, F, R, P>(rfc_obj: &RefCell<T>, mut func: F, param: P) -> R
    where F: FnMut(&Ref<T>, P) -> R
{

    let obj_ref = rfc_obj.borrow();

    func(&obj_ref, param)

}

pub fn call_fnmut_refmut_param<T, F, P>(rfc_obj: &RefCell<T>, mut func: F, param: P)
    where F: FnMut(&mut RefMut<T>, P)
{

    let mut obj_mut = rfc_obj.borrow_mut();

    func(&mut obj_mut, param)

}


pub fn call_fnmutrv_refmut_param<T, F, R, P>(rfc_obj: &RefCell<T>, mut func: F, param: P) -> R
    where F: FnMut(&mut RefMut<T>, P) -> R
{

    let mut obj_mut = rfc_obj.borrow_mut();

    func(&mut obj_mut, param)

}

//FnOnce

pub fn call_fnonce_ref<T, F>(rfc_obj: &RefCell<T>, func: F)
    where F: FnOnce(&Ref<T>)
{

    let obj_ref = rfc_obj.borrow();

    func(&obj_ref)

}


pub fn call_fnoncerv_ref<T, F, R>(rfc_obj: &RefCell<T>, func: F) -> R
    where F: FnOnce(&Ref<T>) -> R
{

    let obj_ref = rfc_obj.borrow();

    func(&obj_ref)

}

pub fn call_fnonce_refmut<T, F>(rfc_obj: &RefCell<T>, func: F)
    where F: FnOnce(&mut RefMut<T>)
{

    let mut obj_mut = rfc_obj.borrow_mut();

    func(&mut obj_mut)

}


pub fn call_fnmoncerv_refmut<T, F, R>(rfc_obj: &RefCell<T>, func: F) -> R
    where F: FnOnce(&mut RefMut<T>) -> R
{

    let mut obj_mut = rfc_obj.borrow_mut();

    func(&mut obj_mut)

}

//Plus a Parameter

pub fn call_fnonce_ref_param<T, F, P>(rfc_obj: &RefCell<T>, func: F, param: P)
    where F: FnOnce(&Ref<T>, P)
{

    let obj_ref = rfc_obj.borrow();

    func(&obj_ref, param)

}


pub fn call_fnoncerv_ref_param<T, F, R, P>(rfc_obj: &RefCell<T>, func: F, param: P) -> R
    where F: FnOnce(&Ref<T>, P) -> R
{

    let obj_ref = rfc_obj.borrow();

    func(&obj_ref, param)

}

pub fn call_fnonce_refmut_param<T, F, P>(rfc_obj: &RefCell<T>,  func: F, param: P)
    where F: FnOnce(&mut RefMut<T>, P)
{

    let mut obj_mut = rfc_obj.borrow_mut();

    func(&mut obj_mut, param)

}

pub fn call_fnmoncerv_refmut_param<T, F, R, P>(rfc_obj: &RefCell<T>, func: F, param: P) -> R
    where F: FnOnce(&mut RefMut<T>, P) -> R
{

    let mut obj_mut = rfc_obj.borrow_mut();

    func(&mut obj_mut, param)

}

//
//Calls borrow on a Refcell or compatible type in a block and makes the resultant reference available to the provided block.
//
//The resultant reference has the name of the provided rfc_obj with "_ref" appended.
//
//Requires
//
//use paste::paste;
//
//in module scope.
//
#[macro_export]
macro_rules! rfc_ref
{

    ($rfc_obj:ident, $rfc_block:block) =>
    {

        {

            paste! {

                let [<$rfc_obj _ref>] = rfc_obj.borrow();

                $rfc_block

            }

        }

    }

}

//
//Calls borrow_mut on a Refcell or compatible type in a block and makes the resultant reference available to the provided block.
//
//The resultant reference has the name of the provided rfc_obj with "_mut" appended.
//
//Requires
//
//use paste::paste;
//
//in module scope.
//
#[macro_export]
macro_rules! rfc_mut
{

    ($rfc_obj:ident, $rfc_block:block) =>
    {

        {

            paste! {

                let mut [<$rfc_obj _mut>] = rfc_obj.borrow_mut();

                $rfc_block

            }

        }

    }

}

pub mod rc
{

    use std::rc::Rc;

    use std::cell::{RefCell, Ref, RefMut};

    use std::ops::{Fn, FnMut, FnOnce};

    pub fn call_fn_ref<T, F>(rc_obj: &Rc<RefCell<T>>, func: F)
        where F: Fn(&Ref<T>)
    {

        let obj_ref = rc_obj.borrow();

        func(&obj_ref)

    }


    pub fn call_fnrv_ref<T, F, R>(rc_obj: &Rc<RefCell<T>>, func: F) -> R
        where F: Fn(&Ref<T>) -> R
    {

        let obj_ref = rc_obj.borrow();

        func(&obj_ref)

    }

    pub fn call_fn_refmut<T, F>(rc_obj: &Rc<RefCell<T>>, func: F)
        where F: Fn(&mut RefMut<T>)
    {

        let mut obj_mut = rc_obj.borrow_mut();

        func(&mut obj_mut)

    }


    pub fn call_fnrv_refmut<T, F, R>(rc_obj: &Rc<RefCell<T>>, func: F) -> R
        where F: Fn(&mut RefMut<T>) -> R
    {

        let mut obj_mut = rc_obj.borrow_mut();

        func(&mut obj_mut)

    }

    //Plus a Parameter

    pub fn call_fn_ref_param<T, F, P>(rc_obj: &Rc<RefCell<T>>, func: F, param: P)
        where F: Fn(&Ref<T>, P)
    {

        let obj_ref = rc_obj.borrow();

        func(&obj_ref, param)

    }


    pub fn call_fnrv_ref_param<T, F, R, P>(rc_obj: &Rc<RefCell<T>>, func: F, param: P) -> R
        where F: Fn(&Ref<T>, P) -> R
    {

        let obj_ref = rc_obj.borrow();

        func(&obj_ref, param)

    }

    pub fn call_fn_refmut_param<T, F, P>(rc_obj: &Rc<RefCell<T>>, func: F, param: P)
        where F: Fn(&mut RefMut<T>, P)
    {

        let mut obj_mut = rc_obj.borrow_mut();

        func(&mut obj_mut, param)

    }

    pub fn call_fnrv_refmut_param<T, F, R, P>(rc_obj: &Rc<RefCell<T>>, func: F, param: P) -> R
        where F: Fn(&mut RefMut<T>, P) -> R
    {

        let mut obj_mut = rc_obj.borrow_mut();

        func(&mut obj_mut, param)

    }

    //FnMut

    pub fn call_fnmut_ref<T, F>(rc_obj: &Rc<RefCell<T>>, mut func: F)
        where F: FnMut(&Ref<T>)
    {

        let obj_ref = rc_obj.borrow();

        func(&obj_ref)

    }


    pub fn call_fnmutrv_ref<T, F, R>(rc_obj: &Rc<RefCell<T>>, mut func: F) -> R
        where F: FnMut(&Ref<T>) -> R
    {

        let obj_ref = rc_obj.borrow();

        func(&obj_ref)

    }

    pub fn call_fnmut_refmut<T, F>(rc_obj: &Rc<RefCell<T>>, mut func: F)
        where F: FnMut(&mut RefMut<T>)
    {

        let mut obj_mut = rc_obj.borrow_mut();

        func(&mut obj_mut)

    }


    pub fn call_fnmutrv_refmut<T, F, R>(rc_obj: &Rc<RefCell<T>>, mut func: F) -> R
        where F: FnMut(&mut RefMut<T>) -> R
    {

        let mut obj_mut = rc_obj.borrow_mut();

        func(&mut obj_mut)

    }

    //Plus a Parameter

    pub fn call_fnmut_ref_param<T, F, P>(rc_obj: &Rc<RefCell<T>>, mut func: F, param: P)
    where F: FnMut(&Ref<T>, P)
    {

        let obj_ref = rc_obj.borrow();

        func(&obj_ref, param)

    }

    pub fn call_fnmutrv_ref_param<T, F, R, P>(rc_obj: &Rc<RefCell<T>>, mut func: F, param: P) -> R
        where F: FnMut(&Ref<T>, P) -> R
    {

        let obj_ref = rc_obj.borrow();

        func(&obj_ref, param)

    }

    pub fn call_fnmut_refmut_param<T, F, P>(rc_obj: &Rc<RefCell<T>>, mut func: F, param: P)
        where F: FnMut(&mut RefMut<T>, P)
    {

        let mut obj_mut = rc_obj.borrow_mut();

        func(&mut obj_mut, param)

    }


    pub fn call_fnmutrv_refmut_param<T, F, R, P>(rc_obj: &Rc<RefCell<T>>, mut func: F, param: P) -> R
        where F: FnMut(&mut RefMut<T>, P) -> R
    {

        let mut obj_mut = rc_obj.borrow_mut();

        func(&mut obj_mut, param)

    }

    //FnOnce

    pub fn call_fnonce_ref<T, F>(rc_obj: &Rc<RefCell<T>>, func: F)
        where F: FnOnce(&Ref<T>)
    {

        let obj_ref = rc_obj.borrow();

        func(&obj_ref)

    }


    pub fn call_fnoncerv_ref<T, F, R>(rc_obj: &Rc<RefCell<T>>, func: F) -> R
        where F: FnOnce(&Ref<T>) -> R
    {

        let obj_ref = rc_obj.borrow();

        func(&obj_ref)

    }

    pub fn call_fnonce_refmut<T, F>(rc_obj: &Rc<RefCell<T>>, func: F)
        where F: FnOnce(&mut RefMut<T>)
    {

        let mut obj_mut = rc_obj.borrow_mut();

        func(&mut obj_mut)

    }


    pub fn call_fnmoncerv_refmut<T, F, R>(rc_obj: &Rc<RefCell<T>>, func: F) -> R
        where F: FnOnce(&mut RefMut<T>) -> R
    {

        let mut obj_mut = rc_obj.borrow_mut();

        func(&mut obj_mut)

    }

    //Plus a Parameter

    pub fn call_fnonce_ref_param<T, F, P>(rc_obj: &Rc<RefCell<T>>, func: F, param: P)
        where F: FnOnce(&Ref<T>, P)
    {

        let obj_ref = rc_obj.borrow();

        func(&obj_ref, param)

    }


    pub fn call_fnoncerv_ref_param<T, F, R, P>(rc_obj: &Rc<RefCell<T>>, func: F, param: P) -> R
        where F: FnOnce(&Ref<T>, P) -> R
    {

        let obj_ref = rc_obj.borrow();

        func(&obj_ref, param)

    }

    pub fn call_fnonce_refmut_param<T, F, P>(rc_obj: &Rc<RefCell<T>>,  func: F, param: P)
        where F: FnOnce(&mut RefMut<T>, P)
    {

        let mut obj_mut = rc_obj.borrow_mut();

        func(&mut obj_mut, param)

    }

    pub fn call_fnmoncerv_refmut_param<T, F, R, P>(rc_obj: &Rc<RefCell<T>>, func: F, param: P) -> R
        where F: FnOnce(&mut RefMut<T>, P) -> R
    {

        let mut obj_mut = rc_obj.borrow_mut();

        func(&mut obj_mut, param)

    }

}

