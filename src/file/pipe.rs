//! The purpose of this file is to deal with the implementation of pipes
//! This includes both normal pipes and named pipes.
//!
//! We should be able to treat stdin, stdout, and stderr as pipes
//! that get created for each process.
//!
//! This file provides the following public functionality:
//!
//! struct pipe {
//!
//! }
//!

/*
A pipe is a special file that acts as a stream.
There are two ends to each pipe:
    - input/sender
    - output/receiver
The pipe should be buffered by newlines/flushes

The current implementation will use a ring buffer to store information
*/

/*
Required functionality:
    Writing:
        if n <= PIPE_BUF
            All n bytes are written atomically; writing may block if there is not room for n bytes to be written immediately
        if n > PIPE_BUF
            the write blocks until n bytes have been written

    Reading:
        if one end of the pipe is closed, 0 is returned, indicating EOF
        depending on if the pipe is NONBLOCKING, either read 0 bytes or block until data is written
*/

use ringbuffer::{AllocRingBuffer, RingBuffer};
use crate::error::error::Error;

// pipe struct definition
pub struct Pipe {
    nonblocking: i32, // whether or not the pipe is nonblocking
    buffer: AllocRingBuffer<u8>, // buffer to contain the data
}

impl Pipe {
    /// constructor
    pub fn new(size: usize) -> Self {
        // make sure the capacity is not 0 (or else it will panic)
        if size == 0 {
            println!("Pipe with capacity of 0 is not allowed!");
            let size = 1;
        }
        // return an instance of a Pipe with an allocated buffer
        return Pipe {
            nonblocking: 0, // TODO: implement nonblocking pipes
            buffer: AllocRingBuffer::new(size),
        };
    }

    /// attempt to write from a buffer to the pipe
    /// return the number of bytes written or -1 on error
    pub fn write(&mut self, buffer: &[u8], bytes: i32) -> i32 {
        let mut written = 0;
        // while there are still bytes to write 
        while written < bytes {
            // if there is no space left in the buffer, wait until there is space
            if self.buffer.is_full() {
                
            }

            // there is now enough space to write
            
        }
            
        return -1;
    }

    /// attempt to read from the pipe into a buffer
    /// return the number of bytes read or -1 on error
    pub fn read(&self, buffer: &[u8], bytes: i32) -> i32 {
        return -1;
    }
    
}



