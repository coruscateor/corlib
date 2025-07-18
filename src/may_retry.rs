
///
/// Runs the provided expression in a loop until a false value is returned.
/// 
#[macro_export]
macro_rules! may_retry
{

    ($expr:expr) =>
    {
        
        {

            loop
            {

                if $expr == false
                {

                    break;

                }

            }

        }

    }

}

///
/// Runs the provided expression in a loop until the Some variant of an Option enum is returned.
/// 
/// Ultimately the value contained in the Option is returned.
/// 
#[macro_export]
macro_rules! may_retry_return
{

    ($expr:expr) =>
    {
        
        {

            let res_val;

            loop
            {

                let opt_val = $expr;

                if let Some(val) = opt_val
                {

                    res_val = val;

                    break;

                }

            }

            res_val

        }

    }

}



/*
pub fn may_retry<F, R>(func: F)
    where F: FnOnce(&R)
{



}
*/


