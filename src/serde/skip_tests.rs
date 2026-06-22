use serde::{Deserialize, Serialize};

use super::Skip;

#[test]
fn skip_test()
{

    #[derive(Serialize, Deserialize, Default)]
    struct JustSkipContents
    {

    }

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