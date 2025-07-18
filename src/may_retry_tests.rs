use crate::{may_retry, may_retry_return};

#[test]
fn once_twice()
{

    let mut inc = 0;

    may_retry!({

        let is_one = inc == 1;

        println!("once_twice - inc: {}", inc);

        inc += 1;

        !is_one

    });  

}

#[test]
fn once_twice_return()
{

    let mut inc = 0;

    let returned = may_retry_return!({

        let is_one = inc == 1;

        println!("once_twice_return - inc: {}", inc);

        if is_one
        {

            Some(inc)

        }
        else
        {

            inc += 1;

            None
            
        }

    });

    println!("once_twice_return - returned: {}", returned);

}

