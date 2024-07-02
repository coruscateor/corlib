
///
///Provides as_str as a trait method.
/// 
pub trait AsStr
{

    fn as_str(&self) -> &str;

}

///
///MovableText: Ideal for when you want to be able to move text around that could either be a String ot a static String slice.
/// 
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MovableText
{

    String(String),
    Str(&'static str)

}

impl MovableText
{

    pub fn is_string(&self) -> bool
    {

        if let MovableText::String(_) = self
        {

            return true;

        }

        false

    }

    pub fn is_str(&self) -> bool
    {

        if let MovableText::Str(_) = self
        {

            return true;

        }

        false

    }

}

impl Into<String> for MovableText
{

    fn into(self) -> String
    {
        
        match self
        {

            MovableText::String(val) => val,
            MovableText::Str(val) => val.to_string()

        }
        
    }

}

impl AsStr for MovableText
{

    fn as_str(&self) -> &str
    {
       
        match self
        {

            MovableText::String(val) => val.as_str(),
            MovableText::Str(val) => val
            
        }


    }
    
}

impl ToString for MovableText
{

    fn to_string(&self) -> String
    {

        match self
        {

            MovableText::String(val) => val.clone(),
            MovableText::Str(val) => val.to_string()
            
        }
        
    }

}

