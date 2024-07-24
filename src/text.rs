use std::{result, sync::Arc};

///
///Provides as_str as a trait method.
/// 
pub trait AsStr
{

    fn as_str(&self) -> &str;

}

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

            SendableText::String(val) => val.as_str(),
            SendableText::Str(val) => val,
            SendableText::ArcStr(val) => &val
            
        }


    }
    
}

impl ToString for SendableText
{

    fn to_string(&self) -> String
    {

        match self
        {

            SendableText::String(val) => val.clone(),
            SendableText::Str(val) => val.to_string(),
            SendableText::ArcStr(val) => val.to_string()
            
        }

    }

}

