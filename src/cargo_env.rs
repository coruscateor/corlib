
#[macro_export]
macro_rules! impl_pub_env_accessor
{

    ($fn_name:ident, $name:expr) =>
    {

        pub fn $fn_name() -> &'static str
        {

            env!($name)

        }

    }

}

//https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-crates

///
/// test
/// 
impl_pub_env_accessor!(cargo, "CARGO");


