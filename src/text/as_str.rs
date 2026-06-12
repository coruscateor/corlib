
///
/// Provides an as_str as a trait method.
/// 
pub trait AsStr
{

    fn as_str(&self) -> &str;

}

pub trait AsMutStr : AsStr
{

    fn as_mut_str(&self) -> &mut str;

}

