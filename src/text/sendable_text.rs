use std::{fmt::Display, ops::Deref, sync::Arc};

use super::AsStr;

#[cfg(feature = "serde")]
use serde::{Serialize, Serializer};

///
///SendableText: Ideal for when you want to be able to move text around that could either be a String ot a static String slice.
/// 
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SendableText
{

    String(String),
    Str(&'static str),
    ArcStr(Arc<str>)

}

impl SendableText
{

    pub fn is_string(&self) -> bool
    {

        if let SendableText::String(_) = self
        {

            return true;

        }

        false

    }

    pub fn is_str(&self) -> bool
    {

        if let SendableText::Str(_) = self
        {

            return true;

        }

        false

    }

    pub fn is_arc_str(&self) -> bool
    {

        if let SendableText::ArcStr(_) = self
        {

            return true;

        }

        false

    }

    pub fn extract_string(self) -> Result<String, Self>
    {

        match self
        {

            SendableText::String(val) => Ok(val),
            SendableText::Str(val) => Err(Self::Str(val)),
            SendableText::ArcStr(val) => Err(Self::ArcStr(val))

        }

    }

}

impl Into<String> for SendableText
{

    fn into(self) -> String
    {
        
        match self
        {

            SendableText::String(val) => val,
            SendableText::Str(val) => val.to_string(),
            SendableText::ArcStr(val) => val.to_string()

        }
        
    }

}

impl AsStr for SendableText
{

    fn as_str(&self) -> &str
    {
       
        match self
        {

            SendableText::String(val) => val,
            SendableText::Str(val) => val,
            SendableText::ArcStr(val) => &val
            
        }


    }
    
}

impl Display for SendableText
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {

        match self
        {

            SendableText::String(val) => write!(f, "{}", val),
            SendableText::Str(val) => write!(f, "{}", val),
            SendableText::ArcStr(val) => write!(f, "{}", val)

        }
        
    }

}

impl Deref for SendableText
{

    type Target = str;

    fn deref(&self) -> &Self::Target
    {

        self.as_str()
        
    }

}

impl From<String> for SendableText
{

    fn from(value: String) -> Self
    {

        SendableText::String(value)

    }

}

impl From<&String> for SendableText
{

    fn from(value: &String) -> Self
    {

        SendableText::String(value.clone())
        
    }

}

impl From<&'static str> for SendableText
{

    fn from(value: &'static str) -> Self
    {

        SendableText::Str(value)
        
    }

}

impl From<Arc<str>> for SendableText
{

    fn from(value: Arc<str>) -> Self
    {

        SendableText::ArcStr(value)
        
    }

}

impl From<&Arc<str>> for SendableText
{

    fn from(value: &Arc<str>) -> Self
    {

        SendableText::ArcStr(value.clone())
        
    }

}

#[cfg(feature = "serde")]
impl Serialize for SendableText
{

    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer
    {

        serializer.serialize_str(self.as_str())

    }

}