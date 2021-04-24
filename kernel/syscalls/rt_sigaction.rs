use crate::arch::UserVAddr;
use crate::ctypes::*;
use crate::result::Result;
use crate::syscalls::SyscallHandler;

impl<'a> SyscallHandler<'a> {
    pub fn sys_rt_sigaction(
        &mut self,
        _signum: c_int,
        _act: Option<UserVAddr>,
        _oldact: Option<UserVAddr>,
    ) -> Result<isize> {
        // TODO:
        Ok(0)
    }
}
