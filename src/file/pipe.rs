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

// The input end of a pipe.
// Senders will write data through the PipeInput
struct PipeInput {}

// The output end of a pipe.
// Receivers will receive data from this output
struct PipeOutput {}

// pipe struct definition
pub struct Pipe {
    input: PipeInput,
    output: PipeOutput,
    buffer: AllocRingBuffer<u8>, // buffer to contain the data
}
