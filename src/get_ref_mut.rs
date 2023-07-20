
pub trait GetRef<T>
{

    fn get_ref(&self) -> &T;

}

pub trait MutSelfGetRef<T>
{

    fn get_ref(&mut self) -> &T;

}

pub trait GetMut<T>
{

    fn get_mut(&mut self) -> &mut T;    

}

pub trait TryGetRef<T>
{

    fn try_get_ref(&self) -> Option<&T>;

}


pub trait TryGetMut<T>
{

    fn try_get_mut(&mut self) -> Option<&mut T>;    

}

