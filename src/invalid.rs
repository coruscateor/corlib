
//For returning an invalid version of Self.
#[deprecated(since = "0.2.0")]
pub trait Invalid
{

    //Rust doesn't support const trait methods

    //const fn invalid() -> Self;

    fn invalid() -> Self;

}

