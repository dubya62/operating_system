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

// pipe struct definition
pub struct Pipe {
    w_lock: i32, // whether or not writing process is suspended
    r_lock: i32, // whether or not reading process is suspended
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
            w_lock: 0,
            r_lock: 0,
            nonblocking: 0, // TODO: implement nonblocking pipes
            buffer: AllocRingBuffer::new(size),
        };
    }

    /// attempt to write from a buffer to the pipe
    /// return the number of bytes written or -1 on error
    pub fn write(&mut self, buffer: &[u8], bytes: usize) -> usize {
        let mut written: usize = 0;
        // while there are still bytes to write 
        while written < bytes {
            // if there is no space left in the buffer, wait until there is space
            if self.buffer.is_full() {
                println!("Write into full pipe blocked.");
                // TODO: suspend the process until a read unsuspends it
                self.w_lock = 1;
                loop{}
            }

            // there is now enough space to write something

            // figure out how much buffer space we have
            let mut writable: usize = self.buffer.capacity() - self.buffer.len();

            // figure out how much still needs to be written
            let remaining: usize = bytes - written;

            // if you are able to write the rest of the buffer,
            // go ahead and do it
            if remaining < writable {
                writable = remaining;
            }

            // write the max number of writable bytes
            for i in 0..writable {
                self.buffer.push(buffer[written]);
                written += 1;
            }
        }

        // if at least 1 byte was written, 
        // the pipe is now allowed to be readable
        if written > 0 {
            self.r_lock = 0;
        }
            
        // return how many bytes were written
        return written;
    }

    
    /// attempt to read from the pipe into a buffer
    /// return the number of bytes read or -1 on error
    pub fn read(&mut self, buffer: &mut [u8], bytes: usize) -> usize {
        // read at most 'bytes' bytes from the pipe into buffer. if the pipe is empty,
        // block until something is written to it
        let bytes_read: usize = 0;

        // if the pipe is empty, block until the writer writes something
        if self.buffer.is_empty() {
            println!("Read from empty pipe blocked.");
            self.r_lock = 1;
            // TODO: suspend the process instead of forever looping
            loop{}
        }

        // if we are reading fewer bytes than are in the buffer, 
        // just read everything available in the buffer
        let available: usize = self.buffer.len(); // try to avoid non-atomic problem
        let mut reading: usize = available;
        if bytes < available {
            reading = bytes;
        } 

        // read all available bytes
        for i in 0..reading {
            // FIXME: handle EOF by closing the pipe
            buffer[i] = self.buffer
                .dequeue()
                .expect("Somehow the pipe's buffer was empty even though it's not?");
        }

        // remove the writing lock
        // should be safe to do even if 0 bytes are read
        // since that would mean the pipe is empty
        self.w_lock = 0;

        // return the number of bytes read
        return bytes_read;
    }

}



