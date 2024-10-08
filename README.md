# Encrypted Operating System

# Install

```sh
rustup component add rust-src
rustup component add llvm-tools-preview
cargo install bootimage
# (install qemu)
cargo r
```

# Groups
-   Each user has their own group with only them and root in it by default
-   Each group has a key

# File Management
-   Use Linux-like file permissions (user,group,other and read,write,execute)
-   The file is encrypted with the group key.
-   If the owner or a member of the group wishes to read/write/execute the file, simply use the group key
-   If any other person wishes to read/execute the file, call the kernel to decrypt with the group key
-   If any other person wishes to write to the file, call the kernel to encrypt with the group key
-   It would be possible to use the kernel to enforce file permissions, but this setup should make it impossible to boot from another device and read what they should not be able to read (even if they have their own keys)

# Key Management
-   There are three main sections of the disk: (though not necessarily partitions)
    - Bootloader
    - Kernel
    - Rest of Disk (user area)
-   Any user should be able to start the computer up.
    - Users will plug in their thumb drive.
    - Bootloader uses information from the thumb drive (including their key) to decrypt the kernel.
    - The kernel gets decrypted and ran.
    - The kernel jumps to user space as the owner of the key.
-   What keys should exist?
    - login keys (asymmetric key to login with)
    - user keys (symmetric key for each user in /etc/passwd)
    - group keys (symmetric key for each group in /etc/group)
-   How is the kernel protected?
    - Make the kernel read only by encrypting it with an asymmetric key and then throwing the key away.
    - For each login key, make a copy of the kernel decryption key encrypted with the login key (this way any user can decrypt the kernel without having multiple copies of the kernel or sharing login keys)
    - The other key can only decrypt the kernel, but cannot encrypt it back
    - To verify the integrity of the kernel, use a hash along with a predetermined value somewhere in the encrypted block.
    - In this way, it is impossible to modify the kernel.
    - To perform a kernel update, we may have to go through this process again and reencrypt the entire kernel since the key will be thrown away.


# Some more ideas (once kernel is done)
-   Add Ransomware protection (detect when file extensions and headers are changing rapidly)
-   Add email/browser sandboxing
-   Add automatic snapshots that allow recovery in case of malware


# TODO
-   Disk management
-   Memory management
-   System call interface
-   Process management
-   Device drivers
-   Implement standard library
-   Init shell process

# DOING

# DONE
-   Write the bootloader (temporary solution)
-   Write "hello world" program (Hayden)

# VERIFIED


# BACKLOG
-   Disk management
-   Memory management
-   System call interface
-   Process management
-   Device drivers
-   Implement standard library
-   Init shell process



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



