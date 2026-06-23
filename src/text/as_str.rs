
///
/// Requires that an as_str method be implemented.
/// 
pub trait AsStr
{

    fn as_str(&self) -> &str;

}

///
/// Requires that as_str and as_mut_str methods be implemented.
/// 
pub trait AsMutStr : AsStr
{

    fn as_mut_str(&self) -> &mut str;

}
