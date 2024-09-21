////////////////////////////////////////////
//! The purpose of this file is to provide 
//! basic timing functionality, such as sleeps
//! getting the current time, etc.
//!
//! It provides the following public functionality
//!
//! now() -> TimeSpec
//!
////////////////////////////////////////////


use crate::time::timestruct::TimeSpec;

// TODO: make this return the current time
pub fn now() -> TimeSpec {
    return TimeSpec::empty();
}


