use std::rc::{Rc, Weak};

use std::cell::RefCell;

pub trait GetRcOrWeakSelf
{

    fn get_rc_self(&self) -> Rc<Self>;

    fn get_weak_self(&self) -> Weak<Self>;
    
}

pub trait GetRcOrWeakRefCellSelf
{

    fn get_rc_self(&self) -> Rc<RefCell<Self>>;

    fn get_weak_self(&self) -> Weak<RefCell<Self>>;
    
}

//rc_self setup in constructors

//Rc<Type>

#[macro_export]
macro_rules! rc_self_setup
{

    //this, weak_self

    //weak_self must be a RefCell 

    ($rc_self:ident, $weak_self:ident) =>
    {

        {

            let mut weak_self_mut = $rc_self.$weak_self.borrow_mut();

            weak_self_mut.set(Rc::downgrade(&$rc_self));

        }

    }

}

//Rc<RefCell<Type>>

#[macro_export]
macro_rules! rc_self_rfc_setup //rc_self_refcell_setup
{

    //this, weak_self

    ($rc_self:ident, $weak_self:ident) =>
    {

        {

            let mut rc_self_mut = $rc_self.borrow_mut();

            rc_self_mut.$weak_self.set(Rc::downgrade(&$rc_self));

        }

    }

}

//

#[macro_export]
macro_rules! rc_self_setup_named
{

    //this, weak_self

    //weak_self must be a RefCell 

    () =>
    {

        {

            let mut weak_self_mut = rc_self.weak_self.borrow_mut();

            weak_self_mut.set(Rc::downgrade(rc_self));

        }

    }

}

#[macro_export]
macro_rules! rc_self_refcell_setup_named
{

    //this, weak_self

    () =>
    {

        {

            let mut rc_self_mut = rc_self.borrow_mut();

            rc_self_mut.weak_self.set(Rc::downgrade(rc_self));

        }

    }

}

//

#[macro_export]
macro_rules! rc_self_init_setup_returned
{

    //this, weak_self

    //weak_self must be a RefCell 

    ($this:ident, $weak_self:ident) =>
    {

        let rc_self = Rc::new($this);

        {

            let mut weak_self_mut = rc_self.$weak_self.borrow_mut();

            weak_self_mut.set(Rc::downgrade(&rc_self));

        }

        rc_self

    }

}

#[macro_export]
macro_rules! rc_self_init_refcell_setup_returned
{

    //this, weak_self

    ($this:ident, $weak_self:ident) =>
    {

        let rc_self = Rc::new(RefCell::new($this));

        //{

            let mut rc_self_mut = rc_self.borrow_mut();

            rc_self_mut.$weak_self.set(Rc::downgrade(&rc_self));

        //}

        rc_self

    }

}

//

#[macro_export]
macro_rules! rc_self_init_setup_returned_named
{

    //this, weak_self

    //weak_self must be a RefCell 

    () =>
    {

        let rc_self = Rc::new(this);

        {

            let mut weak_self_mut = rc_self.weak_self.borrow_mut();

            weak_self_mut.set(Rc::downgrade(&rc_self));

        }

        rc_self

    }

}

#[macro_export]
macro_rules! rc_self_init_refcell_setup_returned_named
{

    //this, weak_self

    () =>
    {

        let rc_self = Rc::new(RefCell::new(this));

        {

            let mut rc_self_mut = rc_self.borrow_mut();

            rc_self_mut.weak_self.set(Rc::downgrade(&rc_self));

        }

        rc_self

    }

}