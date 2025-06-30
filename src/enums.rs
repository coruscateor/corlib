use std::fmt::Display;




pub enum ThisOrThat<TThis, TThat>
{

    This(TThis),
    That(TThat)

}

impl<TThis, TThat> ThisOrThat<TThis, TThat>
{

    pub fn is_this(&self) -> bool
    {

        matches!(self, Self::This(_))

    }

    pub fn is_that(&self) -> bool
    {

        matches!(self, Self::That(_))

    }

}

impl<TThis, TThat> Display for ThisOrThat<TThis, TThat>
    where TThis: Display, TThat: Display
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {

        match self
        {

            ThisOrThat::This(val) =>
            {

                write!(f, "{{ This: {val} }}")

            }
            ThisOrThat::That(val) =>
            {

                write!(f, "{{ That: {val} }}")

            }

        }     
        
    }

}

pub enum ThisThatOther<TThis, TThat, TOther>
{

    This(TThis),
    That(TThat),
    Other(TOther)

}

impl<TThis, TThat, TOther> ThisThatOther<TThis, TThat, TOther>
{

    pub fn is_this(&self) -> bool
    {

        matches!(self, Self::This(_))

    }

    pub fn is_that(&self) -> bool
    {

        matches!(self, Self::That(_))

    }

    pub fn is_other(&self) -> bool
    {

        matches!(self, Self::Other(_))

    }

}

impl<TThis, TThat, TOther> Display for ThisThatOther<TThis, TThat, TOther>
    where TThis: Display, TThat: Display, TOther: Display
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {

        match self
        {

            ThisThatOther::This(val) =>
            {

                write!(f, "{{ This: {val} }}")

            }
            ThisThatOther::That(val) =>
            {

                write!(f, "{{ That: {val} }}")

            }
            ThisThatOther::Other(val) =>
            {

                write!(f, "{{ Other: {val} }}")

            }
            
        } 
        
    }

}

pub enum DefaultOrValue<T>
{

    Default,
    Value(T)

}

impl<T> DefaultOrValue<T>
{

    pub fn is_default(&self) -> bool
    {

        matches!(self, Self::Default)

    }

    pub fn is_value(&self) -> bool
    {

        matches!(self, Self::Value(_))

    }

    pub fn try_get_ref(&self) -> Option<&T>
    {

        if let DefaultOrValue::Value(val) = self
        {
            
            Some(val)

        }
        else
        {

            None
            
        }

    }

    pub fn try_get_mut(&mut self) -> Option<&mut T>
    {

        if let DefaultOrValue::Value(val) = self
        {
            
            Some(val)

        }
        else
        {

            None
            
        }

    }
    
}

impl<T> Default for DefaultOrValue<T>
{

    fn default() -> Self
    {

        Self::Default
        
    }

}

impl<T> Display for DefaultOrValue<T>
    where T: Display
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {

        match self
        {

            DefaultOrValue::Default =>
            {

                write!(f, "{{ Default }}")

            }
            DefaultOrValue::Value(val) =>
            {

                write!(f, "{{ Value: {val} }}")

            }

        }     
        
    }

}
