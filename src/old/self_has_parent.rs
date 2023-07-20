
pub trait SelfHasParent<T>
{

    fn get_parent(&self) -> T;

}

pub trait SelfHasParentRef<'a, T>
{

    fn get_parent_ref(&self) -> &'a T;

}
