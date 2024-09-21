////////////////////////////////////////////////////
//! The purpose of this file is to provide
//! filesystem functionality.
//!
//! This file offers the following public functionality:
//!
//! mod fs
//!     struct stat {
//!     }
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

pub mod fs{
    pub struct Stat {
        st_dev: i32,
        st_ino: i32,
        st_mode: i32,
        st_nlink: i32,
        st_uid: i32,
        st_gid: i32,
        st_rdev: i32,
        st_size: i32,
        st_blksize: i32,
        blkcnt_t: i32,
    }

    
}




