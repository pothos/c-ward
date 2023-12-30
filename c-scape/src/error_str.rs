use rustix::io::Errno;

/// Return a string message for the given error.
///
/// This is used to implement `strerror`-like functions.
pub(crate) const fn error_str(e: Errno) -> Option<&'static str> {
    // Recognize errors documented in POSIX and use the documented strings.
    // <https://pubs.opengroup.org/onlinepubs/9699919799/basedefs/errno.h.html>
    Some(match e {
        Errno::TOOBIG => "Argument list too long",
        Errno::ACCESS => "Permission denied",
        Errno::ADDRINUSE => "Address in use",
        Errno::ADDRNOTAVAIL => "Address not available",
        Errno::AFNOSUPPORT => "Address family not supported",
        Errno::AGAIN => "Resource unavailable, try again",
        Errno::ALREADY => "Connection already in progress",
        Errno::BADF => "Bad file descriptor",
        Errno::BADMSG => "Bad message",
        Errno::BUSY => "Device or resource busy",
        Errno::CANCELED => "Operation canceled",
        Errno::CHILD => "No child processes",
        Errno::CONNABORTED => "Connection aborted",
        Errno::CONNREFUSED => "Connection refused",
        Errno::CONNRESET => "Connection reset",
        Errno::DEADLK => "Resource deadlock would occur",
        Errno::DESTADDRREQ => "Destination address required",
        Errno::DOM => "Mathematics argument out of domain of function",
        Errno::DQUOT => "Reserved",
        Errno::EXIST => "File exists",
        Errno::FAULT => "Bad address",
        Errno::FBIG => "File too large",
        Errno::HOSTUNREACH => "Host is unreachable",
        Errno::IDRM => "Identifier removed",
        Errno::ILSEQ => "Invalid byte sequence",
        Errno::INPROGRESS => "Operation in progress",
        Errno::INTR => "Interrupted function",
        Errno::INVAL => "Invalid argument",
        Errno::IO => "I/O error",
        Errno::ISCONN => "Socket is connected",
        Errno::ISDIR => "Is a directory",
        Errno::LOOP => "Too many levels of symbolic links",
        Errno::MFILE => "File descriptor value too large",
        Errno::MLINK => "Too many links",
        Errno::MSGSIZE => "Message too large",
        Errno::MULTIHOP => "Reserved",
        Errno::NAMETOOLONG => "Filename too long",
        Errno::NETDOWN => "Network is down",
        Errno::NETRESET => "Connection aborted by network",
        Errno::NETUNREACH => "Network unreachable",
        Errno::NFILE => "Too many files open in system",
        Errno::NOBUFS => "No buffer space available",
        #[cfg(not(target_os = "wasi"))]
        Errno::NODATA => "No message is available on the STREAM head read queue",
        Errno::NODEV => "No such device",
        Errno::NOENT => "No such file or directory",
        Errno::NOEXEC => "Executable file format error",
        Errno::NOLCK => "No locks available",
        Errno::NOLINK => "Reserved",

        // Some testsuites depend on the specific string we get, so
        // match the string from the platform libc.
        #[cfg(target_env = "musl")]
        Errno::NOMEM => "Out of memory",
        #[cfg(target_env = "gnu")]
        Errno::NOMEM => "Cannot allocate memory",
        #[cfg(not(any(target_env = "gnu", target_env = "musl")))]
        Errno::NOMEM => "Not enough space", // default to POSIX's string

        Errno::NOMSG => "No message of the desired type",
        Errno::NOPROTOOPT => "Protocol not available",
        Errno::NOSPC => "No space left on device",
        #[cfg(not(target_os = "wasi"))]
        Errno::NOSR => "No STREAM resources",
        #[cfg(not(target_os = "wasi"))]
        Errno::NOSTR => "Not a STREAM",
        Errno::NOSYS => "Functionality not supported",
        Errno::NOTCONN => "The socket is not connected",
        Errno::NOTDIR => "Not a directory",
        Errno::NOTEMPTY => "Directory not empty",
        Errno::NOTRECOVERABLE => "State not recoverable",
        Errno::NOTSOCK => "Not a socket",
        Errno::NOTSUP => "Not supported",
        Errno::NOTTY => "Inappropriate I/O control operation",
        Errno::NXIO => "No such device or address",
        //Errno::OPNOTSUPP => "Operation not supported on socket", // same as `NOTSUP`
        Errno::OVERFLOW => "Value too large to be stored in data type",
        Errno::OWNERDEAD => "Previous owner died",
        Errno::PERM => "Operation not permitted",
        Errno::PIPE => "Broken pipe",
        Errno::PROTO => "Protocol error",
        Errno::PROTONOSUPPORT => "Protocol not supported",
        Errno::PROTOTYPE => "Protocol wrong type for socket",
        Errno::RANGE => "Result too large",
        Errno::ROFS => "Read-only file system",
        Errno::SPIPE => "Invalid seek",
        Errno::SRCH => "No such process",
        Errno::STALE => "Reserved",
        #[cfg(not(target_os = "wasi"))]
        Errno::TIME => "Stream ioctl() timeout",
        Errno::TIMEDOUT => "Connection timed out",
        Errno::TXTBSY => "Text file busy",
        //Errno::WOULDBLOCK => "Operation would block", // same as `AGAIN`
        Errno::XDEV => "Cross-device link",
        #[cfg(target_os = "wasi")]
        Errno::NOTCAPABLE => "Capabilities insufficient",
        _ => return None,
    })
}
