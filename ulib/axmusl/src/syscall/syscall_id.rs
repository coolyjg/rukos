use num_enum::TryFromPrimitive;

// TODO: syscall id are architecture-dependent
#[allow(clippy::upper_case_acronyms)]
#[allow(non_camel_case_types)]
#[repr(usize)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, TryFromPrimitive)]
pub enum SyscallId {
    INVALID = 999,
    #[cfg(feature = "fd")]
    DUP3 = 24,
    #[cfg(feature = "fd")]
    IOCTL = 29,
    #[cfg(feature = "fs")]
    OPENAT = 56,
    #[cfg(feature = "fs")]
    MKDIRAT = 34,
    #[cfg(feature = "fs")]
    UNLINKAT = 35,
    READ = 63,
    WRITE = 64,
    #[cfg(feature = "fd")]
    CLOSE = 57,
    #[cfg(feature = "fs")]
    FSTAT = 80,
    #[cfg(feature = "multitask")]
    SET_TID_ADDRESS = 96,
    #[cfg(feature = "fs")]
    LSEEK = 62,
    #[cfg(feature = "fd")]
    WRITEV = 66,
    #[cfg(feature = "pipe")]
    PIPE2 = 59,
    SCHED_YIELD = 124,
    #[cfg(feature = "fd")]
    DUP = 23,
    NANO_SLEEP = 101,
    #[cfg(feature = "multitask")]
    GETPID = 172,
    #[cfg(feature = "net")]
    SOCKET = 198,
    #[cfg(feature = "net")]
    CONNECT = 203,
    #[cfg(feature = "net")]
    ACCEPT = 202,
    #[cfg(feature = "net")]
    SENDTO = 206,
    #[cfg(feature = "net")]
    RECVFROM = 207,
    #[cfg(feature = "net")]
    SETSOCKOPT = 208,
    #[cfg(feature = "net")]
    SHUTDOWN = 210,
    #[cfg(feature = "net")]
    BIND = 200,
    #[cfg(feature = "net")]
    LISTEN = 201,
    #[cfg(feature = "net")]
    GETSOCKNAME = 204,
    #[cfg(feature = "net")]
    GETPEERNAME = 205,
    EXIT = 93,
    #[cfg(feature = "fd")]
    FCNTL = 25,
    #[cfg(feature = "fs")]
    GETCWD = 17,
    #[cfg(feature = "poll")]
    PPOLL = 73,
    CLOCK_GETTIME = 113,
    #[cfg(feature = "epoll")]
    EPOLL_CREATE1 = 20,
    #[cfg(feature = "epoll")]
    EPOLL_CTL = 21,
    #[cfg(feature = "epoll")]
    EPOLL_PWAIT = 22,
    #[cfg(feature = "multitask")]
    FUTEX = 98,
    // #[cfg(feature = "signal")]
    RT_SIGPROCMASK = 135,
    #[cfg(feature = "alloc")]
    MUNMAP = 215,
    #[cfg(feature = "multitask")]
    CLONE = 220,
    #[cfg(feature = "alloc")]
    MMAP = 222,
    #[cfg(feature = "alloc")]
    MPROTECT = 226,
    UMASK = 166,
    // #[cfg(feature = "signal")]
    RT_SIGACTION = 134,
    SYSINFO = 179,
    PRLIMIT64 = 261,
    GETRLIMIT = 163,
    SETRLIMIT = 164,
    // #[cfg(feature = "fd")]
    // DUP3 = 292,
}
