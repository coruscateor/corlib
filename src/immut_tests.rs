use core::fmt;
use std::fmt::Display;

use crate::Immut;

use serde::{Deserialize, Serialize};
use serde_json::Result;

#[test]
fn serialize_deserialize_test()
{

    let val = Immut::<u32>::new(123);

    let val_string = serde_json::to_string(&val).unwrap();

    println!("\n{}\n", val_string);

    let val_from_str: Immut::<u32> = serde_json::from_str(&val_string).unwrap();

    println!("{}\n", val_from_str);

    println!("Done\n");

}

#[test]
fn serialize_deserialize_test_2()
{

    #[derive(Serialize, Deserialize)]
    struct Something
    {

        a_string: String,
        a_number: i64,
        a_i8_option: Option<i8>,
        a_u32_option: Option<u32>,
        a_unit: ()

    }

    impl Something
    {

        pub fn new() -> Self
        {

            Self
            {

                a_string: "test".to_string(),
                a_number: 568,
                a_i8_option: Some(23),
                a_u32_option: None,
                a_unit: ()

            }


        }

    }

    impl Display for Something
    {

        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
        {

            write!(f, "a_string: {}, a_number: {}, a_i8_option: {:?}, a_u32_option: {:?}, a_unit: {:?}", self.a_string, self.a_number, self.a_i8_option, self.a_u32_option, self.a_unit)

        }
        
    }

    let val = Immut::new(Something::new());

    let val_string = serde_json::to_string(&val).unwrap();

    println!("\n{}\n", val_string);

    let val_from_str: Immut::<Something> = serde_json::from_str(&val_string).unwrap();

    println!("{}\n", val_from_str);

    println!("Done\n");

}
