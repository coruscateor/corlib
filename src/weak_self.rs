use std::rc::Weak;

///
/// This trait is useful for implementing the weak-self design pattern. 
///
/// The weak-self design pattern basically involves a reference counted struct holding a weak reference to itself.
/// 
pub trait WeakSelf
{

    fn weak_self(&self) -> Weak<Self>;

    fn weak_self_ref(&self) -> &Weak<Self>;

}
