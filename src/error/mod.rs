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

use core::fmt::Display;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum Error {
    PERM = 1,
    NOENT,
    SRCH,
    INTR,
    IO,
    NXIO,
    TOOBIG,
    NOEXEC,
    BADF,
    CHILD,
    AGAIN,
    NOMEM,
    ACCES,
    FAULT,
    NOTBLK,
    BUSY,
    EXIST,
    XDEV,
    NODEV,
    NOTDIR,
    ISDIR,
    INVAL,
    NFILE,
    MFILE,
    NOTTY,
    TXTBSY,
    FBIG,
    NOSPC,
    SPIPE,
    ROFS,
    MLINK,
    PIPE,
    DOM,
    RANGE,
    DEADLK,
    NAMETOOLONG,
    NOLCK,
    NOSYS,
    NOTEMPTY,
    LOOP,
    NOMSG,
    IDRM,
    CHRNG,
    L2NSYNC,
    L3HLT,
    L3RST,
    LNRNG,
    UNATCH,
    NOCSI,
    L2HLT,
    BADE,
    BADR,
    XFULL,
    NOANO,
    BADRQC,
    BADSLT,
    BFONT,
    NOSTR,
    NODATA,
    TIME,
    NOSR,
    NONET,
    NOPKG,
    REMOTE,
    NOLINK,
    ADV,
    SRMNT,
    COMM,
    PROTO,
    MULTIHOP,
    DOTDOT,
    BADMSG,
    OVERFLOW,
    NOTUNIQ,
    BADFD,
    REMCHG,
    LIBACC,
    LIBBAD,
    LIBSCN,
    LIBMAX,
    LIBEXEC,
    ILSEQ,
    RESTART,
    STRPIPE,
    USERS,
    NOTSOCK,
    DESTADDRREQ,
    MSGSIZE,
    PROTOTYPE,
    NOPROTOOPT,
    PROTONOSUPPORT,
    SOCKTNOSUPPORT,
    OPNOTSUPP,
    PFNOSUPPORT,
    AFNOSUPPORT,
    ADDRINUSE,
    ADDRNOTAVAIL,
    NETDOWN,
    NETUNREACH,
    NETRESET,
    CONNABORTED,
    CONNRESET,
    NOBUFS,
    ISCONN,
    NOTCONN,
    SHUTDOWN,
    TOOMANYREFS,
    TIMEDOUT,
    CONNREFUSED,
    HOSTDOWN,
    HOSTUNREACH,
    ALREADY,
    INPROGRESS,
    STALE,
    UCLEAN,
    NOTNAM,
    NAVAIL,
    ISNAM,
    REMOTEIO,
    DQUOT,
    NOMEDIUM,
    MEDIUMTYPE,
    CANCELED,
    NOKEY,
    KEYEXPIRED,
    KEYREVOKED,
    KEYREJECTED,
    OWNERDEAD,
    NOTRECOVERABLE,
    RFKILL,
    HWPOISON,
}

impl Error {
    // TODO: make this function send a signal to a process based
    // on the error number
    // pub fn send_sig(&self, pid: i32) {}
}

impl Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::PERM => "Operation not permitted",
                Self::NOENT => "No such file or directory",
                Self::SRCH => "No such process",
                Self::INTR => "Interrupted system call",
                Self::IO => "Input/output error",
                Self::NXIO => "No such device or address",
                Self::TOOBIG => "Argument list too long",
                Self::NOEXEC => "Exec format error",
                Self::BADF => "Bad file descriptor",
                Self::CHILD => "No child processes",
                Self::AGAIN => "Resource temporarily unavailable",
                Self::NOMEM => "Cannot allocate memory",
                Self::ACCES => "Permission denied",
                Self::FAULT => "Bad address",
                Self::NOTBLK => "Block device required",
                Self::BUSY => "Device or resource busy",
                Self::EXIST => "File exists",
                Self::XDEV => "Invalid cross-device link",
                Self::NODEV => "No such device",
                Self::NOTDIR => "Not a directory",
                Self::ISDIR => "Is a directory",
                Self::INVAL => "Invalid argument",
                Self::NFILE => "Too many open files in system",
                Self::MFILE => "Too many open files",
                Self::NOTTY => "Inappropriate ioctl for device",
                Self::TXTBSY => "Text file busy",
                Self::FBIG => "File too large",
                Self::NOSPC => "No space left on device",
                Self::SPIPE => "Illegal seek",
                Self::ROFS => "Read-only file system",
                Self::MLINK => "Too many links",
                Self::PIPE => "Broken pipe",
                Self::DOM => "Numerical argument out of domain",
                Self::RANGE => "Numerical result out of range",
                Self::DEADLK => "Resource deadlock avoided",
                Self::NAMETOOLONG => "File name too long",
                Self::NOLCK => "No locks available",
                Self::NOSYS => "Function not implemented",
                Self::NOTEMPTY => "Directory not empty",
                Self::LOOP => "Too many levels of symbolic links",
                Self::NOMSG => "No message of desired type",
                Self::IDRM => "Identifier removed",
                Self::CHRNG => "Channel number out of range",
                Self::L2NSYNC => "Level 2 not synchronized",
                Self::L3HLT => "Level 3 halted",
                Self::L3RST => "Level 3 reset",
                Self::LNRNG => "Link number out of range",
                Self::UNATCH => "Protocol driver not attached",
                Self::NOCSI => "No CSI structure available",
                Self::L2HLT => "Level 2 halted",
                Self::BADE => "Invalid exchange",
                Self::BADR => "Invalid request descriptor",
                Self::XFULL => "Exchange full",
                Self::NOANO => "No anode",
                Self::BADRQC => "Invalid request code",
                Self::BADSLT => "Invalid slot",
                Self::BFONT => "Bad font file format",
                Self::NOSTR => "Device not a stream",
                Self::NODATA => "No data available",
                Self::TIME => "Timer expired",
                Self::NOSR => "Out of streams resources",
                Self::NONET => "Machine is not on the network",
                Self::NOPKG => "Package not installed",
                Self::REMOTE => "Object is remote",
                Self::NOLINK => "Link has been severed",
                Self::ADV => "Advertise error",
                Self::SRMNT => "Srmount error",
                Self::COMM => "Communication error on send",
                Self::PROTO => "Protocol error",
                Self::MULTIHOP => "Multihop attempted",
                Self::DOTDOT => "RFS specific error",
                Self::BADMSG => "Bad message",
                Self::OVERFLOW => "Value too large for defined data type",
                Self::NOTUNIQ => "Name not unique on network",
                Self::BADFD => "File descriptor in bad state",
                Self::REMCHG => "Remote address changed",
                Self::LIBACC => "Can not access a needed shared library",
                Self::LIBBAD => "Accessing a corrupted shared library",
                Self::LIBSCN => ".lib section in a.out corrupted",
                Self::LIBMAX => "Attempting to link in too many shared libraries",
                Self::LIBEXEC => "Cannot exec a shared library directly",
                Self::ILSEQ => "Invalid or incomplete multibyte or wide character",
                Self::RESTART => "Interrupted system call should be restarted",
                Self::STRPIPE => "Streams pipe error",
                Self::USERS => "Too many users",
                Self::NOTSOCK => "Socket operation on non-socket",
                Self::DESTADDRREQ => "Destination address required",
                Self::MSGSIZE => "Message too long",
                Self::PROTOTYPE => "Protocol wrong type for socket",
                Self::NOPROTOOPT => "Protocol not available",
                Self::PROTONOSUPPORT => "Protocol not supported",
                Self::SOCKTNOSUPPORT => "Socket type not supported",
                Self::OPNOTSUPP => "Operation not supported",
                Self::PFNOSUPPORT => "Protocol family not supported",
                Self::AFNOSUPPORT => "Address family not supported by protocol",
                Self::ADDRINUSE => "Address already in use",
                Self::ADDRNOTAVAIL => "Cannot assign requested address",
                Self::NETDOWN => "Network is down",
                Self::NETUNREACH => "Network is unreachable",
                Self::NETRESET => "Network dropped connection on reset",
                Self::CONNABORTED => "Software caused connection abort",
                Self::CONNRESET => "Connection reset by peer",
                Self::NOBUFS => "No buffer space available",
                Self::ISCONN => "Transport endpoint is already connected",
                Self::NOTCONN => "Transport endpoint is not connected",
                Self::SHUTDOWN => "Cannot send after transport endpoint shutdown",
                Self::TOOMANYREFS => "Too many references: cannot splice",
                Self::TIMEDOUT => "Connection timed out",
                Self::CONNREFUSED => "Connection refused",
                Self::HOSTDOWN => "Host is down",
                Self::HOSTUNREACH => "No route to host",
                Self::ALREADY => "Operation already in progress",
                Self::INPROGRESS => "Operation now in progress",
                Self::STALE => "Stale file handle",
                Self::UCLEAN => "Structure needs cleaning",
                Self::NOTNAM => "Not a XENIX named type file",
                Self::NAVAIL => "No XENIX semaphores available",
                Self::ISNAM => "Is a named type file",
                Self::REMOTEIO => "Remote I/O error",
                Self::DQUOT => "Disk quota exceeded",
                Self::NOMEDIUM => "No medium found",
                Self::MEDIUMTYPE => "Wrong medium type",
                Self::CANCELED => "Operation canceled",
                Self::NOKEY => "Required key not available",
                Self::KEYEXPIRED => "Key has expired",
                Self::KEYREVOKED => "Key has been revoked",
                Self::KEYREJECTED => "Key was rejected by service",
                Self::OWNERDEAD => "Owner died",
                Self::NOTRECOVERABLE => "State not recoverable",
                Self::RFKILL => "Operation not possible due to RF-kill",
                Self::HWPOISON => "Memory page has hardware error",
            }
        )
    }
}
