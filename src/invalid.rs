
//For returning an invalid version of Self.
pub trait Invalid
{

    //Rust doesn't support const trait methods

    //const fn invalid() -> Self;

    fn invalid() -> Self;

}

