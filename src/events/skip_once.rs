//crate::impl_get!();

use paste::paste;

///
///Should the exection of the current event method be skipped?
/// 
pub struct SkipOnce
{

    should_skip: bool

}

impl SkipOnce
{

    pub fn new() -> Self
    {

        Self
        {

            should_skip: true

        }

    }

    crate::impl_get!(should_skip, bool);

    pub fn should_skip_next(&mut self)
    {

        self.should_skip = true;

    }

    pub fn get(&mut self) -> bool
    {

        if !self.should_skip
        {

            return false;

        }

        //let will_supress = self.will_supress;

        self.should_skip = false;

        //will_supress

        true

    }


}
