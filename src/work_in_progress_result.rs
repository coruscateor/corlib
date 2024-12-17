use delegate::delegate;

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


