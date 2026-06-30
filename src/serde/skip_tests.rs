use serde::{Deserialize, Serialize};

use super::Skip;

#[test]
fn skip_test()
{

    #[derive(Serialize, Deserialize, Default)]
    struct JustSkipContents;

    impl JustSkipContents
    {

        pub fn new() -> Self
        {

            Self
            {
            }

        }

    }

    #[derive(Serialize, Deserialize)]
    struct JustSkip
    {

        //#[serde(skip)]
        skip_this: Skip<JustSkipContents> //u32>

    }

    impl JustSkip
    {

        pub fn new() -> Self
        {

            Self
            {

                skip_this: Skip::new(JustSkipContents::new()) //0)

            }

        }

    }

    let js = JustSkip::new();

    let js_json = serde_json::to_string(&js).unwrap();

    println!("\n{}\n", js_json);

    let _new_j: JustSkip = serde_json::from_str(&js_json).unwrap();

    println!("Success!\n");

}

#[test]
fn better_skip_test()
{

    #[derive(Default)]
    struct IgnoreMe;

    impl IgnoreMe
    {

        pub fn new() -> Self
        {

            Self
            {
            }

        }

        /*
        pub fn blah(&self) -> u32
        {

            50

        }
        */

    }

    let ignore_me = IgnoreMe::new();

    let skip = Skip::new(ignore_me);

    //assert_eq!(skip.blah(), 50);

    let skip_json = serde_json::to_string(&skip).unwrap();

    println!("\n{}\n", skip_json);

    let _skip: Skip<IgnoreMe> = serde_json::from_str(&skip_json).unwrap();

    println!("Success!\n");

}