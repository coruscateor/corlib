use std::{sync::{Mutex, Condvar, LockResult, MutexGuard, WaitTimeoutResult, PoisonError}, time::Duration};

use delegate::delegate;

///
/// Returned from a Notifier::wait call
/// 
pub struct NotifierWaitResult<'a, T>
{

    lr: LockResult<MutexGuard<'a, T>>

}

impl<'a, T> NotifierWaitResult<'a, T>
{

    pub fn new(lr: LockResult<MutexGuard<'a, T>>) -> Self
    {

        Self
        {

            lr

        }

    }

    pub fn is_ok(&self) -> bool
    {

        if let Ok(_) = self.lr
        {

            return true;

        }

        false

    }

    pub fn get_mutex_guard_ref(&self) -> &MutexGuard<'a, T>
    {

        match &self.lr
        {
            Ok(val) =>
            {

                return val;

            },
            Err(err) => 
            {

                err.get_ref()

            }

        }

    }

    pub fn get_mutex_guard_mut(&mut self) -> &mut MutexGuard<'a, T>
    {

        match &mut self.lr
        {
            Ok(val) =>
            {

                return val;

            },
            Err(err) => 
            {

                err.get_mut()

            }

        }
        
    }

}

///
/// Returned from a Notifier::wait_timeout call
/// 
pub struct NotifierWaitTimeoutResult<'a, T>
{

    lr: LockResult<(MutexGuard<'a, T>, WaitTimeoutResult)>

}

impl<'a, T> NotifierWaitTimeoutResult<'a, T>
{

    pub fn new(lr: LockResult<(MutexGuard<'a, T>, WaitTimeoutResult)>) -> Self
    {

        Self
        {

            lr

        }

    }

    pub fn is_ok(&self) -> bool
    {

        if let Ok(_) = self.lr
        {

            return true;

        }

        false

    }

    pub fn has_timed_out(&self) -> bool
    {

        match &self.lr
        {

            Ok(val) =>
            {

                return val.1.timed_out();

            },
            Err(err) => 
            {

                err.get_ref().1.timed_out()

            }

        }

    }

    ///
    /// Checks both the returned MutexGuard and WaitTimeoutResult
    /// 
    /// The returd tuple has the following parameters:
    /// 
    /// 0: Is the MutexGuard Ok? 1: Has timed out?
    /// 
    pub fn get_status(&self) -> (bool, bool)
    {

        match &self.lr
        {

            Ok(val) =>
            {

                return (true, val.1.timed_out());

            },
            Err(err) => 
            {

                (false, err.get_ref().1.timed_out())

            }

        }

    }

    pub fn get_mg_wto_ref(&'a self) -> &(MutexGuard<'_, T>, WaitTimeoutResult)
    {

        match &self.lr
        {
            Ok(val) =>
            {

                return val;

            },
            Err(err) => 
            {

                err.get_ref()

            }

        }

    }

    pub fn get_mutex_guard_mut(&'a mut self) -> &mut MutexGuard<'a, T>
    {

        match &mut self.lr
        {
            Ok(val) =>
            {

                return &mut val.0;

            },
            Err(err) => 
            {

                &mut err.get_mut().0

            }

        }
        
    }

}

///
/// Block threads until notified (or they time-out).
/// 
/// Comprised of an std::sync::Mutex and Condvar.
/// 
#[derive(Default)]
pub struct Notifier<T = ()>
{

    mtx: Mutex<T>,
    cndvr: Condvar

}

/*
impl<T> Notifier<T>
    where T: Default
{

    pub fn new() -> Self
    {

        Self
        {

            mtx: Mutex::new(T::default()),
            cndvr: Condvar::new()

        }

    }

}
*/

impl<T> Notifier<T>
{

    pub fn new(value: T) -> Self
    {

        Self
        {

            mtx: Mutex::new(value),
            cndvr: Condvar::new()

        }

    }

    pub fn wait<'a>(&'a self) -> NotifierWaitResult<'a, T>
    {

        let mtx_lk_res = self.mtx.lock();

        let mtx_lk;

        match mtx_lk_res
        {

            Ok(res) =>
            {

                mtx_lk = res;

            },
            Err(_) => 
            {

                return NotifierWaitResult::<'a, T>::new(mtx_lk_res);

            }

        }

        NotifierWaitResult::new(self.cndvr.wait(mtx_lk))

    }

    pub fn wait_timeout<'a>(&'a self, dur: Duration) -> Result<NotifierWaitTimeoutResult<'a, T>, PoisonError<MutexGuard<'_, T>>>
    {

        let mtx_lk_res = self.mtx.lock();

        let mtx_lk;

        match mtx_lk_res
        {

            Ok(res) =>
            {

                mtx_lk = res;

            },
            Err(err) => 
            {

                return Err(err);

            }

        }

        Ok(NotifierWaitTimeoutResult::new(self.cndvr.wait_timeout(mtx_lk, dur)))

    }

    delegate! {
        to self.cndvr {

            pub fn notify_one(&self);

            pub fn notify_all(&self);

        }
    }

}


