
static NOT_INSTANTIATED_ERROR_MSG: &str = "VarNonOption<T>: Not instantiated with a value";

pub struct VarNonOption<T>
{

    value: Option<T>

}

impl<T> VarNonOption<T>
{

    pub fn new(value: T) -> Self
    {

        Self
        {

            value: Some(value)

        }

    }

    pub fn invalid() -> Self
    {

        Self
        {

            value: None

        }

    }

    
    pub fn get_ref(&self) -> &T
    {

        self.value.as_ref().expect(NOT_INSTANTIATED_ERROR_MSG)

    }
    
    pub fn get_mut(&mut self) -> &mut T
    {

        self.value.as_mut().expect(NOT_INSTANTIATED_ERROR_MSG)

    }

    /*
    pub fn set_value(&mut self, value: T)
    {

        self.value = Some(value);

    }

    pub fn as_ref(&self) -> Option<&T>
    {

        self.value.as_ref()

    }

    pub fn as_mut(&mut self) -> Option<&mut T>
    {

        self.value.as_mut()

    }
    */

    pub fn set(&mut self, value: T)
    {

        self.value = Some(value);

    }

    pub fn is_valid(&self) -> bool
    {

        self.value.is_some()

    }

    pub fn is_invalid(&self) -> bool
    {

        self.value.is_none()

    }

    pub fn unwrap_or(self, default: T) -> T
    {

        self.value.unwrap_or(default)

    }

}


impl<T> Default for VarNonOption<T>
    where T: Default
{

    fn default() -> Self
    {

        VarNonOption::new(Default::default())

    }

}

impl<T> VarNonOption<T>
    where T: Copy
{

    pub fn get(&self) -> T
    {

        self.value.unwrap() //.expect(NOT_INSTANTIATED_ERROR_MSG)

    }

}

impl<T> VarNonOption<T>
    where T: Default + Copy
{

    pub fn get_or_default(&self) -> T
    {

        self.value.unwrap_or_default()

    }

}

impl<T> Clone for VarNonOption<T>
    where T: Clone
{

    fn clone(&self) -> Self
    {

        Self
        {
            
            value: self.value.clone()

        }
        
    }

}

