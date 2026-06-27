use std::{any::TypeId, fmt::write};

use serde::de::Expected;

pub struct GenericVisitorExpected
{

    type_id: TypeId

}

impl GenericVisitorExpected
{

    pub fn new(type_id: TypeId) -> Self
    {

        Self
        {

            type_id

        }

    }

}

impl Expected for GenericVisitorExpected
{

    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        
        write!(formatter, "Expected: {:?}", self.type_id)
        
    }

}