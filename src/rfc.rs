
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
