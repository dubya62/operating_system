# How the Linux Filesystem works
-   Reference: 
    - https://www.bogotobogo.com/Linux/linux_File_Types.php

# Different types of files in Linux
-   Regular files ('-')
-   Directory files ('d')
-   Special files
    - block file ('b')
    - character device file ('c')
    - named pipe/pipe file ('p')
    - symbolic link file ('l')
    - socket file ('s')

## Regular Files
## Directory Files
## Block Files
-   These files are hardware files, and most of them are present in /dev. 
-   They are created either by fdisk command or by parititioning

## Character Device Files
-   Provide a serial stream of input or output.
-   Our terminals are classic example for this type of files.

## Named Pipe/Pipe Files
## Symbolic Link Files
-   These are linked files to other files.
-   They are either Directory/Regular File.
-   The inode number for this file and its parent files are smae.
-   There are two types of link files available in Linux/Unix: soft and hard link.

## Socket Files
-   Used to pass information between applications.

# Creating stdin, stdout, and stderr
-   Reference: 
    - https://www.howtogeek.com/435903/what-are-stdin-stdout-and-stderr-on-linux/

## Streams
-   The stdin, stdout, and stderr of each process is handled as a stream.
-   The shell that launches the command determines the other ends of each stream.

## My Implementation Idea So Far
-   Have a main stdin stream that the kernel accepts from the keyboard.
-   The kernel redirects its stdin as buffered input to the stdin of whatever process is currently focused (i.e. a shell)
-   stdout and stderr could probably use the same buffered stream implementation if desired.


