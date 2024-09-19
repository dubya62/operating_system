////////////////////////////////////////////////
// This is the main C file that contains
// system calls and their usage information
//
// Currently, the plan is to use the same system calls
// in the same order as the 64 bit linux system calls
//
// For now, I am not defining any of the stddef types since 
// the C code needs to be translated anyway.
//
// I will use size_t = unsigned int
// and ssize_t = int
//
// TODO: Translate to rust
////////////////////////////////////////////////

//==============================================
// SYSCALL 0x000 (000)
// ssize_t sys_read(unsigned int fd, char* buf, size_t count);
// TODO
//==============================================
int sys_read(int fd, char* buf, unsigned int count);

//==============================================
// SYSCALL 0x001 (001)
// ssize_t sys_write(unsigned int fd, const char* buf, size_t count);
// TODO
//==============================================
int sys_write(unsigned int fd, const char* buf, unsigned int count);

//==============================================
// SYSCALL 0x002 (002)
// int sys_open(const char* filename, int flags, int mode);
// TODO
//==============================================
int sys_open(const char* filename, int flags, int mode);

//==============================================
// SYSCALL 0x003 (003)
// int sys_close(unsigned int fd);
// TODO
//==============================================
int sys_close(unsigned int fd);

//==============================================
// SYSCALL 0x004 (004)
// int sys_stat(const char* pathname, struct stat* statbuf);
// TODO
//==============================================
int sys_stat(const char* pathname, struct stat* statbuf);

//==============================================
// SYSCALL 0x005 (005)
// int sys_fstat(int fd, struct stat* statbuf);
// TODO
//==============================================
int sys_fstat(int fd, struct stat* statbuf);

//==============================================
// SYSCALL 0x006 (006)
// int sys_lstat(const char* pathname, struct stat* statbuf);
// TODO
//==============================================
int sys_lstat(const char* pathname, struct stat* statbuf);

//==============================================
// SYSCALL 0x007 (007)
// int sys_poll(struct poll_fd *ufds, unsigned int nfds, long timeout_msecs);
// TODO
//==============================================
int sys_poll(struct poll_fd *ufds, unsigned int nfds, long timeout_msecs);

//==============================================
// SYSCALL 0x008 (008)
// int sys_lseek(unsigned int fd, off_t offset, unsigned int origin);
// TODO
//==============================================
int sys_lseek(unsigned int fd, int offset, unsigned int origin);

//==============================================
// SYSCALL 0x009 (009)
// int sys_mmap(unsigned long addr, unsigned long len);
// TODO
//==============================================
int sys_lseek(unsigned int fd, int offset, unsigned int origin);




