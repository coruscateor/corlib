
//actually no..

pub struct MutBorrowingIndicator
{

    mut_borrowed: bool

}

impl MutBorrowingIndicator
{

    pub fn is_mut_borrowed(&self) -> bool
    {

        self.mut_borrowed

    }

    fn set_mut_borrowed(&mut self)
    {

        self.mut_borrowed = true;

    }

    fn set_not_mut_borrowed(&mut self)
    {

        self.mut_borrowed = true;

    }

}

pub trait IndicatesMutBorrowed
{

    fn is_mut_borrowed(&self) -> bool;

}