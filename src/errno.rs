use linux_raw_sys::errno;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Errno {
    raw: u16,
}

impl Errno {
    pub const PERM: Self = Self::from_errno(errno::EPERM);
    pub const NOENT: Self = Self::from_errno(errno::ENOENT);
    pub const SRCH: Self = Self::from_errno(errno::ESRCH);
    pub const INTR: Self = Self::from_errno(errno::EINTR);
    pub const IO: Self = Self::from_errno(errno::EIO);
    pub const NXIO: Self = Self::from_errno(errno::ENXIO);
    pub const TOOBIG: Self = Self::from_errno(errno::E2BIG);
    pub const NOEXEC: Self = Self::from_errno(errno::ENOEXEC);
    pub const BADF: Self = Self::from_errno(errno::EBADF);
    pub const CHILD: Self = Self::from_errno(errno::ECHILD);
    pub const AGAIN: Self = Self::from_errno(errno::EAGAIN);
    pub const NOMEM: Self = Self::from_errno(errno::ENOMEM);
    pub const ACCES: Self = Self::from_errno(errno::EACCES);
    pub const FAULT: Self = Self::from_errno(errno::EFAULT);
    pub const NOTBLK: Self = Self::from_errno(errno::ENOTBLK);
    pub const BUSY: Self = Self::from_errno(errno::EBUSY);
    pub const EXIST: Self = Self::from_errno(errno::EEXIST);
    pub const XDEV: Self = Self::from_errno(errno::EXDEV);
    pub const NODEV: Self = Self::from_errno(errno::ENODEV);
    pub const NOTDIR: Self = Self::from_errno(errno::ENOTDIR);
    pub const ISDIR: Self = Self::from_errno(errno::EISDIR);
    pub const INVAL: Self = Self::from_errno(errno::EINVAL);
    pub const NFILE: Self = Self::from_errno(errno::ENFILE);
    pub const MFILE: Self = Self::from_errno(errno::EMFILE);
    pub const NOTTY: Self = Self::from_errno(errno::ENOTTY);
    pub const TXTBSY: Self = Self::from_errno(errno::ETXTBSY);
    pub const FBIG: Self = Self::from_errno(errno::EFBIG);
    pub const NOSPC: Self = Self::from_errno(errno::ENOSPC);
    pub const SPIPE: Self = Self::from_errno(errno::ESPIPE);
    pub const ROFS: Self = Self::from_errno(errno::EROFS);
    pub const MLINK: Self = Self::from_errno(errno::EMLINK);
    pub const PIPE: Self = Self::from_errno(errno::EPIPE);
    pub const DOM: Self = Self::from_errno(errno::EDOM);
    pub const RANGE: Self = Self::from_errno(errno::ERANGE);
    pub const DEADLK: Self = Self::from_errno(errno::EDEADLK);
    pub const NAMETOOLONG: Self = Self::from_errno(errno::ENAMETOOLONG);
    pub const NOLCK: Self = Self::from_errno(errno::ENOLCK);
    pub const NOSYS: Self = Self::from_errno(errno::ENOSYS);
    pub const NOTEMPTY: Self = Self::from_errno(errno::ENOTEMPTY);
    pub const LOOP: Self = Self::from_errno(errno::ELOOP);
    pub const WOULDBLOCK: Self = Self::from_errno(errno::EWOULDBLOCK);
    pub const NOMSG: Self = Self::from_errno(errno::ENOMSG);
    pub const IDRM: Self = Self::from_errno(errno::EIDRM);
    pub const CHRNG: Self = Self::from_errno(errno::ECHRNG);
    pub const L2NSYNC: Self = Self::from_errno(errno::EL2NSYNC);
    pub const L3HLT: Self = Self::from_errno(errno::EL3HLT);
    pub const L3RST: Self = Self::from_errno(errno::EL3RST);
    pub const LNRNG: Self = Self::from_errno(errno::ELNRNG);
    pub const UNATCH: Self = Self::from_errno(errno::EUNATCH);
    pub const NOCSI: Self = Self::from_errno(errno::ENOCSI);
    pub const L2HLT: Self = Self::from_errno(errno::EL2HLT);
    pub const BADE: Self = Self::from_errno(errno::EBADE);
    pub const BADR: Self = Self::from_errno(errno::EBADR);
    pub const XFULL: Self = Self::from_errno(errno::EXFULL);
    pub const NOANO: Self = Self::from_errno(errno::ENOANO);
    pub const BADRQC: Self = Self::from_errno(errno::EBADRQC);
    pub const BADSLT: Self = Self::from_errno(errno::EBADSLT);
    pub const DEADLOCK: Self = Self::from_errno(errno::EDEADLOCK);
    pub const BFONT: Self = Self::from_errno(errno::EBFONT);
    pub const NOSTR: Self = Self::from_errno(errno::ENOSTR);
    pub const NODATA: Self = Self::from_errno(errno::ENODATA);
    pub const TIME: Self = Self::from_errno(errno::ETIME);
    pub const NOSR: Self = Self::from_errno(errno::ENOSR);
    pub const NONET: Self = Self::from_errno(errno::ENONET);
    pub const NOPKG: Self = Self::from_errno(errno::ENOPKG);
    pub const REMOTE: Self = Self::from_errno(errno::EREMOTE);
    pub const NOLINK: Self = Self::from_errno(errno::ENOLINK);
    pub const ADV: Self = Self::from_errno(errno::EADV);
    pub const SRMNT: Self = Self::from_errno(errno::ESRMNT);
    pub const COMM: Self = Self::from_errno(errno::ECOMM);
    pub const PROTO: Self = Self::from_errno(errno::EPROTO);
    pub const MULTIHOP: Self = Self::from_errno(errno::EMULTIHOP);
    pub const DOTDOT: Self = Self::from_errno(errno::EDOTDOT);
    pub const BADMSG: Self = Self::from_errno(errno::EBADMSG);
    pub const OVERFLOW: Self = Self::from_errno(errno::EOVERFLOW);
    pub const NOTUNIQ: Self = Self::from_errno(errno::ENOTUNIQ);
    pub const BADFD: Self = Self::from_errno(errno::EBADFD);
    pub const REMCHG: Self = Self::from_errno(errno::EREMCHG);
    pub const LIBACC: Self = Self::from_errno(errno::ELIBACC);
    pub const LIBBAD: Self = Self::from_errno(errno::ELIBBAD);
    pub const LIBSCN: Self = Self::from_errno(errno::ELIBSCN);
    pub const LIBMAX: Self = Self::from_errno(errno::ELIBMAX);
    pub const LIBEXEC: Self = Self::from_errno(errno::ELIBEXEC);
    pub const ILSEQ: Self = Self::from_errno(errno::EILSEQ);
    pub const RESTART: Self = Self::from_errno(errno::ERESTART);
    pub const STRPIPE: Self = Self::from_errno(errno::ESTRPIPE);
    pub const USERS: Self = Self::from_errno(errno::EUSERS);
    pub const NOTSOCK: Self = Self::from_errno(errno::ENOTSOCK);
    pub const DESTADDRREQ: Self = Self::from_errno(errno::EDESTADDRREQ);
    pub const MSGSIZE: Self = Self::from_errno(errno::EMSGSIZE);
    pub const PROTOTYPE: Self = Self::from_errno(errno::EPROTOTYPE);
    pub const NOPROTOOPT: Self = Self::from_errno(errno::ENOPROTOOPT);
    pub const PROTONOSUPPORT: Self = Self::from_errno(errno::EPROTONOSUPPORT);
    pub const SOCKTNOSUPPORT: Self = Self::from_errno(errno::ESOCKTNOSUPPORT);
    pub const OPNOTSUPP: Self = Self::from_errno(errno::EOPNOTSUPP);
    pub const PFNOSUPPORT: Self = Self::from_errno(errno::EPFNOSUPPORT);
    pub const AFNOSUPPORT: Self = Self::from_errno(errno::EAFNOSUPPORT);
    pub const ADDRINUSE: Self = Self::from_errno(errno::EADDRINUSE);
    pub const ADDRNOTAVAIL: Self = Self::from_errno(errno::EADDRNOTAVAIL);
    pub const NETDOWN: Self = Self::from_errno(errno::ENETDOWN);
    pub const NETUNREACH: Self = Self::from_errno(errno::ENETUNREACH);
    pub const NETRESET: Self = Self::from_errno(errno::ENETRESET);
    pub const CONNABORTED: Self = Self::from_errno(errno::ECONNABORTED);
    pub const CONNRESET: Self = Self::from_errno(errno::ECONNRESET);
    pub const NOBUFS: Self = Self::from_errno(errno::ENOBUFS);
    pub const ISCONN: Self = Self::from_errno(errno::EISCONN);
    pub const NOTCONN: Self = Self::from_errno(errno::ENOTCONN);
    pub const SHUTDOWN: Self = Self::from_errno(errno::ESHUTDOWN);
    pub const TOOMANYREFS: Self = Self::from_errno(errno::ETOOMANYREFS);
    pub const TIMEDOUT: Self = Self::from_errno(errno::ETIMEDOUT);
    pub const CONNREFUSED: Self = Self::from_errno(errno::ECONNREFUSED);
    pub const HOSTDOWN: Self = Self::from_errno(errno::EHOSTDOWN);
    pub const HOSTUNREACH: Self = Self::from_errno(errno::EHOSTUNREACH);
    pub const ALREADY: Self = Self::from_errno(errno::EALREADY);
    pub const INPROGRESS: Self = Self::from_errno(errno::EINPROGRESS);
    pub const STALE: Self = Self::from_errno(errno::ESTALE);
    pub const UCLEAN: Self = Self::from_errno(errno::EUCLEAN);
    pub const NOTNAM: Self = Self::from_errno(errno::ENOTNAM);
    pub const NAVAIL: Self = Self::from_errno(errno::ENAVAIL);
    pub const ISNAM: Self = Self::from_errno(errno::EISNAM);
    pub const REMOTEIO: Self = Self::from_errno(errno::EREMOTEIO);
    pub const DQUOT: Self = Self::from_errno(errno::EDQUOT);
    pub const NOMEDIUM: Self = Self::from_errno(errno::ENOMEDIUM);
    pub const MEDIUMTYPE: Self = Self::from_errno(errno::EMEDIUMTYPE);
    pub const CANCELED: Self = Self::from_errno(errno::ECANCELED);
    pub const NOKEY: Self = Self::from_errno(errno::ENOKEY);
    pub const KEYEXPIRED: Self = Self::from_errno(errno::EKEYEXPIRED);
    pub const KEYREVOKED: Self = Self::from_errno(errno::EKEYREVOKED);
    pub const KEYREJECTED: Self = Self::from_errno(errno::EKEYREJECTED);
    pub const OWNERDEAD: Self = Self::from_errno(errno::EOWNERDEAD);
    pub const NOTRECOVERABLE: Self = Self::from_errno(errno::ENOTRECOVERABLE);
    pub const RFKILL: Self = Self::from_errno(errno::ERFKILL);
    pub const HWPOISON: Self = Self::from_errno(errno::EHWPOISON);

    pub(crate) const unsafe fn from_raw(raw: u16) -> Self {
        Self { raw }
    }

    #[inline]
    pub const fn raw_os_error(self) -> i32 {
        (self.raw as i16 as i32).wrapping_neg()
    }

    const fn from_errno(errno: u32) -> Self {
        Self {
            raw: errno.wrapping_neg() as u16,
        }
    }
}

pub type Result<T, E = Errno> = core::result::Result<T, E>;
