use std::{fmt::{write, Display}};

use delegate::delegate;

use std::fmt::Debug;

///
/// For indicating when some work has been done or is in the process of being done, potentially being accompanied by a result value of some kind.
/// 
pub struct WorkInProgressResult<T>
{

    result: Option<T>,
    done: bool

}

impl<T> WorkInProgressResult<T>
{

    pub fn new(result: Option<T>, done: bool) -> Self
    {

        Self
        {

            result,
            done

        }

    }

    pub fn done(result: T) -> Self
    {

        Self::new(Some(result), true)

    }

    pub fn opt_done(result: Option<T>) -> Self
    {

        Self::new(result, true)

    }

    pub fn not_done(result: T) -> Self
    {

        Self::new(Some(result), false)

    }

    pub fn opt_not_done(result: Option<T>) -> Self
    {

        Self::new(result, false)

    }

    pub fn done_none() -> Self
    {

        Self::new(None, true)

    }

    pub fn not_done_none() -> Self
    {

        Self::new(None, false)

    }

    pub fn result(&self) -> &Option<T>
    {

        &self.result

    }

    pub fn is_done(&self) -> bool
    {

        self.done

    }

    pub fn take_result(self) -> Option<T>
    {

        self.result

    }

}

impl<T> Display for WorkInProgressResult<T>
    where T: Display
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {

        match &self.result
        {

            Some(val) =>
            {

                write!(f, "{{ result: {0}, done: {1} }}", val, self.done)

            }
            None =>
            {

                write!(f, "{{ done: {0} }}", self.done)

            }

        } 
        
    }

}

impl<T> Debug for WorkInProgressResult<T>
    where T: Debug
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WorkInProgressResult").field("result", &self.result).field("done", &self.done).finish()
    }

}


///
/// The same as WorkInProgressResult, but with an id.
/// 
pub struct IdedWorkInProgressResult<ID, T>
{

    id: ID,
    work_in_progress_result: WorkInProgressResult<T>

}

impl<ID, T> IdedWorkInProgressResult<ID, T>
{

    pub fn new(id: ID, result: Option<T>, done: bool) -> Self
    {

        Self
        {

            id,
            work_in_progress_result: WorkInProgressResult::new(result, done)

        }

    }

    pub fn id(&self) -> &ID
    {

        &self.id

    }

    pub fn done(id: ID, result: T) -> Self
    {

        Self::new(id, Some(result), true)

    }

    pub fn opt_done(id: ID, result: Option<T>) -> Self
    {

        Self::new(id, result, true)

    }

    pub fn not_done(id: ID, result: T) -> Self
    {

        Self::new(id, Some(result), false)

    }

    pub fn opt_not_done(id: ID, result: Option<T>) -> Self
    {

        Self::new(id, result, false)

    }

    pub fn done_none(id: ID) -> Self
    {

        Self::new(id, None, true)

    }

    pub fn not_done_none(id: ID) -> Self
    {

        Self::new(id, None, false)

    }

    pub fn take_id_and_result(self) -> (ID, Option<T>)
    {

        (self.id, self.work_in_progress_result.result)

    }
    
    delegate!
    {

        to self.work_in_progress_result
        {

            pub fn result(&self) -> &Option<T>;

            pub fn is_done(&self) -> bool;

            pub fn take_result(self) -> Option<T>;

        }

    }

}

impl<ID, T> Display for IdedWorkInProgressResult<ID, T>
    where ID: Display,
          T: Display
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {

        match &self.work_in_progress_result.result
        {

            Some(val) =>
            {

                write!(f, "{{ Id: {0}, result: {1}, done: {2} }}", self.id, val, self.work_in_progress_result.done)

            }
            None =>
            {

                write!(f, "{{ Id: {0}, done: {1} }}", self.id, self.work_in_progress_result.done)

            }

        } 
        
    }

}

impl<ID, T> Debug for IdedWorkInProgressResult<ID, T>
    where ID: Debug,
          T: Debug
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IdedWorkInProgressResult").field("id", &self.id).field("work_in_progress_result", &self.work_in_progress_result).finish()
    }

}


