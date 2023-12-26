pub mod syscall_id;

use core::ffi::c_int;
use ruxos_posix_api::ctypes;
use syscall_id::SyscallId;

pub fn syscall(syscall_id: SyscallId, args: [usize; 6]) -> isize {
    debug!("x86 syscall <= syscall_name: {:?}", syscall_id);

    unsafe {
        match syscall_id {
            SyscallId::INVALID => ruxos_posix_api::sys_invalid(syscall_id as usize as c_int) as _,
        }
    }
}
