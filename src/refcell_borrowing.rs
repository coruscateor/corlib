use std::cell::{RefCell, Ref, RefMut};

use std::ops::{Fn, FnMut, FnOnce};

//All rfc related macros in getters_setters_callers should be moved here.

//Naming:

//rv: Return Value

//Fn

pub fn call_fn_ref<T, F>(rfc_obj: &RefCell<T>, func: F)
    where F: Fn(&Ref<T>)
{

    let obj_ref = rfc_obj.borrow();

    func(&obj_ref)

}


pub fn call_fn_rv_ref<T, F, R>(rfc_obj: &RefCell<T>, func: F) -> R
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


pub fn call_fn_rv_refmut<T, F, R>(rfc_obj: &RefCell<T>, func: F) -> R
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


pub fn call_fn_rv_ref_param<T, F, R, P>(rfc_obj: &RefCell<T>, func: F, param: P) -> R
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


pub fn call_fn_rv_refmut_param<T, F, R, P>(rfc_obj: &RefCell<T>, func: F, param: P) -> R
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


pub fn call_fnmut_rv_ref<T, F, R>(rfc_obj: &RefCell<T>, mut func: F) -> R
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


pub fn call_fnmut_rv_refmut<T, F, R>(rfc_obj: &RefCell<T>, mut func: F) -> R
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

pub fn call_fnmut_rv_ref_param<T, F, R, P>(rfc_obj: &RefCell<T>, mut func: F, param: P) -> R
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


pub fn call_fnmut_rv_refmut_param<T, F, R, P>(rfc_obj: &RefCell<T>, mut func: F, param: P) -> R
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


pub fn call_fnonce_rv_ref<T, F, R>(rfc_obj: &RefCell<T>, func: F) -> R
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


pub fn call_fnonce_rv_refmut<T, F, R>(rfc_obj: &RefCell<T>, func: F) -> R
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


pub fn call_fnonce_rv_ref_param<T, F, R, P>(rfc_obj: &RefCell<T>, func: F, param: P) -> R
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

pub fn call_fnonce_rv_refmut_param<T, F, R, P>(rfc_obj: &RefCell<T>, func: F, param: P) -> R
    where F: FnOnce(&mut RefMut<T>, P) -> R
{

    let mut obj_mut = rfc_obj.borrow_mut();

    func(&mut obj_mut, param)

}

//These macros probably don't work

///
///Calls borrow on a std::cell::RefCell or compatible type in a block and makes the resultant reference available to the provided block.
///
///In the first match rule the RefCell is expected to be part of self with the name of "rfc_object".
///
///"Borrow()" is called on this RefCell object which results in "rfc_object_ref" being made available to the provided block.
///
///The second match rule requires a RefCell object for "rfc_obj" and a bock ("rfc_block").
///
///The name of the provided "rfc_obj" parameter and "_ref" are appended to create the reference name which the result of "rfc_obj.borrow()" is placed into and made available to the provided block ("rfc_block").
///
///Requires:
///
///The paste crate and:
///
///use paste::paste;
///
///in module scope for the second match rule only.
///
#[macro_export]
macro_rules! rfc_ref
{

    ($rfc_block:block) =>
    {

        {

            paste! {

                let rfc_object_ref = rfc_object.borrow();

                $rfc_block

            }

        }

    };
    ($rfc_obj:ident, $rfc_block:block) =>
    {

        {

            paste! {

                let [<$rfc_obj _ref>] = $rfc_obj.borrow();

                $rfc_block

            }

        }

    }

}

#[macro_export]
macro_rules! owned_rfc_ref
{

    ($owner:ident, $rfc_block:block) =>
    {

        {

            paste! {

                let rfc_object_ref = $owner.rfc_object.borrow();

                $rfc_block

            }

        }

    };
    ($owner:ident, $rfc_obj:ident, $rfc_block:block) =>
    {

        {

            paste! {

                let [<$rfc_obj _ref>] = $owner.$rfc_obj.borrow();

                $rfc_block

            }

        }

    }

}


//Methods

///
///Impelments the rfc_ref macro in a method.
///
#[macro_export]
macro_rules! impl_owned_rfc_ref_method
{

    ($method_name:ident, $rfc_block:block) =>
    {

        pub fn $method_name(&self)
        {

            owned_rfc_ref!(self, $rfc_block);

        }

    };
    ($method_name:ident, $rfc_obj:ident, $rfc_block:block) =>
    {

        pub fn $method_name(&self)
        {

            owned_rfc_ref!(self, $rfc_obj, $rfc_block);

        }

    };
    ($method_name:ident, $rfc_block:block, $return_type:ty) =>
    {

        pub fn $method_name(&self) -> $return_type
        {

            owned_rfc_ref!(self, $rfc_block);

        }

    };
    ($method_name:ident, $rfc_obj:ident, $rfc_block:block, $return_type:ty) =>
    {

        pub fn $method_name(&self) -> $return_type
        {

            owned_rfc_ref!(self, $rfc_obj, $rfc_block);

        }

    }

}

///
///Calls borrow_mut on a std::cell::RefCell or compatible type in a block and makes the resultant reference available to the provided block.
///
///In the first match rule the RefCell is expected to be part of self with the name of "rfc_object".
///
///"Borrow_mut()" is called on this RefCell object which results in "rfc_object_mut" being made available to the provided block.
///
///The second match rule requires a RefCell object for "rfc_obj" and a bock ("rfc_block").
///
///The name of the provided "rfc_obj" parameter and "_mut" are appended to create the reference name which the result of "rfc_obj.borrow_mut()" is placed into and made available to the provided block ("rfc_block").
///
///Requires:
///
///The paste crate and:
///
///use paste::paste;
///
///in module scope for the second match rule only.
///
#[macro_export]
macro_rules! rfc_mut
{

    ($rfc_block:block) =>
    {

        {

            paste! {

                let mut rfc_object_mut = rfc_object.borrow_mut();

                $rfc_block

            }

        }

    };
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

#[macro_export]
macro_rules! self_rfc_mut
{

    ($rfc_block:block) =>
    {

        {

            paste! {

                let mut rfc_object_mut = self.rfc_object.borrow_mut();

                $rfc_block

            }

        }

    };
    ($rfc_obj:ident, $rfc_block:block) =>
    {

        {

            paste! {

                let mut [<$rfc_obj _mut>] = self.$rfc_obj.borrow_mut();

                $rfc_block

            }

        }

    }

}

//Methods

///
///Impelments the rfc_mut macro in a method.
///
#[macro_export]
macro_rules! impl_self_rfc_mut_method
{

    ($method_name:ident, $rfc_block:block) =>
    {

        pub fn $method_name(&self)
        {

            self_rfc_mut!($rfc_block);

        }

    };
    ($method_name:ident, $rfc_obj:ident, $rfc_block:block) =>
    {

        pub fn $method_name(&self)
        {

            self_rfc_mut!($rfc_obj, $rfc_block);

        }

    };
    ($method_name:ident, $rfc_block:block, $return_type:ty) =>
    {

        pub fn $method_name(&self) -> $return_type
        {

            self_rfc_ref!($rfc_block);

        }

    };
    ($method_name:ident, $rfc_obj:ident, $rfc_block:block, $return_type:ty) =>
    {

        pub fn $method_name(&self) -> $return_type
        {

            self_rfc_ref!($rfc_obj, $rfc_block);

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


    pub fn call_fn_rv_ref<T, F, R>(rc_obj: &Rc<RefCell<T>>, func: F) -> R
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


    pub fn call_fn_rv_refmut<T, F, R>(rc_obj: &Rc<RefCell<T>>, func: F) -> R
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


    pub fn call_fn_rv_ref_param<T, F, R, P>(rc_obj: &Rc<RefCell<T>>, func: F, param: P) -> R
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

    pub fn call_fn_rv_refmut_param<T, F, R, P>(rc_obj: &Rc<RefCell<T>>, func: F, param: P) -> R
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


    pub fn call_fnmut_rv_ref<T, F, R>(rc_obj: &Rc<RefCell<T>>, mut func: F) -> R
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


    pub fn call_fnmut_rv_refmut<T, F, R>(rc_obj: &Rc<RefCell<T>>, mut func: F) -> R
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

    pub fn call_fnmut_rv_ref_param<T, F, R, P>(rc_obj: &Rc<RefCell<T>>, mut func: F, param: P) -> R
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


    pub fn call_fnmut_rv_refmut_param<T, F, R, P>(rc_obj: &Rc<RefCell<T>>, mut func: F, param: P) -> R
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


    pub fn call_fnonce_rv_ref<T, F, R>(rc_obj: &Rc<RefCell<T>>, func: F) -> R
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


    pub fn call_fnonce_rv_refmut<T, F, R>(rc_obj: &Rc<RefCell<T>>, func: F) -> R
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


    pub fn call_fnonce_rv_ref_param<T, F, R, P>(rc_obj: &Rc<RefCell<T>>, func: F, param: P) -> R
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

    pub fn call_fnonce_rv_refmut_param<T, F, R, P>(rc_obj: &Rc<RefCell<T>>, func: F, param: P) -> R
        where F: FnOnce(&mut RefMut<T>, P) -> R
    {

        let mut obj_mut = rc_obj.borrow_mut();

        func(&mut obj_mut, param)

    }

}

#[cfg(test)]
mod tests
{

    //use super::*;

    /*
    use std::cell::RefCell;

    use paste::paste;

    struct RFCDTestObject
    {

        value: i32

    }

    impl RFCDTestObject
    {

        fn get_value(&self) -> i32
        {

            self.value

        }

        fn set_value(&mut self, value: i32)
        {

            self.value = value;

        }

    }

    struct RFCContinerTestObject
    {

        rfc_object: RefCell<RFCDTestObject>

    }

    impl RFCContinerTestObject
    {

        impl_owned_rfc_ref_method!(get_value, {

            //rfc_object_ref.

            rfc_object_ref.get_value()

        });

    }
    */

    #[test]
    fn rfc_methods_test()
    {


    }

}

