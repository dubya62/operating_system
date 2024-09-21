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
pub struct TimeSpec {
    pub tv_sec: i32,
    pub tv_nsec: i32,
}

impl TimeSpec {
    // Construct a TimeSpec instance with known time
    pub fn new(tv_sec: i32, tv_nsec: i32) -> Self {
        return TimeSpec {
            tv_sec: tv_sec,
            tv_nsec: tv_nsec,
        };
    }

    // Construct a TimeSpec instance initialized with 0's
    pub fn empty() -> Self {
        return TimeSpec{
            tv_sec: 0,
            tv_nsec: 0,
        }
    }
}





