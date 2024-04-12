
#[deprecated(since = "0.2.0")]
pub trait GetRef<T>
{

    fn get_ref(&self) -> &T;

}

#[deprecated(since = "0.2.0")]
pub trait MutSelfGetRef<T>
{

    fn get_ref(&mut self) -> &T;

}

#[deprecated(since = "0.2.0")]
pub trait GetMut<T>
{

    fn get_mut(&mut self) -> &mut T;    

}

#[deprecated(since = "0.2.0")]
pub trait TryGetRef<T>
{

    fn try_get_ref(&self) -> Option<&T>;

}

#[deprecated(since = "0.2.0")]
pub trait TryGetMut<T>
{

    fn try_get_mut(&mut self) -> Option<&mut T>;    

}

