/////////////////////////////////////////////
//! The purpose of this file is to provide basic
//! time structures
//!
//! It provides the following public functionality:
//!
//! struct TimeSpec{
//!     tv_sec: i32,
//!     tv_nsec: i32,
//! }
//!     new(tv_sec: i32, tv_nsec: i32) -> Self - constructor
//!     empty() -> Self - constructor with empty vals
//!
/////////////////////////////////////////////

// timespec structure
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct TimeSpec {
    pub tv_sec: i32,
    pub tv_nsec: i32,
}

impl TimeSpec {
    // Construct a TimeSpec instance with known time
    pub fn new(tv_sec: i32, tv_nsec: i32) -> Self {
        TimeSpec { tv_sec, tv_nsec }
    }

    // Construct a TimeSpec instance initialized with 0's
    pub fn empty() -> Self {
        TimeSpec {
            tv_sec: 0,
            tv_nsec: 0,
        }
    }
}
