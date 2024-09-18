# Encrypted Operating System

#Install

```sh
rustup component add rust-src
rustup component add llvm-tools-preview
cargo install bootimage
# (install qemu)
cargo r
```

# TODO
-   Everything else (will break up tasks later)

# DOING
-   Write the bootloader (W)
-   Write "hello world" program (Hayden)

# DONE

# VERIFIED




## An encrypted operating system based on high Security and Accountability
-   User creates an asymmetric key pair and writes it to a thumb drive as the root key (only on installation)
    - thumb drive should be able to hold keys for multiple users, but should be in a way that the root key is not accessible by user programs
-   Both the kernel key and user key should be recreatable from the key on the thumb drive (the root key).
-   Each user should have their own key. (The root user has the master key that has authority to do anything).

## Bootloader:
-   Verify bootloader integrity and use the thumb drive to decrypt a stored symmetric key.
-   use the symmetric key to decrypt the kernel and jump to the kernel code

## Kernel: (kernel should share keys with root since they should have same authority)
-   Once the kernel has finished loading, use the user's asymmetric key to decrypt the user's symmetric key (which allows access to that user's files).
-   Kernel should keep its own copy of the users' symmetric keys for the sake of making priviledged transactions

## Kernel Authority:
-   able to decrypt any file on the (encrypted) filesystem
-   able to change ownership of a file (need to have both users' keys)

## User Authority:
-   escalate privileges to root/kernel 
-   create new files (encrypted with that user's key)
-   read files that are owned by this user

## Program Authority: (Programs should have only the privileges that are required for their operation)
-   Users should be able to take away privileges from a program if desired
    - for example: a user wants to run an iffy application
        - the user should be able to take away the app's ability to read/write files, launch sub-processes, attempt priviledge escalation, use the network card/networking in general, use the disk, use peripherals (usbs, cameras, microphones)
        - all accesses made by the program should also be logged (using signatures) for viewing by the root user

## Signatures:
-   Every action taken on the system requires a signature from that user's private key. (logged in a root owned file)
    - This should cause high accountability

## There should be some way to encrypt the memory so that even buffer overflows are ineffective
- Not only that, but memory forensics should be impossible without the key.

## The os should also focus on privacy.
- Anything done over the network should be heavily protected.
- If the user destroys all copies of the root private key, the entire disk is destroyed (since it is encrypted)
- use spoofed mac addr


## Users (I do not know if this should be a priviledged action or not) should be able to run a program in unencrypted memory



