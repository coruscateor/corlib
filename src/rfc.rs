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
