//!
//! Convert Options into certainty with the get_some declarative macro.
//! 
//! get_some will take one or more Option objects (up to 8) and return their taken values, panicing with GET_SOME_ERROR_MESSAGE if any of the provided Option values are None.
//! 

//Disabled

//pub static GET_SOME_ERROR_MESSAGE: &str = "Error: Nothing here!";

/// 
/// Takes one or more Option enum objects and returns the taken objects from those Options, panicking with "Error: Nothing here!" if at least one object isn’t found.
/// 
#[macro_export]
macro_rules! get_some
{

    ($option_name:ident) =>
    {

        {

            //use crate::get_some;

            //use corlib::get_some;

            let res = $option_name.take();

            res.expect("Error: Nothing here!")

            //res.expect(get_some::GET_SOME_ERROR_MESSAGE)

        }

    };
    ($option_name1:ident, $option_name2:ident) =>
    {

        {

            let res1 = get_some!($option_name1);

            let res2 = get_some!($option_name2);

            (res1, res2)

        }

    };
    ($option_name1:ident, $option_name2:ident, $option_name3:ident) =>
    {

        {

            let res1 = get_some!($option_name1);

            let res2 = get_some!($option_name2);

            let res3 = get_some!($option_name3);

            (res1, res2, res3)

        }

    };
    ($option_name1:ident, $option_name2:ident, $option_name3:ident, $option_name4:ident) =>
    {

        {

            let res1 = get_some!($option_name1);

            let res2 = get_some!($option_name2);

            let res3 = get_some!($option_name3);

            let res4 = get_some!($option_name4);

            (res1, res2, res3, res4)

        }

    };
    ($option_name1:ident, $option_name2:ident, $option_name3:ident, $option_name4:ident, $option_name5:ident) =>
    {

        {

            let res1 = get_some!($option_name1);

            let res2 = get_some!($option_name2);

            let res3 = get_some!($option_name3);

            let res4 = get_some!($option_name4);

            let res5 = get_some!($option_name5);

            (res1, res2, res3, res4, res5)

        }

    };
    ($option_name1:ident, $option_name2:ident, $option_name3:ident, $option_name4:ident, $option_name5:ident, $option_name6:ident) =>
    {

        {

            let res1 = get_some!($option_name1);

            let res2 = get_some!($option_name2);

            let res3 = get_some!($option_name3);

            let res4 = get_some!($option_name4);

            let res5 = get_some!($option_name5);

            let res6 = get_some!($option_name6);

            (res1, res2, res3, res4, res5, res6)

        }

    };
    ($option_name1:ident, $option_name2:ident, $option_name3:ident, $option_name4:ident, $option_name5:ident, $option_name6:ident, $option_name7:ident) =>
    {

        {

            let res1 = get_some!($option_name1);

            let res2 = get_some!($option_name2);

            let res3 = get_some!($option_name3);

            let res4 = get_some!($option_name4);

            let res5 = get_some!($option_name5);

            let res6 = get_some!($option_name6);

            let res7 = get_some!($option_name7);

            (res1, res2, res3, res4, res5, res6, res7)

        }

    };
    ($option_name1:ident, $option_name2:ident, $option_name3:ident, $option_name4:ident, $option_name5:ident, $option_name6:ident, $option_name7:ident, $option_name8:ident) =>
    {

        {

            let res1 = get_some!($option_name1);

            let res2 = get_some!($option_name2);

            let res3 = get_some!($option_name3);

            let res4 = get_some!($option_name4);

            let res5 = get_some!($option_name5);

            let res6 = get_some!($option_name6);

            let res7 = get_some!($option_name7);

            let res8 = get_some!($option_name8);

            (res1, res2, res3, res4, res5, res6, res7, res8)

        }

    }

}

/*

unresolved import `corlib`
use of undeclared crate or module `corlib`rustcClick for full compiler diagnostic
get_some.rs(50, 19): Error originated from macro call here

*/

/*
#[cfg(test)]
mod tests
{

    //use super::*;

    #[test]
    fn try_get_some()
    {

        let mut one_option = Option::Some(1);

        let res = get_some!(one_option);

        assert_eq!(1, res);

    }

}
*/
/*
#[cfg(test)]
mod tests
{

    //use super::*;

    #[test]
    fn try_get_some2()
    {

        let mut option1 = Option::Some(1);

        let mut option2 = Option::Some(2);

        let res = get_some!(option1, option2);

        assert_eq!(1, res.0);

        assert_eq!(2, res.1);

    }

}
*/