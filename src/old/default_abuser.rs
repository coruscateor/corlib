use std::ops::{Deref, DerefMut};

pub trait DefaultAbuser : Default + Deref + DerefMut
{

}

