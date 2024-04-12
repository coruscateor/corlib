use std::{ops::{AddAssign, Add, Index, SubAssign}, result};

use crate::has_one::*;

//ptr::Pointee

use crate::collections::List; //list::List;

///
/// Increments a number returning "gap" mumbers, numbers that are reported to be no longer "be in use", first.
/// 
pub struct GapFillingCounter<T, HO>
    where T: PartialEq, HO: HasOne<T>
{

    value: T,
    gaps: List<T>, //Replace with a sorted set
    has_one: HO

} 

impl<T, HO> GapFillingCounter<T, HO>
    where T: PartialEq + Default + AddAssign + SubAssign + Copy,
          HO: HasOne<T>
{

    pub fn new_has_one(has_one: HO) -> Self
    {

        Self
        {

            value: Default::default(),
            gaps: List::new(),
            has_one

        }

    }

    pub fn start_at_has_one(value: T, has_one: HO) -> Self
    {

        Self
        {

            value,
            gaps: List::new(),
            has_one

        }

    }

    /*
    pub fn next(&mut self) -> T
    {

        self.value += 1 as T;



    }
    */

    /*
    pub fn gap_value(&mut self, val: T) -> bool
    {

        if val == self.value
        {



        }

    }
    */

    /*
    fn try_remove(&mut self, val: T) -> bool
    {

        /*
        if self.gaps.contains(&val)
        {

            self.gaps.remove(val);

        }
        */

        if let Some(index) = self.gaps.index_of(&val)
        {

            self.gaps.remove_at(index);

            return true;

        }

        false

    }
    */

    //
    //Get the next value, either incremented ot from a "gap"
    //
    pub fn next(&mut self) -> T
    {

        if let Some(val) = self.gaps.pop()
        {

            return val;

        }

        self.value += HO::one();

        self.value

    }

    ///
    /// "return" a value
    ///
    pub fn gap_value(&mut self, val: T) -> bool
    {

        //let mut decremented = false;

        //let mut gapped = false;

        if val == self.value
        {

            self.value -= HO::one();

            //check that the value wasn't already in the gaps list

            if let Some(index) = self.gaps.index_of(&val)
            {

                self.gaps.remove_at(index);

            }

            //decremented = true;

            return true;

        }
        else if !self.gaps.contains(&val)
        {

            self.gaps.add(val);

            return true;
            
        }

        false

        //gapped

        //let removed = self.try_remove(val);

        //decremented || removed

    }

    //Remove a value from the gap store if nessary

    pub fn un_gap_value(&mut self, val: T) -> bool
    {

        self.gaps.remove(val)

        /*
        let result = self.gaps.remove(val);

        if let Ok(_) = result
        {

            return true;

        }

        false
        */

    }

    pub fn gaps_len(&self) -> usize
    {

        self.gaps.len()

    }

    //add types

    pub fn new_i8() -> GapFillingCounter<i8, I8HasOne>
    {

        GapFillingCounter::<i8, I8HasOne>::new()

    }

    pub fn new_i16() -> GapFillingCounter<i16, I16HasOne>
    {

        GapFillingCounter::<i16, I16HasOne>::new()

    }

    pub fn new_i32() -> GapFillingCounter<i32, I32HasOne>
    {

        GapFillingCounter::<i32, I32HasOne>::new()

    }

    pub fn new_i64() -> GapFillingCounter<i64, I64HasOne>
    {

        GapFillingCounter::<i64, I64HasOne>::new()

    }

    pub fn new_i128() -> GapFillingCounter<i128, I128HasOne>
    {

        GapFillingCounter::<i128, I128HasOne>::new()

    }

    pub fn new_isize() -> GapFillingCounter<isize, ISizeHasOne>
    {

        GapFillingCounter::<isize, ISizeHasOne>::new()

    }

    pub fn new_u8() -> GapFillingCounter<u8, U8HasOne>
    {

        GapFillingCounter::<u8, U8HasOne>::new()

    }

    pub fn new_u16() -> GapFillingCounter<u16, U16HasOne>
    {

        GapFillingCounter::<u16, U16HasOne>::new()

    }

    pub fn new_u32() -> GapFillingCounter<u32, U32HasOne>
    {

        GapFillingCounter::<u32, U32HasOne>::new()

    }

    pub fn new_u64() -> GapFillingCounter<u64, U64HasOne>
    {

        GapFillingCounter::<u64, U64HasOne>::new()

    }

    pub fn new_u128() -> GapFillingCounter<u128, U128HasOne>
    {

        GapFillingCounter::<u128, U128HasOne>::new()

    }

    pub fn new_usize() -> GapFillingCounter<usize, USizeHasOne>
    {

        GapFillingCounter::<usize, USizeHasOne>::new()

    }

}

impl GapFillingCounter<i8, I8HasOne>
{

    pub fn new() -> Self
    {

        Self
        {

            value: Default::default(),
            gaps: List::new(),
            has_one: I8HasOne()

        }

    }

    /*
    pub fn start_at_1() -> Self
    {

        Self
        {

            value: 1, // as i8,
            gaps: List::new(),
            has_one: i8HasOne()

        }

    }
    */

}

impl GapFillingCounter<i16, I16HasOne>
{

    pub fn new() -> Self
    {

        Self
        {

            value: Default::default(),
            gaps: List::new(),
            has_one: I16HasOne()

        }

    }

}

impl GapFillingCounter<i32, I32HasOne>
{

    pub fn new() -> Self
    {

        Self
        {

            value: Default::default(),
            gaps: List::new(),
            has_one: I32HasOne()

        }

    }

}

impl GapFillingCounter<i64, I64HasOne>
{

    pub fn new() -> Self
    {

        Self
        {

            value: Default::default(),
            gaps: List::new(),
            has_one: I64HasOne()

        }

    }

}

impl GapFillingCounter<i128, I128HasOne>
{

    pub fn new() -> Self
    {

        Self
        {

            value: Default::default(),
            gaps: List::new(),
            has_one: I128HasOne()

        }

    }

}

impl GapFillingCounter<isize, ISizeHasOne>
{

    pub fn new() -> Self
    {

        Self
        {

            value: Default::default(),
            gaps: List::new(),
            has_one: ISizeHasOne()

        }

    }

}

impl GapFillingCounter<u8, U8HasOne>
{

    pub fn new() -> Self
    {

        Self
        {

            value: Default::default(),
            gaps: List::new(),
            has_one: U8HasOne()

        }

    }

}

impl GapFillingCounter<u16, U16HasOne>
{

    pub fn new() -> Self
    {

        Self
        {

            value: Default::default(),
            gaps: List::new(),
            has_one: U16HasOne()

        }

    }

}

impl GapFillingCounter<u32, U32HasOne>
{

    pub fn new() -> Self
    {

        Self
        {

            value: Default::default(),
            gaps: List::new(),
            has_one: U32HasOne()

        }

    }

}

impl GapFillingCounter<u64, U64HasOne>
{

    pub fn new() -> Self
    {

        Self
        {

            value: Default::default(),
            gaps: List::new(),
            has_one: U64HasOne()

        }

    }

}

impl GapFillingCounter<u128, U128HasOne>
{

    pub fn new() -> Self
    {

        Self
        {

            value: Default::default(),
            gaps: List::new(),
            has_one: U128HasOne()

        }

    }

}

impl GapFillingCounter<usize, USizeHasOne>
{

    pub fn new() -> Self
    {

        Self
        {

            value: Default::default(),
            gaps: List::new(),
            has_one: USizeHasOne()

        }

    }

}



/* 
impl<T> Add for GapFillingCounter<T>
    where T: PartialEq
{

    type Output = T;

    fn add(self, rhs: Self) -> Self::Output
    {

       

    }

}

impl<T> AddAssign for GapFillingCounter<T>
    where T: PartialEq
{

    fn add_assign(&mut self, rhs: Self) {

        *self = self.value + rhs.value;
        
    }

}
*/

