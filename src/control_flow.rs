use std::{fmt::Display, ops::Deref};

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

///
/// Like the standard ControlFlow enum, without the type requirements.
/// 
#[derive(Clone, Debug, Hash, PartialEq, Eq, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ControlFlow
{

    Continue,
    Break

}

impl ControlFlow
{

    pub fn match_core<B, C>(cf: &core::ops::ControlFlow<B, C>) -> Self
    {

        match cf
        {

            std::ops::ControlFlow::Continue(_) => ControlFlow::Continue,
            std::ops::ControlFlow::Break(_) => ControlFlow::Break

        }

    }

    pub fn is_continue(&self) -> bool
    {

        matches!(self, ControlFlow::Continue)

    }

    pub fn is_break(&self) -> bool
    {

        matches!(self, ControlFlow::Break)

    }

    /*
    pub fn as_result<E>(&self) -> Result<bool, E>
    {

        match self
        {

            ControlFlow::Continue =>
            {

                Ok(true)

            }
            ControlFlow::Break =>
            {

                Ok(false)

            }

        }

    }
    */
    
}

impl From<bool> for ControlFlow
{

    ///
    /// true == Continue, false == Break
    /// 
    fn from(value: bool) -> Self
    {

        if value
        {

            Self::Continue

        }
        else
        {

            Self::Break
            
        }
        
    }

}

impl From<std::ops::ControlFlow<()>> for ControlFlow
{

    fn from(value: std::ops::ControlFlow<()>) -> Self
    {
        
        match value
        {

            std::ops::ControlFlow::Continue(_) => ControlFlow::Continue,
            std::ops::ControlFlow::Break(_) => ControlFlow::Break,

        }

    }

}

impl Display for ControlFlow
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        
        match self
        {

            ControlFlow::Continue =>
            {

                write!(f, "ControlFlow::Continue")

            }
            ControlFlow::Break =>
            {

                write!(f, "ControlFlow::Break")

            }

        }
        
    }

}

impl Deref for ControlFlow
{

    type Target = bool;

    fn deref(&self) -> &Self::Target
    {

        match self
        {

            ControlFlow::Continue => &true,
            ControlFlow::Break => &false

        }

    }

}

impl From<ControlFlow> for bool
{

    fn from(value: ControlFlow) -> Self
    {
        
        value.is_continue()
        
    }

}
