
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

///
/// Runs the provided expression in a loop until a false value is returned, eagerly incrementing the provided $count variable.
/// 
/// When a $limit vairable is provided, the loop now increments the count, checks if the limit has been reached or passed and returns a bool indicating this (true if limit reached, false if not).
/// 
#[macro_export]
macro_rules! may_retry_count
{

    ($expr:expr, $count:ident) =>
    {
        
        {

            loop
            {

                $count += 1;

                if $expr == false
                {

                    break;

                }

            }

        }

    };
    ($expr:expr, $count:ident, $limit:ident) =>
    {
        
        {

            let retry_limit_reached = false;

            loop
            {

                $count += 1;

                if $expr == false
                {

                    break;

                }

                if $count => $limit
                {

                    retry_limit_reached = true;

                }

            }

            retry_limit_reached

        }

    }

}

///
/// Runs the provided expression in a loop until the Some variant of an Option enum is returned.
/// 
/// Ultimately the value contained in the Option is returned.
/// 
/// Increments the provided $count variable.
/// 
/// When a $limit is provided, the $count must be lower than $count.
/// 
/// Returns a tuple with the optional value and bool indicating whether or not the retry limit was reached.
/// 
#[macro_export]
macro_rules! may_retry_count_return
{

    ($expr:expr, $count:ident) =>
    {
        
        {

            let res_val;

            loop
            {

                $count += 1;

                let opt_val = $expr;

                if let Some(val) = opt_val
                {

                    res_val = val;

                    break;

                }

            }

            res_val

        }

    };
    ($expr:expr, $count:ident, $limit:ident) =>
    {

        {

            let retry_limit_reached = false;

            let res_val;

            loop
            {

                $count += 1;

                let opt_val = $expr;

                if let Some(val) = opt_val
                {

                    res_val = val;

                    break;

                }

                if $count => $limit
                {

                    retry_limit_reached = true;

                }

            }

            (res_val, retry_limit_reached)

        }

    }

}

/*
pub fn may_retry<F, R>(func: F)
    where F: FnOnce(&R)
{



}
*/


