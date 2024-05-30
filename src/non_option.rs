use std::cell::RefCell;

static NOT_INSTANTIATED_ERROR_MSG: &str = "NonOption<T>: Not instantiated with a value";

/// An Option that is not really optional (but is still kinda).
/// Start by creating the object by calling invalid (probably as a field in a struct constructor) and setting a value by calling "set" with a parameter as soon as possible.
/// Attempting to use an invalid NonOption will cause a panic, so make absolutely sure you've set it with a value before calling "get_ref", "get_mut" etc.
/// 
/// NonOptions are useful when you have objects that refernce each other but cannot have these references set during initalisation (becuse one of the objects does not yet exist).
pub struct NonOption<T>
{

    value: Option<T>

}

impl<T> NonOption<T>
{

    ///
    /// Constructs a new NonOption object with a value.
    /// 
    pub fn new(value: T) -> Self
    {

        Self
        {

            value: Some(value)

        }

    }

    ///
    /// Constructs a new NonOption object with no value.
    /// 
    pub const fn invalid() -> Self
    {

        Self
        {

            value: None

        }

    }

    pub fn new_rfc(value: T) -> RefCell<Self>
    {

        RefCell::new(Self::new(value))

    }

    pub fn invalid_rfc() -> RefCell<Self>
    {

        RefCell::new(Self::invalid())

    }
    
    pub fn get_ref(&self) -> &T
    {

        self.value.as_ref().expect(NOT_INSTANTIATED_ERROR_MSG)

    }
    
    pub fn get_mut(&mut self) -> &mut T
    {

        self.value.as_mut().expect(NOT_INSTANTIATED_ERROR_MSG)

    }

    pub fn try_get_ref(&self) -> Option<&T>
    {

        self.value.as_ref()

    }

    pub fn try_get_mut(&mut self) -> Option<&mut T>
    {

        self.value.as_mut()

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

    pub fn expect(self) -> T
    {

        self.value.expect(NOT_INSTANTIATED_ERROR_MSG)

    }
    
    /*
    pub fn set(&mut self, value: T) -> bool
    {

        if self.value.is_none()
        {

            self.value = Some(value);

            return true;

        }

        false

    }
    */

    pub fn set(&mut self, value: T)
    {

        self.value = Some(value);

    }

    /*
    pub fn take(&mut self) -> T
    {

        let res = self.value.take();

        res.expect(NOT_INSTANTIATED_ERROR_MSG)

    }

    pub fn try_take(&mut self) -> Option<T>
    {

        self.value.take()

    }
    */
    
}

impl<T> Default for NonOption<T>
{

    fn default() -> Self
    {

        NonOption::invalid()

    }

}

impl<T> Clone for NonOption<T>
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

