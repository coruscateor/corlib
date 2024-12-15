use delegate::delegate;

pub struct ResultFrame<T>
{

    result: T,
    done: bool

}

impl<T> ResultFrame<T>
{

    pub fn new(result: T, done: bool) -> Self
    {

        Self
        {

            result,
            done

        }

    }

    pub fn done(result: T) -> Self
    {

        Self::new(result, true)

    }

    pub fn not_done(result: T) -> Self
    {

        Self::new(result, false)

    }

    pub fn result(&self) -> &T
    {

        &self.result

    }

    pub fn is_done(&self) -> bool
    {

        self.done

    }

    pub fn take_result(self) -> T
    {

        self.result

    }

}

pub struct IdedResultFrame<ID, T>
{

    id: ID,
    result_frame: ResultFrame<T>

}

impl<ID, T> IdedResultFrame<ID, T>
{

    pub fn new(id: ID, result: T, done: bool) -> Self
    {

        Self
        {

            id,
            result_frame: ResultFrame::new(result, done)

        }

    }

    pub fn id(&self) -> &ID
    {

        &self.id

    }

    pub fn done(id: ID, result: T) -> Self
    {

        Self::new(id, result, true)

    }

    pub fn not_done(id: ID, result: T) -> Self
    {

        Self::new(id, result, false)

    }

    pub fn take_id_and_result(self) -> (ID, T)
    {

        (self.id, self.result_frame.result)

    }
    
    delegate!
    {

        to self.result_frame
        {

            pub fn result(&self) -> &T;

            pub fn is_done(&self) -> bool;

            pub fn take_result(self) -> T;

        }

    }

}


