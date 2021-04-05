pub type c_char = u8;
pub type wchar_t = u32;
pub type c_long = i32;
pub type c_ulong = u32;

// sys/types.h
pub type blkcnt_t = ::c_long;
pub type blksize_t = ::c_long;

pub type clock_t = ::c_ulong;
pub type time_t = i64;

pub type fsblkcnt_t = u64;
pub type fsfilcnt_t = u32;

pub type id_t = u32;
pub type ino_t = ::c_ulong;

pub type off_t = ::c_longlong;
pub type dev_t = ::c_short;
pub type key_t = ::c_long;
pub type mode_t = ::c_ushort;
pub type nlink_t = ::c_ushort;

pub type clockid_t = ::c_ulong;
pub type timer_t = ::c_ulong;

pub type useconds_t = ::c_ulong;
pub type suseconds_t = ::c_long;

// sys/termios.h
pub type tcflag_t = ::c_uint;
pub type speed_t = ::c_uint;

pub const NCCS: usize = 13;

s! {
    pub struct termios {
        pub c_iflag: ::tcflag_t,
        pub c_oflag: ::tcflag_t,
        pub c_cflag: ::tcflag_t,
        pub c_lflag: ::tcflag_t,
        pub c_cc: [::cc_t; ::NCCS],
    }
}

// sys/stat.h
pub const S_IFMT: ::mode_t = 0170000;
pub const S_IFDIR: ::mode_t = 0040000;
pub const S_IFCHR: ::mode_t = 0020000;
pub const S_IFBLK: ::mode_t = 0060000;
pub const S_IFREG: ::mode_t = 0100000;
pub const S_IFLNK: ::mode_t = 0120000;
pub const S_IFSOCK: ::mode_t = 0140000;
pub const S_IFIFO: ::mode_t = 0010000;

s! {
    pub struct stat {
        pub st_dev: ::dev_t,
        pub st_ino: ::ino_t,
        pub st_mode: ::mode_t,
        pub st_nlink: ::nlink_t,
        pub st_uid: ::uid_t,
        pub st_gid: ::gid_t,
        pub st_rdev: dev_t,
        pub st_size: off_t,
        pub st_atime: time_t,
        pub st_spare1: ::c_long,
        pub st_mtime: time_t,
        pub st_spare2: ::c_long,
        pub st_ctime: time_t,
        pub st_spare3: ::c_long,
        pub st_blksize: blksize_t,
        pub st_blocks: blkcnt_t,
        pub st_spare4: [::c_long; 2usize],
    }
}

// sys/dirent.h
s! {
    pub struct dirent {
        pub d_ino: ::c_ulong,
        pub d_off: ::off_t,
        pub d_reclen: ::c_ushort,
        pub d_type: ::c_char,
        pub d_name: [::c_char; 256usize],
    }
}

// sys/unistd.h
pub const STDIN_FILENO: ::c_int = 0;
pub const STDOUT_FILENO: ::c_int = 1;
pub const STDERR_FILENO: ::c_int = 2;

pub const SEEK_SET: ::c_int = 0;
pub const SEEK_CUR: ::c_int = 1;
pub const SEEK_END: ::c_int = 2;

pub const _SC_ARG_MAX: ::c_int = 0;
pub const _SC_CHILD_MAX: ::c_int = 1;
pub const _SC_CLK_TCK: ::c_int = 2;
pub const _SC_NGROUPS_MAX: ::c_int = 3;
pub const _SC_OPEN_MAX: ::c_int = 4;
pub const _SC_JOB_CONTROL: ::c_int = 5;
pub const _SC_SAVED_IDS: ::c_int = 6;
pub const _SC_VERSION: ::c_int = 7;
pub const _SC_PAGESIZE: ::c_int = 8;
pub const _SC_PAGE_SIZE: ::c_int = _SC_PAGESIZE;
pub const _SC_NPROCESSORS_CONF: ::c_int = 9;
pub const _SC_NPROCESSORS_ONLN: ::c_int = 10;
pub const _SC_PHYS_PAGES: ::c_int = 11;
pub const _SC_AVPHYS_PAGES: ::c_int = 12;
pub const _SC_MQ_OPEN_MAX: ::c_int = 13;
pub const _SC_MQ_PRIO_MAX: ::c_int = 14;
pub const _SC_RTSIG_MAX: ::c_int = 15;
pub const _SC_SEM_NSEMS_MAX: ::c_int = 16;
pub const _SC_SEM_VALUE_MAX: ::c_int = 17;
pub const _SC_SIGQUEUE_MAX: ::c_int = 18;
pub const _SC_TIMER_MAX: ::c_int = 19;
pub const _SC_TZNAME_MAX: ::c_int = 20;
pub const _SC_ASYNCHRONOUS_IO: ::c_int = 21;
pub const _SC_FSYNC: ::c_int = 22;
pub const _SC_MAPPED_FILES: ::c_int = 23;
pub const _SC_MEMLOCK: ::c_int = 24;
pub const _SC_MEMLOCK_RANGE: ::c_int = 25;
pub const _SC_MEMORY_PROTECTION: ::c_int = 26;
pub const _SC_MESSAGE_PASSING: ::c_int = 27;
pub const _SC_PRIORITIZED_IO: ::c_int = 28;
pub const _SC_REALTIME_SIGNALS: ::c_int = 29;
pub const _SC_SEMAPHORES: ::c_int = 30;
pub const _SC_SHARED_MEMORY_OBJECTS: ::c_int = 31;
pub const _SC_SYNCHRONIZED_IO: ::c_int = 32;
pub const _SC_TIMERS: ::c_int = 33;
pub const _SC_AIO_LISTIO_MAX: ::c_int = 34;
pub const _SC_AIO_MAX: ::c_int = 35;
pub const _SC_AIO_PRIO_DELTA_MAX: ::c_int = 36;
pub const _SC_DELAYTIMER_MAX: ::c_int = 37;
pub const _SC_THREAD_KEYS_MAX: ::c_int = 38;
pub const _SC_THREAD_STACK_MIN: ::c_int = 39;
pub const _SC_THREAD_THREADS_MAX: ::c_int = 40;
pub const _SC_TTY_NAME_MAX: ::c_int = 41;
pub const _SC_THREADS: ::c_int = 42;
pub const _SC_THREAD_ATTR_STACKADDR: ::c_int = 43;
pub const _SC_THREAD_ATTR_STACKSIZE: ::c_int = 44;
pub const _SC_THREAD_PRIORITY_SCHEDULING: ::c_int = 45;
pub const _SC_THREAD_PRIO_INHERIT: ::c_int = 46;
pub const _SC_THREAD_PRIO_PROTECT: ::c_int = 47;
pub const _SC_THREAD_PRIO_CEILING: ::c_int = _SC_THREAD_PRIO_PROTECT;
pub const _SC_THREAD_PROCESS_SHARED: ::c_int = 48;
pub const _SC_THREAD_SAFE_FUNCTIONS: ::c_int = 49;
pub const _SC_GETGR_R_SIZE_MAX: ::c_int = 50;
pub const _SC_GETPW_R_SIZE_MAX: ::c_int = 51;
pub const _SC_LOGIN_NAME_MAX: ::c_int = 52;
pub const _SC_THREAD_DESTRUCTOR_ITERATIONS: ::c_int = 53;
pub const _SC_ADVISORY_INFO: ::c_int = 54;
pub const _SC_ATEXIT_MAX: ::c_int = 55;
pub const _SC_BARRIERS: ::c_int = 56;
pub const _SC_BC_BASE_MAX: ::c_int = 57;
pub const _SC_BC_DIM_MAX: ::c_int = 58;
pub const _SC_BC_SCALE_MAX: ::c_int = 59;
pub const _SC_BC_STRING_MAX: ::c_int = 60;
pub const _SC_CLOCK_SELECTION: ::c_int = 61;
pub const _SC_COLL_WEIGHTS_MAX: ::c_int = 62;
pub const _SC_CPUTIME: ::c_int = 63;
pub const _SC_EXPR_NEST_MAX: ::c_int = 64;
pub const _SC_HOST_NAME_MAX: ::c_int = 65;
pub const _SC_IOV_MAX: ::c_int = 66;
pub const _SC_IPV6: ::c_int = 67;
pub const _SC_LINE_MAX: ::c_int = 68;
pub const _SC_MONOTONIC_CLOCK: ::c_int = 69;
pub const _SC_RAW_SOCKETS: ::c_int = 70;
pub const _SC_READER_WRITER_LOCKS: ::c_int = 71;
pub const _SC_REGEXP: ::c_int = 72;
pub const _SC_RE_DUP_MAX: ::c_int = 73;
pub const _SC_SHELL: ::c_int = 74;
pub const _SC_SPAWN: ::c_int = 75;
pub const _SC_SPIN_LOCKS: ::c_int = 76;
pub const _SC_SPORADIC_SERVER: ::c_int = 77;
pub const _SC_SS_REPL_MAX: ::c_int = 78;
pub const _SC_SYMLOOP_MAX: ::c_int = 79;
pub const _SC_THREAD_CPUTIME: ::c_int = 80;
pub const _SC_THREAD_SPORADIC_SERVER: ::c_int = 81;
pub const _SC_TIMEOUTS: ::c_int = 82;
pub const _SC_TRACE: ::c_int = 83;
pub const _SC_TRACE_EVENT_FILTER: ::c_int = 84;
pub const _SC_TRACE_EVENT_NAME_MAX: ::c_int = 85;
pub const _SC_TRACE_INHERIT: ::c_int = 86;
pub const _SC_TRACE_LOG: ::c_int = 87;
pub const _SC_TRACE_NAME_MAX: ::c_int = 88;
pub const _SC_TRACE_SYS_MAX: ::c_int = 89;
pub const _SC_TRACE_USER_EVENT_MAX: ::c_int = 90;
pub const _SC_TYPED_MEMORY_OBJECTS: ::c_int = 91;
pub const _SC_V7_ILP32_OFF32: ::c_int = 92;
pub const _SC_V6_ILP32_OFF32: ::c_int = _SC_V7_ILP32_OFF32;
pub const _SC_XBS5_ILP32_OFF32: ::c_int = _SC_V7_ILP32_OFF32;
pub const _SC_V7_ILP32_OFFBIG: ::c_int = 93;
pub const _SC_V6_ILP32_OFFBIG: ::c_int = _SC_V7_ILP32_OFFBIG;
pub const _SC_XBS5_ILP32_OFFBIG: ::c_int = _SC_V7_ILP32_OFFBIG;
pub const _SC_V7_LP64_OFF64: ::c_int = 94;
pub const _SC_V6_LP64_OFF64: ::c_int = _SC_V7_LP64_OFF64;
pub const _SC_XBS5_LP64_OFF64: ::c_int = _SC_V7_LP64_OFF64;
pub const _SC_V7_LPBIG_OFFBIG: ::c_int = 95;
pub const _SC_V6_LPBIG_OFFBIG: ::c_int = _SC_V7_LPBIG_OFFBIG;
pub const _SC_XBS5_LPBIG_OFFBIG: ::c_int = _SC_V7_LPBIG_OFFBIG;
pub const _SC_XOPEN_CRYPT: ::c_int = 96;
pub const _SC_XOPEN_ENH_I18N: ::c_int = 97;
pub const _SC_XOPEN_LEGACY: ::c_int = 98;
pub const _SC_XOPEN_REALTIME: ::c_int = 99;
pub const _SC_STREAM_MAX: ::c_int = 100;
pub const _SC_PRIORITY_SCHEDULING: ::c_int = 101;
pub const _SC_XOPEN_REALTIME_THREADS: ::c_int = 102;
pub const _SC_XOPEN_SHM: ::c_int = 103;
pub const _SC_XOPEN_STREAMS: ::c_int = 104;
pub const _SC_XOPEN_UNIX: ::c_int = 105;
pub const _SC_XOPEN_VERSION: ::c_int = 106;
pub const _SC_2_CHAR_TERM: ::c_int = 107;
pub const _SC_2_C_BIND: ::c_int = 108;
pub const _SC_2_C_DEV: ::c_int = 109;
pub const _SC_2_FORT_DEV: ::c_int = 110;
pub const _SC_2_FORT_RUN: ::c_int = 111;
pub const _SC_2_LOCALEDEF: ::c_int = 112;
pub const _SC_2_PBS: ::c_int = 113;
pub const _SC_2_PBS_ACCOUNTING: ::c_int = 114;
pub const _SC_2_PBS_CHECKPOINT: ::c_int = 115;
pub const _SC_2_PBS_LOCATE: ::c_int = 116;
pub const _SC_2_PBS_MESSAGE: ::c_int = 117;
pub const _SC_2_PBS_TRACK: ::c_int = 118;
pub const _SC_2_SW_DEV: ::c_int = 119;
pub const _SC_2_UPE: ::c_int = 120;
pub const _SC_2_VERSION: ::c_int = 121;
pub const _SC_THREAD_ROBUST_PRIO_INHERIT: ::c_int = 122;
pub const _SC_THREAD_ROBUST_PRIO_PROTECT: ::c_int = 123;
pub const _SC_XOPEN_UUCP: ::c_int = 124;
pub const _SC_LEVEL1_ICACHE_SIZE: ::c_int = 125;
pub const _SC_LEVEL1_ICACHE_ASSOC: ::c_int = 126;
pub const _SC_LEVEL1_ICACHE_LINESIZE: ::c_int = 127;
pub const _SC_LEVEL1_DCACHE_SIZE: ::c_int = 128;
pub const _SC_LEVEL1_DCACHE_ASSOC: ::c_int = 129;
pub const _SC_LEVEL1_DCACHE_LINESIZE: ::c_int = 130;
pub const _SC_LEVEL2_CACHE_SIZE: ::c_int = 131;
pub const _SC_LEVEL2_CACHE_ASSOC: ::c_int = 132;
pub const _SC_LEVEL2_CACHE_LINESIZE: ::c_int = 133;
pub const _SC_LEVEL3_CACHE_SIZE: ::c_int = 134;
pub const _SC_LEVEL3_CACHE_ASSOC: ::c_int = 135;
pub const _SC_LEVEL3_CACHE_LINESIZE: ::c_int = 136;
pub const _SC_LEVEL4_CACHE_SIZE: ::c_int = 137;
pub const _SC_LEVEL4_CACHE_ASSOC: ::c_int = 138;
pub const _SC_LEVEL4_CACHE_LINESIZE: ::c_int = 139;
pub const _SC_POSIX_26_VERSION: ::c_int = 140;

// sys/fcntl.h
pub const F_DUPFD: ::c_int = 0;
pub const F_GETFD: ::c_int = 1;
pub const F_SETFD: ::c_int = 2;
pub const F_GETFL: ::c_int = 3;
pub const F_SETFL: ::c_int = 4;
pub const F_GETOWN: ::c_int = 5;
pub const F_SETOWN: ::c_int = 6;
pub const F_GETLK: ::c_int = 7;
pub const F_SETLK: ::c_int = 8;
pub const F_SETLKW: ::c_int = 9;
pub const F_RGETLK: ::c_int = 10;
pub const F_RSETLK: ::c_int = 11;
pub const F_CNVT: ::c_int = 12;
pub const F_RSETLKW: ::c_int = 13;
pub const F_DUPFD_CLOEXEC: ::c_int = 14;

pub const O_ACCMODE: ::c_int = O_RDONLY | O_WRONLY | O_RDWR;

pub const O_RDONLY: ::c_int = 0;
pub const O_WRONLY: ::c_int = 1;
pub const O_RDWR: ::c_int = 2;

pub const O_APPEND: ::c_int = 0x0008;
pub const O_CREAT: ::c_int = 0x0200;
pub const O_TRUNC: ::c_int = 0x0400;
pub const O_EXCL: ::c_int = 0x0800;
pub const O_SYNC: ::c_int = 0x2000;
pub const O_NONBLOCK: ::c_int = 0x4000;
pub const O_FNOCTTY: ::c_int = 0x8000;

pub const O_CLOEXEC: ::c_int = 0x40000;
pub const O_NOFOLLOW: ::c_int = 0x100000;
pub const O_DIRECTORY: ::c_int = 0x200000;
pub const O_EXEC: ::c_int = 0x400000;
pub const O_SEARCH: ::c_int = 0x400000;

// sys/errno.h
pub const EPERM: ::c_int = 1;
pub const ENOENT: ::c_int = 2;
pub const ESRCH: ::c_int = 3;
pub const EINTR: ::c_int = 4;
pub const EIO: ::c_int = 5;
pub const ENXIO: ::c_int = 6;
pub const E2BIG: ::c_int = 7;
pub const ENOEXEC: ::c_int = 8;
pub const EBADF: ::c_int = 9;
pub const ECHILD: ::c_int = 10;
pub const EAGAIN: ::c_int = 11;
pub const ENOMEM: ::c_int = 12;
pub const EACCES: ::c_int = 13;
pub const EFAULT: ::c_int = 14;
pub const EBUSY: ::c_int = 16;
pub const EEXIST: ::c_int = 17;
pub const EXDEV: ::c_int = 18;
pub const ENODEV: ::c_int = 19;
pub const ENOTDIR: ::c_int = 20;
pub const EISDIR: ::c_int = 21;
pub const EINVAL: ::c_int = 22;
pub const ENFILE: ::c_int = 23;
pub const EMFILE: ::c_int = 24;
pub const ENOTTY: ::c_int = 25;
pub const ETXTBSY: ::c_int = 26;
pub const EFBIG: ::c_int = 27;
pub const ENOSPC: ::c_int = 28;
pub const ESPIPE: ::c_int = 29;
pub const EROFS: ::c_int = 30;
pub const EMLINK: ::c_int = 31;
pub const EPIPE: ::c_int = 32;
pub const EDOM: ::c_int = 33;
pub const ERANGE: ::c_int = 34;
pub const ENOMSG: ::c_int = 35;
pub const EIDRM: ::c_int = 36;
pub const EDEADLK: ::c_int = 45;
pub const ENOLCK: ::c_int = 46;
pub const ENOSTR: ::c_int = 60;
pub const ENODATA: ::c_int = 61;
pub const ETIME: ::c_int = 62;
pub const ENOSR: ::c_int = 63;
pub const ENOLINK: ::c_int = 67;
pub const EPROTO: ::c_int = 71;
pub const EMULTIHOP: ::c_int = 74;
pub const EBADMSG: ::c_int = 77;
pub const EFTYPE: ::c_int = 79;
pub const ENOSYS: ::c_int = 88;
pub const ENOTEMPTY: ::c_int = 90;
pub const ENAMETOOLONG: ::c_int = 91;
pub const ELOOP: ::c_int = 92;
pub const EOPNOTSUPP: ::c_int = 95;
pub const EPFNOSUPPORT: ::c_int = 96;
pub const ECONNRESET: ::c_int = 104;
pub const ENOBUFS: ::c_int = 105;
pub const EAFNOSUPPORT: ::c_int = 106;
pub const EPROTOTYPE: ::c_int = 107;
pub const ENOTSOCK: ::c_int = 108;
pub const ENOPROTOOPT: ::c_int = 109;
pub const ECONNREFUSED: ::c_int = 111;
pub const EADDRINUSE: ::c_int = 112;
pub const ECONNABORTED: ::c_int = 113;
pub const ENETUNREACH: ::c_int = 114;
pub const ENETDOWN: ::c_int = 115;
pub const ETIMEDOUT: ::c_int = 116;
pub const EHOSTDOWN: ::c_int = 117;
pub const EHOSTUNREACH: ::c_int = 118;
pub const EINPROGRESS: ::c_int = 119;
pub const EALREADY: ::c_int = 120;
pub const EDESTADDRREQ: ::c_int = 121;
pub const EMSGSIZE: ::c_int = 122;
pub const EPROTONOSUPPORT: ::c_int = 123;
pub const EADDRNOTAVAIL: ::c_int = 125;
pub const ENETRESET: ::c_int = 126;
pub const EISCONN: ::c_int = 127;
pub const ENOTCONN: ::c_int = 128;
pub const ETOOMANYREFS: ::c_int = 129;
pub const EDQUOT: ::c_int = 132;
pub const ESTALE: ::c_int = 133;
pub const ENOTSUP: ::c_int = 134;
pub const EILSEQ: ::c_int = 138;
pub const EOVERFLOW: ::c_int = 139;
pub const ECANCELED: ::c_int = 140;
pub const ENOTRECOVERABLE: ::c_int = 141;
pub const EOWNERDEAD: ::c_int = 142;
pub const EWOULDBLOCK: ::c_int = ::EAGAIN;

// sys/signal.h
pub type sigset_t = ::c_ulong;

s! {
    pub struct sigaction {
        pub sa_handler: extern fn(arg1: ::c_int),
        pub sa_mask: sigset_t,
        pub sa_flags: ::c_int,
    }
}

// sys/select.h
pub const FD_SETSIZE: usize = 64;
pub const NDFBITS: usize = 32 * 8;

pub type fd_mask = ::c_ulong;

s! {
    pub struct fd_set {
        fds_bits: [::fd_mask; (FD_SETSIZE + NDFBITS - 1) / NDFBITS]
    }
}

// pthread.h
pub type pthread_t = u32;
pub type pthread_key_t = u32;

pub const __SIZEOF_PTHREAD_ATTR_T: usize = 32;
pub const __SIZEOF_PTHREAD_MUTEX_T: usize = 16;
pub const __SIZEOF_PTHREAD_MUTEXATTR_T: usize = 12;
pub const __SIZEOF_PTHREAD_COND_T: usize = 8;
pub const __SIZEOF_PTHREAD_CONDATTR_T: usize = 8;

// These are not supported by miosix (?)
pub const __SIZEOF_PTHREAD_RWLOCK_T: usize = 0;
pub const __SIZEOF_PTHREAD_RWLOCKATTR_T: usize = 0;

align_const! {
    pub const PTHREAD_MUTEX_INITIALIZER: pthread_mutex_t = pthread_mutex_t {
        size: [0; __SIZEOF_PTHREAD_MUTEX_T],
    };
    pub const PTHREAD_COND_INITIALIZER: pthread_cond_t = pthread_cond_t {
        size: [0; __SIZEOF_PTHREAD_COND_T],
    };
    pub const PTHREAD_RWLOCK_INITIALIZER: pthread_rwlock_t = pthread_rwlock_t {
        size: [0; __SIZEOF_PTHREAD_RWLOCK_T],
    };
}

s! {
    pub struct pthread_attr_t {
        size: [u8; __SIZEOF_PTHREAD_ATTR_T]
    }

    pub struct pthread_mutex_t {
        size: [u8; __SIZEOF_PTHREAD_MUTEX_T]
    }

    pub struct pthread_mutexattr_t {
        size: [u8; __SIZEOF_PTHREAD_MUTEXATTR_T]
    }

    pub struct pthread_cond_t {
        size: [u8; __SIZEOF_PTHREAD_COND_T]
    }

    pub struct pthread_condattr_t {
        size: [u8; __SIZEOF_PTHREAD_CONDATTR_T]
    }

    pub struct pthread_rwlock_t {
        size: [u8; __SIZEOF_PTHREAD_RWLOCK_T]
    }

    pub struct pthread_rwlockattr_t {
        size: [u8; __SIZEOF_PTHREAD_RWLOCKATTR_T]
    }
}

// malloc.h
extern "C" {
    pub fn memalign(align: ::size_t, size: ::size_t) -> *mut ::c_void;
}

// pwd.h
s! {
    pub struct passwd {
        pub pw_name: *mut ::c_char,
        pub pw_passwd: *mut ::c_char,
        pub pw_uid: ::uid_t,
        pub pw_gid: ::gid_t,
        pub pw_comment: *mut ::c_char,
        pub pw_gecos: *mut ::c_char,
        pub pw_dir: *mut ::c_char,
        pub pw_shell: *mut ::c_char,
    }
}

// time.h
pub const CLOCK_MONOTONIC: ::clockid_t = 4;
pub const CLOCK_REALTIME: ::clockid_t = 1;

extern "C" {
    pub fn clock_settime(
        clock_id: ::clockid_t,
        tp: *const ::timespec,
    ) -> ::c_int;
    pub fn clock_gettime(
        clock_id: ::clockid_t,
        tp: *mut ::timespec,
    ) -> ::c_int;
    pub fn clock_getres(
        clock_id: ::clockid_t,
        res: *mut ::timespec,
    ) -> ::c_int;
}

s! {
    pub struct tm {
        pub tm_sec: ::c_int,
        pub tm_min: ::c_int,
        pub tm_hour: ::c_int,
        pub tm_mday: ::c_int,
        pub tm_mon: ::c_int,
        pub tm_year: ::c_int,
        pub tm_wday: ::c_int,
        pub tm_yday: ::c_int,
        pub tm_isdst: ::c_int,
    }
}

// locale.h
s! {
    pub struct lconv {
        pub decimal_point: *mut ::c_char,
        pub thousands_sep: *mut ::c_char,
        pub grouping: *mut ::c_char,
        pub int_curr_symbol: *mut ::c_char,
        pub currency_symbol: *mut ::c_char,
        pub mon_decimal_point: *mut ::c_char,
        pub mon_thousands_sep: *mut ::c_char,
        pub mon_grouping: *mut ::c_char,
        pub positive_sign: *mut ::c_char,
        pub negative_sign: *mut ::c_char,
        pub int_frac_digits: ::c_char,
        pub frac_digits: ::c_char,
        pub p_cs_precedes: ::c_char,
        pub p_sep_by_space: ::c_char,
        pub n_cs_precedes: ::c_char,
        pub n_sep_by_space: ::c_char,
        pub p_sign_posn: ::c_char,
        pub n_sign_posn: ::c_char,
        pub int_n_cs_precedes: ::c_char,
        pub int_n_sep_by_space: ::c_char,
        pub int_n_sign_posn: ::c_char,
        pub int_p_cs_precedes: ::c_char,
        pub int_p_sep_by_space: ::c_char,
        pub int_p_sign_posn: ::c_char,
    }
}

// Some of this structs are not supported by miosix, so they are just placeholder

// Unverified
pub type rlim_t = u32;
pub type nfds_t = u32;

// Networking types, unverified
pub type sa_family_t = u8;
pub type socklen_t = u32;

s! {
    pub struct sockaddr {
        pub sa_family: ::sa_family_t,
        pub sa_data: [::c_char; 14],
    }

    pub struct addrinfo {
        pub ai_flags: ::c_int,
        pub ai_family: ::c_int,
        pub ai_socktype: ::c_int,
        pub ai_protocol: ::c_int,
        pub ai_addrlen: socklen_t,
        pub ai_canonname: *mut ::c_char,
        pub ai_addr: *mut sockaddr,
        pub ai_next: *mut addrinfo,
    }
}

s! {
    pub struct Dl_info {
        pub dli_fname: *const ::c_char,
        pub dli_fbase: *mut ::c_void,
        pub dli_sname: *const ::c_char,
        pub dli_saddr: *mut ::c_void,
    }

    pub struct sem_t {
        __size: [::c_char; 16],
    }

    pub struct statvfs {
        pub f_bsize: ::c_ulong,
        pub f_frsize: ::c_ulong,
        pub f_blocks: fsblkcnt_t,
        pub f_bfree: fsblkcnt_t,
        pub f_bavail: fsblkcnt_t,
        pub f_files: fsfilcnt_t,
        pub f_ffree: fsfilcnt_t,
        pub f_favail: fsfilcnt_t,
        pub f_fsid: ::c_ulong,
        pub f_flag: ::c_ulong,
        pub f_namemax: ::c_ulong,
    }
}
