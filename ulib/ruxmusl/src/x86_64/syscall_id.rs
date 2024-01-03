use num_enum::TryFromPrimitive;

// TODO: syscall id are architecture-dependent
#[allow(clippy::upper_case_acronyms)]
#[allow(non_camel_case_types)]
#[repr(usize)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, TryFromPrimitive)]
pub enum SyscallId {
    INVALID = 999,

    READ = 0,
    WRITE = 1,

    #[cfg(feature = "fs")]
    OPEN = 2,

    #[cfg(feature = "fd")]
    CLOSE = 3,

    #[cfg(feature = "fs")]
    STAT = 4,

    #[cfg(feature = "fs")]
    FSTAT = 5,

    #[cfg(feature = "fs")]
    LSTAT = 6,

    #[cfg(feature = "poll")]
    POLL = 7,

    #[cfg(feature = "fs")]
    LSEEK = 8,

    #[cfg(feature = "alloc")]
    MMAP = 9,

    #[cfg(feature = "alloc")]
    MPROTECT = 10,

    #[cfg(feature = "alloc")]
    MUNMAP = 11,

    #[cfg(feature = "signal")]
    RT_SIGACTION = 13,

    #[cfg(feature = "signal")]
    RT_SIGPROCMASK = 14,

    #[cfg(feature = "fd")]
    IOCTL = 16,

    #[cfg(feature = "fd")]
    READV = 19,

    #[cfg(feature = "fd")]
    WRITEV = 20,

    #[cfg(feature = "pipe")]
    PIPE = 22,

    #[cfg(feature = "select")]
    SELECT = 23,

    SCHED_YIELD = 24,

    #[cfg(feature = "alloc")]
    MREMAP = 25,

    EXIT = 60,

    ARCH_PRCTL = 158,

    #[cfg(feature = "multitask")]
    SET_TID_ADDRESS = 218,
}
