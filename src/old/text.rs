use std::{fmt::Display, ops::Deref, sync::Arc};

use crate::collections::Queue;

use delegate::delegate;

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

pub struct SendableTextLog
{

    st_queue: Queue<SendableText>,
    limit: usize

}

impl SendableTextLog
{

    pub fn new(limit: usize) -> Self
    {

        Self
        {

            st_queue: Queue::new(),
            limit

        }

    }
    
    pub fn with_capacity(capacity: usize) -> Self
    {

        Self
        {

            st_queue: Queue::with_capacity(capacity),
            limit: capacity

        }

    }

    pub fn with_capacit_and_limit(capacity: usize, limit: usize) -> Self
    {

        Self
        {

            st_queue: Queue::with_capacity(capacity),
            limit

        }

    }

    pub fn limit(&self) -> usize
    {

        self.limit

    }

    pub fn set_limit(&mut self, new_limit: usize)
    {

        self.limit = new_limit;

    }

    delegate! {
        to self.st_queue {

            pub fn capacity(&self) -> usize;

            pub fn len(&self) -> usize;

            pub fn is_empty(&self) -> bool;

            pub fn clear(&mut self); 

        }
    }

    pub fn push(&mut self, st: SendableText)
    {

        if self.st_queue.len() == self.limit
        {

            self.st_queue.pop();

        }

        self.st_queue.push(st);

    }

    //https://doc.rust-lang.org/std/collections/vec_deque/struct.Iter.html

    pub fn append_to(&self, output: &mut String)
    {

        for item in self.st_queue.iter().rev()
        {

            output.push_str(item);

        }

    }

    pub fn overwrite(&self, output: &mut String)
    {

        output.clear();

        self.append_to(output);

    }

}

static NO_STRING_ERROR_MESSAGE: &str = "Error: There has to be a String here.";

pub struct SendableTextLogWithBuffer
{

    stl: SendableTextLog,
    buffer: Option<String>

}

impl SendableTextLogWithBuffer
{

    pub fn new(limit: usize) -> Self
    {

        Self
        {

            stl: SendableTextLog::new(limit),
            buffer: Some(String::new())

        }

    }
    
    pub fn with_capacity(capacity: usize) -> Self
    {

        Self
        {

            stl: SendableTextLog::with_capacity(capacity),
            buffer: Some(String::new())

        }

    }

    pub fn with_capacit_and_limit(capacity: usize, limit: usize) -> Self
    {

        Self
        {

            stl: SendableTextLog::with_capacit_and_limit(capacity, limit),
            buffer: Some(String::new())

        }

    }

    //buffer

    pub fn with_buffer(limit: usize, buffer: String) -> Self
    {

        Self
        {

            stl: SendableTextLog::new(limit),
            buffer: Some(buffer)

        }

    }
    
    pub fn with_capacity_and_buffer(capacity: usize, buffer: String) -> Self
    {

        Self
        {

            stl: SendableTextLog::with_capacity(capacity),
            buffer: Some(buffer)

        }

    }

    pub fn with_capaciy_limit_and_buffer(capacity: usize, limit: usize, buffer: String) -> Self
    {

        Self
        {

            stl: SendableTextLog::with_capacit_and_limit(capacity, limit),
            buffer: Some(buffer)

        }

    }

    //

    delegate! {
        to self.stl {

            pub fn capacity(&self) -> usize;

            pub fn len(&self) -> usize;

            pub fn is_empty(&self) -> bool;

            pub fn limit(&self) -> usize;

            pub fn set_limit(&mut self, new_limit: usize);

            pub fn append_to(&self, output: &mut String);

            pub fn overwrite(&self, output: &mut String);

            #[call(push)]
            pub fn push_only(&mut self, st: SendableText);

        }
    }

    pub fn buffer(&self) -> &String
    {

        self.buffer.as_ref().expect(NO_STRING_ERROR_MESSAGE)

    }

    pub fn push(&mut self, st: SendableText)
    {

        self.stl.push(st);

        self.overwite_buffer();

    }

    pub fn clear(&mut self)
    {

        self.stl.clear();

        let mut_buffer = self.buffer.as_mut().expect(NO_STRING_ERROR_MESSAGE);
        
        mut_buffer.clear();

    }

    pub fn overwite_buffer(&mut self)
    {

        let mut res = self.buffer.take().expect(NO_STRING_ERROR_MESSAGE);

        self.overwrite(&mut res);

        self.buffer = Some(res);

    }

}