//! The purpose of this file is to allow graceful error handling
//! For now, the main usage will be to help debug other parts of the kernel
//! It will use Linux error codes
//!
//! It provides the following public functionality:
//!
//! struct Error {
//!     errno: i32
//! }
//!     new(errno: i32) -> Self - constructor
//!     perror(&self) -> () - print the string representation of an error number
//!     (NOT IMPLEMENTED YET): send_sig(&self, pid: i32) -> () - send a signal to a process based on the error number
//!

pub const EPERM: i32 = 1;
pub const ENOENT: i32 = 2;
pub const ESRCH: i32 = 3;
pub const EINTR: i32 = 4;
pub const EIO: i32 = 5;
pub const ENXIO: i32 = 6;
pub const E2BIG: i32 = 7;
pub const ENOEXEC: i32 = 8;
pub const EBADF: i32 = 9;
pub const ECHILD: i32 = 10;
pub const EAGAIN: i32 = 11;
pub const ENOMEM: i32 = 12;
pub const EACCES: i32 = 13;
pub const EFAULT: i32 = 14;
pub const ENOTBLK: i32 = 15;
pub const EBUSY: i32 = 16;
pub const EEXIST: i32 = 17;
pub const EXDEV: i32 = 18;
pub const ENODEV: i32 = 19;
pub const ENOTDIR: i32 = 20;
pub const EISDIR: i32 = 21;
pub const EINVAL: i32 = 22;
pub const ENFILE: i32 = 23;
pub const EMFILE: i32 = 24;
pub const ENOTTY: i32 = 25;
pub const ETXTBSY: i32 = 26;
pub const EFBIG: i32 = 27;
pub const ENOSPC: i32 = 28;
pub const ESPIPE: i32 = 29;
pub const EROFS: i32 = 30;
pub const EMLINK: i32 = 31;
pub const EPIPE: i32 = 32;
pub const EDOM: i32 = 33;
pub const ERANGE: i32 = 34;
pub const EDEADLK: i32 = 35;
pub const ENAMETOOLONG: i32 = 36;
pub const ENOLCK: i32 = 37;
pub const ENOSYS: i32 = 38;
pub const ENOTEMPTY: i32 = 39;
pub const ELOOP: i32 = 40;
pub const EWOULDBLOCK: i32 = 11;
pub const ENOMSG: i32 = 42;
pub const EIDRM: i32 = 43;
pub const ECHRNG: i32 = 44;
pub const EL2NSYNC: i32 = 45;
pub const EL3HLT: i32 = 46;
pub const EL3RST: i32 = 47;
pub const ELNRNG: i32 = 48;
pub const EUNATCH: i32 = 49;
pub const ENOCSI: i32 = 50;
pub const EL2HLT: i32 = 51;
pub const EBADE: i32 = 52;
pub const EBADR: i32 = 53;
pub const EXFULL: i32 = 54;
pub const ENOANO: i32 = 55;
pub const EBADRQC: i32 = 56;
pub const EBADSLT: i32 = 57;
pub const EDEADLOCK: i32 = 35;
pub const EBFONT: i32 = 59;
pub const ENOSTR: i32 = 60;
pub const ENODATA: i32 = 61;
pub const ETIME: i32 = 62;
pub const ENOSR: i32 = 63;
pub const ENONET: i32 = 64;
pub const ENOPKG: i32 = 65;
pub const EREMOTE: i32 = 66;
pub const ENOLINK: i32 = 67;
pub const EADV: i32 = 68;
pub const ESRMNT: i32 = 69;
pub const ECOMM: i32 = 70;
pub const EPROTO: i32 = 71;
pub const EMULTIHOP: i32 = 72;
pub const EDOTDOT: i32 = 73;
pub const EBADMSG: i32 = 74;
pub const EOVERFLOW: i32 = 75;
pub const ENOTUNIQ: i32 = 76;
pub const EBADFD: i32 = 77;
pub const EREMCHG: i32 = 78;
pub const ELIBACC: i32 = 79;
pub const ELIBBAD: i32 = 80;
pub const ELIBSCN: i32 = 81;
pub const ELIBMAX: i32 = 82;
pub const ELIBEXEC: i32 = 83;
pub const EILSEQ: i32 = 84;
pub const ERESTART: i32 = 85;
pub const ESTRPIPE: i32 = 86;
pub const EUSERS: i32 = 87;
pub const ENOTSOCK: i32 = 88;
pub const EDESTADDRREQ: i32 = 89;
pub const EMSGSIZE: i32 = 90;
pub const EPROTOTYPE: i32 = 91;
pub const ENOPROTOOPT: i32 = 92;
pub const EPROTONOSUPPORT: i32 = 93;
pub const ESOCKTNOSUPPORT: i32 = 94;
pub const EOPNOTSUPP: i32 = 95;
pub const EPFNOSUPPORT: i32 = 96;
pub const EAFNOSUPPORT: i32 = 97;
pub const EADDRINUSE: i32 = 98;
pub const EADDRNOTAVAIL: i32 = 99;
pub const ENETDOWN: i32 = 100;
pub const ENETUNREACH: i32 = 101;
pub const ENETRESET: i32 = 102;
pub const ECONNABORTED: i32 = 103;
pub const ECONNRESET: i32 = 104;
pub const ENOBUFS: i32 = 105;
pub const EISCONN: i32 = 106;
pub const ENOTCONN: i32 = 107;
pub const ESHUTDOWN: i32 = 108;
pub const ETOOMANYREFS: i32 = 109;
pub const ETIMEDOUT: i32 = 110;
pub const ECONNREFUSED: i32 = 111;
pub const EHOSTDOWN: i32 = 112;
pub const EHOSTUNREACH: i32 = 113;
pub const EALREADY: i32 = 114;
pub const EINPROGRESS: i32 = 115;
pub const ESTALE: i32 = 116;
pub const EUCLEAN: i32 = 117;
pub const ENOTNAM: i32 = 118;
pub const ENAVAIL: i32 = 119;
pub const EISNAM: i32 = 120;
pub const EREMOTEIO: i32 = 121;
pub const EDQUOT: i32 = 122;
pub const ENOMEDIUM: i32 = 123;
pub const EMEDIUMTYPE: i32 = 124;
pub const ECANCELED: i32 = 125;
pub const ENOKEY: i32 = 126;
pub const EKEYEXPIRED: i32 = 127;
pub const EKEYREVOKED: i32 = 128;
pub const EKEYREJECTED: i32 = 129;
pub const EOWNERDEAD: i32 = 130;
pub const ENOTRECOVERABLE: i32 = 131;
pub const ERFKILL: i32 = 132;
pub const EHWPOISON: i32 = 133;
pub const ENOTSUP: i32 = 95;

pub struct Error {
    pub errno: i32, // error number of the error
}

impl Error {
    // constructor
    pub fn new(errno: i32) -> Self {
        return Error { errno: errno };
    }

    pub fn perror(&self) {
        // print the string version of the error number
        println!(
            "{}",
            match self.errno {
                EPERM => "Operation not permitted",
                ENOENT => "No such file or directory",
                ESRCH => "No such process",
                EINTR => "Interrupted system call",
                EIO => "Input/output error",
                ENXIO => "No such device or address",
                E2BIG => "Argument list too long",
                ENOEXEC => "Exec format error",
                EBADF => "Bad file descriptor",
                ECHILD => "No child processes",
                EAGAIN => "Resource temporarily unavailable",
                ENOMEM => "Cannot allocate memory",
                EACCES => "Permission denied",
                EFAULT => "Bad address",
                ENOTBLK => "Block device required",
                EBUSY => "Device or resource busy",
                EEXIST => "File exists",
                EXDEV => "Invalid cross-device link",
                ENODEV => "No such device",
                ENOTDIR => "Not a directory",
                EISDIR => "Is a directory",
                EINVAL => "Invalid argument",
                ENFILE => "Too many open files in system",
                EMFILE => "Too many open files",
                ENOTTY => "Inappropriate ioctl for device",
                ETXTBSY => "Text file busy",
                EFBIG => "File too large",
                ENOSPC => "No space left on device",
                ESPIPE => "Illegal seek",
                EROFS => "Read-only file system",
                EMLINK => "Too many links",
                EPIPE => "Broken pipe",
                EDOM => "Numerical argument out of domain",
                ERANGE => "Numerical result out of range",
                EDEADLK => "Resource deadlock avoided",
                ENAMETOOLONG => "File name too long",
                ENOLCK => "No locks available",
                ENOSYS => "Function not implemented",
                ENOTEMPTY => "Directory not empty",
                ELOOP => "Too many levels of symbolic links",
                ENOMSG => "No message of desired type",
                EIDRM => "Identifier removed",
                ECHRNG => "Channel number out of range",
                EL2NSYNC => "Level 2 not synchronized",
                EL3HLT => "Level 3 halted",
                EL3RST => "Level 3 reset",
                ELNRNG => "Link number out of range",
                EUNATCH => "Protocol driver not attached",
                ENOCSI => "No CSI structure available",
                EL2HLT => "Level 2 halted",
                EBADE => "Invalid exchange",
                EBADR => "Invalid request descriptor",
                EXFULL => "Exchange full",
                ENOANO => "No anode",
                EBADRQC => "Invalid request code",
                EBADSLT => "Invalid slot",
                EBFONT => "Bad font file format",
                ENOSTR => "Device not a stream",
                ENODATA => "No data available",
                ETIME => "Timer expired",
                ENOSR => "Out of streams resources",
                ENONET => "Machine is not on the network",
                ENOPKG => "Package not installed",
                EREMOTE => "Object is remote",
                ENOLINK => "Link has been severed",
                EADV => "Advertise error",
                ESRMNT => "Srmount error",
                ECOMM => "Communication error on send",
                EPROTO => "Protocol error",
                EMULTIHOP => "Multihop attempted",
                EDOTDOT => "RFS specific error",
                EBADMSG => "Bad message",
                EOVERFLOW => "Value too large for defined data type",
                ENOTUNIQ => "Name not unique on network",
                EBADFD => "File descriptor in bad state",
                EREMCHG => "Remote address changed",
                ELIBACC => "Can not access a needed shared library",
                ELIBBAD => "Accessing a corrupted shared library",
                ELIBSCN => ".lib section in a.out corrupted",
                ELIBMAX => "Attempting to link in too many shared libraries",
                ELIBEXEC => "Cannot exec a shared library directly",
                EILSEQ => "Invalid or incomplete multibyte or wide character",
                ERESTART => "Interrupted system call should be restarted",
                ESTRPIPE => "Streams pipe error",
                EUSERS => "Too many users",
                ENOTSOCK => "Socket operation on non-socket",
                EDESTADDRREQ => "Destination address required",
                EMSGSIZE => "Message too long",
                EPROTOTYPE => "Protocol wrong type for socket",
                ENOPROTOOPT => "Protocol not available",
                EPROTONOSUPPORT => "Protocol not supported",
                ESOCKTNOSUPPORT => "Socket type not supported",
                EOPNOTSUPP => "Operation not supported",
                EPFNOSUPPORT => "Protocol family not supported",
                EAFNOSUPPORT => "Address family not supported by protocol",
                EADDRINUSE => "Address already in use",
                EADDRNOTAVAIL => "Cannot assign requested address",
                ENETDOWN => "Network is down",
                ENETUNREACH => "Network is unreachable",
                ENETRESET => "Network dropped connection on reset",
                ECONNABORTED => "Software caused connection abort",
                ECONNRESET => "Connection reset by peer",
                ENOBUFS => "No buffer space available",
                EISCONN => "Transport endpoint is already connected",
                ENOTCONN => "Transport endpoint is not connected",
                ESHUTDOWN => "Cannot send after transport endpoint shutdown",
                ETOOMANYREFS => "Too many references: cannot splice",
                ETIMEDOUT => "Connection timed out",
                ECONNREFUSED => "Connection refused",
                EHOSTDOWN => "Host is down",
                EHOSTUNREACH => "No route to host",
                EALREADY => "Operation already in progress",
                EINPROGRESS => "Operation now in progress",
                ESTALE => "Stale file handle",
                EUCLEAN => "Structure needs cleaning",
                ENOTNAM => "Not a XENIX named type file",
                ENAVAIL => "No XENIX semaphores available",
                EISNAM => "Is a named type file",
                EREMOTEIO => "Remote I/O error",
                EDQUOT => "Disk quota exceeded",
                ENOMEDIUM => "No medium found",
                EMEDIUMTYPE => "Wrong medium type",
                ECANCELED => "Operation canceled",
                ENOKEY => "Required key not available",
                EKEYEXPIRED => "Key has expired",
                EKEYREVOKED => "Key has been revoked",
                EKEYREJECTED => "Key was rejected by service",
                EOWNERDEAD => "Owner died",
                ENOTRECOVERABLE => "State not recoverable",
                ERFKILL => "Operation not possible due to RF-kill",
                EHWPOISON => "Memory page has hardware error",
                _ => "Unknown Error",
            }
        );
    }

    // TODO: make this function send a signal to a process based
    // on the error number
    // pub fn send_sig(&self, pid: i32) {}
}
