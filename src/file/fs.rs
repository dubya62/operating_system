////////////////////////////////////////////////////
//! The purpose of this file is to provide
//! filesystem functionality.
//!
//! This file offers the following public functionality:
//!
//! struct stat {
//!     st_dev: i32,
//!     st_ino: i32,
//!     st_mode: i32,
//!     st_nlink: i32,
//!     st_uid: i32,
//!     st_gid: i32,
//!     st_rdev: i32,
//!     st_size: i32,
//!     st_blksize: i32,
//!     blkcnt_t: i32,
//!
//!     st_atim: timestruct::TimeSpec,
//!     st_mtim: timestruct::TimeSpec,
//!     st_ctim: timestruct::TimeSpec,
//! }
//!     new() -> Self - constructor
//!     empty() -> Self - construct with each field empty
//!     
//!     
//!
////////////////////////////////////////////////////

/*
Linux implementation of file information

struct stat{
    dev_t      st_dev;      /* ID of device containing file */
    ino_t      st_ino;      /* Inode number */
    mode_t     st_mode;     /* File type and mode */
    nlink_t    st_nlink;    /* Number of hard links */
    uid_t      st_uid;      /* User ID of owner */
    gid_t      st_gid;      /* Group ID of owner */
    dev_t      st_rdev;     /* Device ID (if special file) */
    off_t      st_size;     /* Total size, in bytes */
    blksize_t  st_blksize;  /* Block size for filesystem I/O */
    blkcnt_t   st_blocks;   /* Number of 512 B blocks allocated */

    /* Since POSIX.1-2008, this structure supports nanosecond
      precision for the following timestamp fields.
      For the details before POSIX.1-2008, see VERSIONS. */

    struct timespec  st_atim;  /* Time of last access */
    struct timespec  st_mtim;  /* Time of last modification */
    struct timespec  st_ctim;  /* Time of last status change */
}

*/

use crate::time::timestruct;

// file information structure
pub struct Stat {
    pub st_dev: i32,
    pub st_ino: i32,
    pub st_mode: i32,
    pub st_nlink: i32,
    pub st_uid: i32,
    pub st_gid: i32,
    pub st_rdev: i32,
    pub st_size: i32,
    pub st_blksize: i32,
    pub st_blocks: i32,

    pub st_atim: timestruct::TimeSpec,
    pub st_mtim: timestruct::TimeSpec,
    pub st_ctim: timestruct::TimeSpec,
}

impl Stat {
    pub fn new(
        st_dev: i32,
        st_ino: i32,
        st_mode: i32,
        st_nlink: i32,
        st_uid: i32,
        st_gid: i32,
        st_rdev: i32,
        st_size: i32,
        st_blksize: i32,
        st_blocks: i32,
        st_atim: timestruct::TimeSpec,
        st_mtim: timestruct::TimeSpec,
        st_ctim: timestruct::TimeSpec,
    ) -> Self {
        return Stat {
            st_dev: st_dev,
            st_ino: st_ino,
            st_mode: st_mode,
            st_nlink: st_nlink,
            st_uid: st_uid,
            st_gid: st_gid,
            st_rdev: st_rdev,
            st_size: st_size,
            st_blksize: st_blksize,
            st_blocks: st_blocks,

            st_atim: st_atim,
            st_mtim: st_mtim,
            st_ctim: st_ctim,
        };
    }

    pub fn empty() -> Self {
        return Stat {
            st_dev: 0,
            st_ino: 0,
            st_mode: 0,
            st_nlink: 0,
            st_uid: 0,
            st_gid: 0,
            st_rdev: 0,
            st_size: 0,
            st_blksize: 0,
            st_blocks: 0,

            st_atim: timestruct::TimeSpec::empty(),
            st_mtim: timestruct::TimeSpec::empty(),
            st_ctim: timestruct::TimeSpec::empty(),
        };
    }
}
