pub mod syscall_id;

use core::ffi::{c_int, c_ulong, c_void};
use ruxos_posix_api::ctypes;
use syscall_id::SyscallId;

pub fn syscall(syscall_id: SyscallId, args: [usize; 6]) -> isize {
    debug!("x86 syscall <= syscall_name: {:?}", syscall_id);

    unsafe {
        match syscall_id {
            SyscallId::INVALID => ruxos_posix_api::sys_invalid(syscall_id as usize as c_int) as _,

            SyscallId::READ => {
                ruxos_posix_api::sys_read(args[0] as c_int, args[1] as *mut c_void, args[2]) as _
            }

            SyscallId::WRITE => {
                ruxos_posix_api::sys_write(args[0] as c_int, args[1] as *const c_void, args[2]) as _
            }

            #[cfg(feature = "fs")]
            SyscallId::OPEN => ruxos_posix_api::sys_open(
                args[0] as *const core::ffi::c_char,
                args[1] as c_int,
                args[2] as ctypes::mode_t,
            ) as _,

            #[cfg(feature = "fd")]
            SyscallId::CLOSE => ruxos_posix_api::sys_close(args[0] as c_int) as _,

            #[cfg(feature = "fs")]
            SyscallId::STAT => ruxos_posix_api::sys_stat(
                args[0] as *const core::ffi::c_char,
                args[1] as *mut c_void,
            ) as _,

            #[cfg(feature = "fs")]
            SyscallId::FSTAT => {
                ruxos_posix_api::sys_fstat(args[0] as c_int, args[1] as *mut c_void) as _
            }

            // TODO: lstat is identical to stat(), except that if pathname is a symbolic link,
            // then it returns information about the link itself, not the file that the link refers to.
            #[cfg(feature = "fs")]
            SyscallId::LSTAT => ruxos_posix_api::sys_stat(
                args[0] as *const core::ffi::c_char,
                args[1] as *mut c_void,
            ) as _,

            #[cfg(feature = "poll")]
            SyscallId::POLL => ruxos_posix_api::sys_poll(
                args[0] as *mut ctypes::pollfd,
                args[1] as ctypes::nfds_t,
                args[2] as c_int,
            ) as _,

            #[cfg(feature = "fs")]
            SyscallId::LSEEK => ruxos_posix_api::sys_lseek(
                args[0] as c_int,
                args[1] as ctypes::off_t,
                args[2] as c_int,
            ) as _,

            #[cfg(feature = "alloc")]
            SyscallId::MMAP => ruxos_posix_api::sys_mmap(
                args[0] as *mut c_void,
                args[1] as ctypes::size_t,
                args[2] as c_int,
                args[3] as c_int,
                args[4] as c_int,
                args[5] as ctypes::off_t,
            ) as _,

            #[cfg(feature = "alloc")]
            SyscallId::MPROTECT => ruxos_posix_api::sys_mprotect(
                args[0] as *mut c_void,
                args[1] as ctypes::size_t,
                args[2] as c_int,
            ) as _,

            #[cfg(feature = "alloc")]
            SyscallId::MUNMAP => {
                ruxos_posix_api::sys_munmap(args[0] as *mut c_void, args[1] as ctypes::size_t) as _
            }

            #[cfg(feature = "signal")]
            SyscallId::RT_SIGACTION => ruxos_posix_api::sys_rt_sigaction(
                args[0] as c_int,
                args[1] as *const ctypes::sigaction,
                args[2] as *mut ctypes::sigaction,
                args[3] as ctypes::size_t,
            ) as _,

            #[cfg(feature = "signal")]
            SyscallId::RT_SIGPROCMASK => ruxos_posix_api::sys_rt_sigprocmask(
                args[0] as c_int,
                args[1] as *const usize,
                args[2] as *mut usize,
                args[3],
            ) as _,

            #[cfg(feature = "fd")]
            SyscallId::IOCTL => ruxos_posix_api::sys_ioctl(args[0] as c_int, args[1], args[2]) as _,

            #[cfg(feature = "fd")]
            SyscallId::READV => ruxos_posix_api::sys_readv(
                args[0] as c_int,
                args[1] as *const ctypes::iovec,
                args[2] as c_int,
            ) as _,

            #[cfg(feature = "fd")]
            SyscallId::WRITEV => ruxos_posix_api::sys_writev(
                args[0] as c_int,
                args[1] as *const ctypes::iovec,
                args[2] as c_int,
            ) as _,

            #[cfg(feature = "pipe")]
            SyscallId::PIPE => {
                ruxos_posix_api::sys_pipe(core::slice::from_raw_parts_mut(args[0] as *mut c_int, 2))
                    as _
            }

            #[cfg(feature = "select")]
            SyscallId::SELECT => ruxos_posix_api::sys_select(
                args[0] as c_int,
                args[1] as *mut ctypes::fd_set,
                args[2] as *mut ctypes::fd_set,
                args[3] as *mut ctypes::fd_set,
                args[4] as *mut ctypes::timeval,
            ) as _,

            SyscallId::SCHED_YIELD => ruxos_posix_api::sys_sched_yield() as _,

            #[cfg(feature = "alloc")]
            SyscallId::MREMAP => ruxos_posix_api::sys_mremap(
                args[0] as *mut core::ffi::c_void,
                args[1] as ctypes::size_t,
                args[2] as ctypes::size_t,
                args[3] as c_int,
                args[4] as *mut core::ffi::c_void,
            ) as _,

            #[cfg(not(feature = "multitask"))]
            SyscallId::EXIT => ruxos_posix_api::sys_exit(args[0] as c_int) as _,

            #[allow(unreachable_code)]
            #[cfg(feature = "multitask")]
            SyscallId::EXIT => ruxos_posix_api::sys_pthread_exit(args[0] as *mut c_void) as _,

            SyscallId::ARCH_PRCTL => {
                ruxos_posix_api::sys_arch_prctl(args[0] as c_int, args[1] as c_ulong) as _
            }

            #[cfg(feature = "multitask")]
            SyscallId::SET_TID_ADDRESS => ruxos_posix_api::sys_set_tid_address(args[0]) as _,
        }
    }
}
