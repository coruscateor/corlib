
/*
pub trait IntoString
{

    fn into_string(self) -> String;

}
*/

///Provides as_str as a trait method.
pub trait AsStr
{

    fn as_str(&self) -> &str;

}

/*
pub enum Text
{

    String(String),
    Str(&'static str)

}

impl Text
{

    pub fn is_string(&self) -> bool
    {

        if let Text::String(_) = self
        {

            return true;

        }

        false

    }

    pub fn is_str(&self) -> bool
    {

        if let Text::Str(_) = self
        {

            return true;

        }

        false

    }


}

impl IntoString for Text
{

    fn into_string(self) -> String
    {

        match self
        {

            Text::String(val) => val,
            Text::Str(val) => val.to_string()

        }

    }

}

impl AsStr for Text
{

    fn as_str(&self) -> &str
    {
       
        match self
        {

            Text::String(val) => val.as_str(),
            Text::Str(val) => val
            
        }


    }

}

impl ToString for Text
{

    fn to_string(&self) -> String
    {

        match self
        {

            Text::String(val) => val.to_string(),
            Text::Str(val) => val.to_string()
            
        }
        
    }

}
*/

//Unifes owned Strings with lifetime-labaled String references and String-slices. 
pub enum ScopedText<'a>
{

    String(String),
    StringRef(&'a String),
    StringMut(&'a mut String),
    Str(&'a str)

}

impl<'a> ScopedText<'a>
{

    pub fn is_string(&self) -> bool
    {

        if let ScopedText::String(_) = self
        {

            return true;

        }

        false

    }

    pub fn is_string_ref(&self) -> bool
    {

        if let ScopedText::StringRef(_) = self
        {

            return true;

        }

        false

    }

    pub fn is_string_mut(&self) -> bool
    {

        if let ScopedText::StringMut(_) = self
        {

            return true;

        }

        false

    }

    pub fn is_str(&self) -> bool
    {

        if let ScopedText::Str(_) = self
        {

            return true;

        }

        false

    }

}

/*
impl IntoString for ScopedText<'_>
{

    fn into_string(self) -> String
    {

        match self
        {

            ScopedText::String(val) => val,
            ScopedText::StringRef(val) => val.clone(),
            ScopedText::StringMut(val) => val.clone(),
            ScopedText::Str(val) => val.to_string(),

        }

    }

}
*/

impl Into<String> for ScopedText<'_>
{

    fn into(self) -> String
    {
        
        match self
        {

            ScopedText::String(val) => val,
            ScopedText::StringRef(val) => val.clone(),
            ScopedText::StringMut(val) => val.clone(),
            ScopedText::Str(val) => val.to_string(),

        }
        
    }

}

impl AsStr for ScopedText<'_>
{

    fn as_str(&self) -> &str
    {
       
        match self
        {

            ScopedText::String(val) => val.as_str(),
            ScopedText::StringRef(val) => val.as_str(),
            ScopedText::StringMut(val) => val.as_str(),
            ScopedText::Str(val) => val
            
        }


    }
    
}

impl ToString for ScopedText<'_>
{

    fn to_string(&self) -> String
    {

        match self
        {

            ScopedText::String(val) => val.clone(),
            ScopedText::StringRef(val) => (*val).clone(),
            ScopedText::StringMut(val) => (*val).clone(),
            ScopedText::Str(val) => val.to_string()
            
        }
        
    }

}

//Unifes owned Strings with static lifetime-labaled String references and String-slices.
pub type Text = ScopedText<'static>;


/*
fn new() -> Text
{

    Text::Str("1234")

}


fn new_a<'a>() -> ScopedText<'a>
{

    ScopedText::Str("1234")

}
*/

//Unifes owned Strings with static and non-static lifetime-labaled String references and String-slices.
pub enum ScopedStaticText<'a>
{

    String(String),
    StringRef(&'a String),
    StringMut(&'a mut String),
    Str(&'a str),
    StaticStringRef(&'static String),
    StaticStringMut(&'static mut String),
    StaticStr(&'static str)

}

impl<'a> ScopedStaticText<'a>
{

    pub fn is_string(&self) -> bool
    {

        if let ScopedStaticText::String(_) = self
        {

            return true;

        }

        false

    }

    pub fn is_string_ref(&self) -> bool
    {

        if let ScopedStaticText::StringRef(_) = self
        {

            return true;

        }

        false

    }

    pub fn is_string_mut(&self) -> bool
    {

        if let ScopedStaticText::StringMut(_) = self
        {

            return true;

        }

        false

    }

    pub fn is_str(&self) -> bool
    {

        if let ScopedStaticText::Str(_) = self
        {

            return true;

        }

        false

    }

    pub fn is_static_string_ref(&self) -> bool
    {

        if let ScopedStaticText::StaticStringRef(_) = self
        {

            return true;

        }

        false

    }

    pub fn is_static_string_mut(&self) -> bool
    {

        if let ScopedStaticText::StringMut(_) = self
        {

            return true;

        }

        false

    }

    pub fn is_static_str(&self) -> bool
    {

        if let ScopedStaticText::StaticStr(_) = self
        {

            return true;

        }

        false

    }

}

impl Into<String> for ScopedStaticText<'_>
{

    fn into(self) -> String
    {
        
        match self
        {

            ScopedStaticText::String(val) => val,
            ScopedStaticText::StringRef(val) => val.clone(),
            ScopedStaticText::StringMut(val) => val.clone(),
            ScopedStaticText::Str(val) => val.to_string(),
            ScopedStaticText::StaticStringRef(val) => val.clone(),
            ScopedStaticText::StaticStringMut(val) => val.clone(),
            ScopedStaticText::StaticStr(val) => val.to_string()

        }
        
    }

}

impl AsStr for ScopedStaticText<'_>
{

    fn as_str(&self) -> &str
    {
       
        match self
        {

            ScopedStaticText::String(val) => val.as_str(),
            ScopedStaticText::StringRef(val) => val.as_str(),
            ScopedStaticText::StringMut(val) => val.as_str(),
            ScopedStaticText::Str(val) => val,
            ScopedStaticText::StaticStringRef(val) => val.clone(),
            ScopedStaticText::StaticStringMut(val) => val.clone(),
            ScopedStaticText::StaticStr(val) => val
            
        }


    }
    
}

impl ToString for ScopedStaticText<'_>
{

    fn to_string(&self) -> String
    {

        match self
        {

            ScopedStaticText::String(val) => val.clone(),
            ScopedStaticText::StringRef(val) => (*val).clone(),
            ScopedStaticText::StringMut(val) => (*val).clone(),
            ScopedStaticText::Str(val) => val.to_string(),
            ScopedStaticText::StaticStringRef(val) => (*val).clone(),
            ScopedStaticText::StaticStringMut(val) => (*val).clone(),
            ScopedStaticText::StaticStr(val) => val.to_string()
            
        }
        
    }

}
