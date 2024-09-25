# Modularization Plan

# Modules
-   Memory (mem)
    - handles memory, paging, and heap
-   Interrupts (interrupt)
    - provide interface for rest of the kernel to handle interrupts
    - kernel should be able to change the interrupt handler for each interrupt dynamically
-   Device (dev)
    - handles device drivers and communication
    - provides a nice interface for the rest of the kernel
-   Filesystem (fs)
    - handles filesystems and mounting
    - provides a nice interface for the rest of the kernel
-   Processes (proc)
    - handle processes, multithreading/multiprocessing, and user space
-   Compatibility (comp)
    - handle the compatibility with windows, linux, and mac programs
-   Init (init)
    - handle system startup and jumping to user space
-   Error (error)
    - graceful error handling
-   Cryptography (crypt)
    - handle cryptographic operations/ functionality
-   Library (lib)
    - provide library functions for user space programs
    - provide syscalls

# File Hierarchy

## mem
## interrupt
## dev
## fs
## proc
## comp
## init
## error
## crypt
## lib


