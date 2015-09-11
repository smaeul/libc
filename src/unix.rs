extern {
    pub fn socket(domain: ::c_int, ty: ::c_int, protocol: ::c_int) -> ::c_int;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
               link_name = "connect$UNIX2003")]
    pub fn connect(socket: ::c_int, address: *const ::sockaddr,
                   len: ::socklen_t) -> ::c_int;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
               link_name = "bind$UNIX2003")]
    pub fn bind(socket: ::c_int, address: *const ::sockaddr,
                address_len: ::socklen_t) -> ::c_int;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
               link_name = "listen$UNIX2003")]
    pub fn listen(socket: ::c_int, backlog: ::c_int) -> ::c_int;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
               link_name = "accept$UNIX2003")]
    pub fn accept(socket: ::c_int, address: *mut ::sockaddr,
                  address_len: *mut ::socklen_t) -> ::c_int;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
               link_name = "getpeername$UNIX2003")]
    pub fn getpeername(socket: ::c_int, address: *mut ::sockaddr,
                       address_len: *mut ::socklen_t) -> ::c_int;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
               link_name = "getsockname$UNIX2003")]
    pub fn getsockname(socket: ::c_int, address: *mut ::sockaddr,
                       address_len: *mut ::socklen_t) -> ::c_int;
    pub fn setsockopt(socket: ::c_int, level: ::c_int, name: ::c_int,
                      value: *const ::c_void,
                      option_len: ::socklen_t) -> ::c_int;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
               link_name = "recv$UNIX2003")]
    pub fn recv(socket: ::c_int, buf: *mut ::c_void, len: ::size_t,
                flags: ::c_int) -> ::ssize_t;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
               link_name = "send$UNIX2003")]
    pub fn send(socket: ::c_int, buf: *const ::c_void, len: ::size_t,
                flags: ::c_int) -> ::ssize_t;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
               link_name = "recvfrom$UNIX2003")]
    pub fn recvfrom(socket: ::c_int, buf: *mut ::c_void, len: ::size_t,
                    flags: ::c_int, addr: *mut ::sockaddr,
                    addrlen: *mut ::socklen_t) -> ::ssize_t;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
               link_name = "sendto$UNIX2003")]
    pub fn sendto(socket: ::c_int, buf: *const ::c_void, len: ::size_t,
                  flags: ::c_int, addr: *const ::sockaddr,
                  addrlen: ::socklen_t) -> ::ssize_t;
    pub fn getifaddrs(ifap: *mut *mut ::ifaddrs) -> ::c_int;
    pub fn freeifaddrs(ifa: *mut ::ifaddrs);
    pub fn shutdown(socket: ::c_int, how: ::c_int) -> ::c_int;

    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
               link_name = "chmod$UNIX2003")]
    pub fn chmod(path: *const ::c_char, mode: ::mode_t) -> ::c_int;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
               link_name = "fchmod$UNIX2003")]
    pub fn fchmod(fd: ::c_int, mode: ::mode_t) -> ::c_int;

    #[cfg_attr(target_os = "macos", link_name = "fstat$INODE64")]
    pub fn fstat(fildes: ::c_int, buf: *mut ::stat) -> ::c_int;

    pub fn mkdir(path: *const ::c_char, mode: ::mode_t) -> ::c_int;
    pub fn mkfifo(path: *const ::c_char, mode: ::mode_t) -> ::c_int;

    #[cfg_attr(target_os = "macos", link_name = "::stat$INODE64")]
    pub fn stat(path: *const ::c_char, buf: *mut ::stat) -> ::c_int;

    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
               link_name = "popen$UNIX2003")]
    pub fn popen(command: *const ::c_char,
                 mode: *const ::c_char) -> *mut ::FILE;
    pub fn pclose(stream: *mut ::FILE) -> ::c_int;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
               link_name = "fdopen$UNIX2003")]
    pub fn fdopen(fd: ::c_int, mode: *const ::c_char) -> *mut ::FILE;
    pub fn fileno(stream: *mut ::FILE) -> ::c_int;

    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
               link_name = "open$UNIX2003")]
    pub fn open(path: *const ::c_char, oflag: ::c_int, ...) -> ::c_int;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
               link_name = "creat$UNIX2003")]
    pub fn creat(path: *const ::c_char, mode: ::mode_t) -> ::c_int;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
               link_name = "fcntl$UNIX2003")]
    pub fn fcntl(fd: ::c_int, cmd: ::c_int, ...) -> ::c_int;

    #[cfg_attr(all(target_os = "macos", target_arch = "x86_64"),
               link_name = "opendir$INODE64")]
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
               link_name = "opendir$INODE64$UNIX2003")]
    pub fn opendir(dirname: *const ::c_char) -> *mut ::DIR;
    #[cfg_attr(target_os = "macos", link_name = "readdir_r$INODE64")]
    pub fn readdir_r(dirp: *mut ::DIR, entry: *mut ::dirent,
                      result: *mut *mut ::dirent) -> ::c_int;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
               link_name = "closedir$UNIX2003")]
    pub fn closedir(dirp: *mut ::DIR) -> ::c_int;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86_64"),
               link_name = "rewinddir$INODE64")]
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
               link_name = "rewinddir$INODE64$UNIX2003")]
    pub fn rewinddir(dirp: *mut ::DIR);
    #[cfg_attr(all(target_os = "macos", target_arch = "x86_64"),
               link_name = "seekdir$INODE64")]
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
               link_name = "seekdir$INODE64$UNIX2003")]
    pub fn seekdir(dirp: *mut ::DIR, loc: ::c_long);
    #[cfg_attr(all(target_os = "macos", target_arch = "x86_64"),
               link_name = "telldir$INODE64")]
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
               link_name = "telldir$INODE64$UNIX2003")]
    pub fn telldir(dirp: *mut ::DIR) -> ::c_long;

    pub fn access(path: *const ::c_char, amode: ::c_int) -> ::c_int;
    pub fn alarm(seconds: ::c_uint) -> ::c_uint;
    pub fn chdir(dir: *const ::c_char) -> ::c_int;
    pub fn chown(path: *const ::c_char, uid: ::uid_t,
                 gid: ::gid_t) -> ::c_int;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
               link_name = "close$UNIX2003")]
    pub fn close(fd: ::c_int) -> ::c_int;
    pub fn dup(fd: ::c_int) -> ::c_int;
    pub fn dup2(src: ::c_int, dst: ::c_int) -> ::c_int;
    pub fn execv(prog: *const ::c_char,
                 argv: *const *const ::c_char) -> ::c_int;
    pub fn execve(prog: *const ::c_char, argv: *const *const ::c_char,
                  envp: *const *const ::c_char)
                  -> ::c_int;
    pub fn execvp(c: *const ::c_char,
                  argv: *const *const ::c_char) -> ::c_int;
    pub fn fork() -> ::pid_t;
    pub fn fpathconf(filedes: ::c_int, name: ::c_int) -> ::c_long;
    pub fn getcwd(buf: *mut ::c_char, size: ::size_t) -> *mut ::c_char;
    pub fn getegid() -> ::gid_t;
    pub fn geteuid() -> ::uid_t;
    pub fn getgid() -> ::gid_t;
    pub fn getgroups(ngroups_max: ::c_int, groups: *mut ::gid_t)
                     -> ::c_int;
    pub fn getlogin() -> *mut ::c_char;
    // GNU getopt(3) modifies its arguments despite the
    // char * const [] prototype; see the manpage.
    pub fn getopt(argc: ::c_int, argv: *mut *mut ::c_char,
                  optstr: *const ::c_char) -> ::c_int;
    pub fn getpgrp() -> ::pid_t;
    pub fn getpid() -> ::pid_t;
    pub fn getppid() -> ::pid_t;
    pub fn getuid() -> ::uid_t;
    pub fn getsid(pid: ::pid_t) -> ::pid_t;
    pub fn isatty(fd: ::c_int) -> ::c_int;
    pub fn link(src: *const ::c_char, dst: *const ::c_char) -> ::c_int;
    pub fn lseek(fd: ::c_int, offset: ::off_t, whence: ::c_int)
                 -> ::off_t;
    pub fn pathconf(path: *const ::c_char, name: ::c_int) -> ::c_long;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
               link_name = "pause$UNIX2003")]
    pub fn pause() -> ::c_int;
    pub fn pipe(fds: *mut ::c_int) -> ::c_int;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
               link_name = "read$UNIX2003")]
    pub fn read(fd: ::c_int, buf: *mut ::c_void, count: ::size_t)
                -> ::ssize_t;
    pub fn rmdir(path: *const ::c_char) -> ::c_int;
    pub fn setgid(gid: ::gid_t) -> ::c_int;
    pub fn setpgid(pid: ::pid_t, pgid: ::pid_t) -> ::c_int;
    pub fn setsid() -> ::pid_t;
    pub fn setuid(uid: ::uid_t) -> ::c_int;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
               link_name = "sleep$UNIX2003")]
    pub fn sleep(secs: ::c_uint) -> ::c_uint;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
               link_name = "usleep$UNIX2003")]
    pub fn usleep(secs: ::c_uint) -> ::c_int;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
               link_name = "nanosleep$UNIX2003")]
    pub fn nanosleep(rqtp: *const ::timespec,
                     rmtp: *mut ::timespec) -> ::c_int;
    pub fn sysconf(name: ::c_int) -> ::c_long;
    pub fn tcgetpgrp(fd: ::c_int) -> ::pid_t;
    pub fn ttyname(fd: ::c_int) -> *mut ::c_char;
    pub fn unlink(c: *const ::c_char) -> ::c_int;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
               link_name = "wait$UNIX2003")]
    pub fn wait(status: *mut ::c_int) -> ::pid_t;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
               link_name = "waitpid$UNIX2003")]
    pub fn waitpid(pid: ::pid_t, status: *mut ::c_int, options: ::c_int)
                   -> ::pid_t;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
               link_name = "write$UNIX2003")]
    pub fn write(fd: ::c_int, buf: *const ::c_void, count: ::size_t)
                 -> ::ssize_t;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
               link_name = "pread$UNIX2003")]
    pub fn pread(fd: ::c_int, buf: *mut ::c_void, count: ::size_t,
                 offset: ::off_t) -> ::ssize_t;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
               link_name = "pwrite$UNIX2003")]
    pub fn pwrite(fd: ::c_int, buf: *const ::c_void, count: ::size_t,
                  offset: ::off_t) -> ::ssize_t;
    pub fn utime(file: *const ::c_char, buf: *const ::utimbuf) -> ::c_int;

    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
                   link_name = "kill$UNIX2003")]
    pub fn kill(pid: ::pid_t, sig: ::c_int) -> ::c_int;

    pub fn mlock(addr: *const ::c_void, len: ::size_t) -> ::c_int;
    pub fn munlock(addr: *const ::c_void, len: ::size_t) -> ::c_int;
    pub fn mlockall(flags: ::c_int) -> ::c_int;
    pub fn munlockall() -> ::c_int;

    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
               link_name = "mprotect$UNIX2003")]
    pub fn mprotect(addr: *mut ::c_void, len: ::size_t, prot: ::c_int)
                    -> ::c_int;

    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
               link_name = "msync$UNIX2003")]
    pub fn msync(addr: *mut ::c_void, len: ::size_t, flags: ::c_int)
                 -> ::c_int;
    #[cfg(target_os = "macos")]
    pub fn shm_open(name: *const ::c_char, oflag: ::c_int, ...)
                    -> ::c_int;
    #[cfg(target_os = "linux")]
    pub fn shm_open(name: *const ::c_char, oflag: ::c_int,
                    mode: ::mode_t) -> ::c_int;
    pub fn shm_unlink(name: *const ::c_char) -> ::c_int;

    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
               link_name = "mmap$UNIX2003")]
    pub fn mmap(addr: *mut ::c_void,
                len: ::size_t,
                prot: ::c_int,
                flags: ::c_int,
                fd: ::c_int,
                offset: ::off_t)
                -> *mut ::c_void;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
               link_name = "munmap$UNIX2003")]
    pub fn munmap(addr: *mut ::c_void, len: ::size_t) -> ::c_int;

    pub fn if_nametoindex(ifname: *const ::c_char) -> ::c_uint;

    #[cfg_attr(target_os = "macos", link_name = "lstat$INODE64")]
    pub fn lstat(path: *const ::c_char, buf: *mut ::stat) -> ::c_int;

    pub fn readlink(path: *const ::c_char,
                    buf: *mut ::c_char,
                    bufsz: ::size_t)
                    -> ::ssize_t;

    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
               link_name = "fsync$UNIX2003")]
    pub fn fsync(fd: ::c_int) -> ::c_int;

    #[cfg(any(target_os = "linux", target_os = "android"))]
    pub fn fdatasync(fd: ::c_int) -> ::c_int;

    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
               link_name = "setenv$UNIX2003")]
    pub fn setenv(name: *const ::c_char, val: *const ::c_char,
                  overwrite: ::c_int) -> ::c_int;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
               link_name = "unsetenv$UNIX2003")]
    pub fn unsetenv(name: *const ::c_char) -> ::c_int;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"),
               link_name = "putenv$UNIX2003")]
    pub fn putenv(string: *mut ::c_char) -> ::c_int;

    pub fn symlink(path1: *const ::c_char,
                   path2: *const ::c_char) -> ::c_int;

    pub fn ftruncate(fd: ::c_int, length: ::off_t) -> ::c_int;

    pub fn signal(signum: ::c_int,
                  handler: ::sighandler_t) -> ::sighandler_t;

    pub fn glob(pattern: *const ::c_char,
                flags: ::c_int,
                errfunc: Option<extern "C" fn(epath: *const ::c_char,
                                                  errno: ::c_int) -> ::c_int>,
                pglob: *mut ::glob_t);
    pub fn globfree(pglob: *mut ::glob_t);

    pub fn posix_madvise(addr: *mut ::c_void, len: ::size_t, advice: ::c_int)
                         -> ::c_int;

    pub fn getrlimit(resource: ::c_int, rlim: *mut ::rlimit) -> ::c_int;
    pub fn setrlimit(resource: ::c_int, rlim: *const ::rlimit) -> ::c_int;
    pub fn getrusage(resource: ::c_int, usage: *mut ::rusage) -> ::c_int;

    pub fn sysctl(name: *mut ::c_int,
                  namelen: ::c_int,
                  oldp: *mut ::c_void,
                  oldlenp: *mut ::size_t,
                  newp: *mut ::c_void,
                  newlen: ::size_t)
                  -> ::c_int;
    pub fn getdtablesize() -> ::c_int;
    pub fn madvise(addr: *mut ::c_void, len: ::size_t, advice: ::c_int)
                   -> ::c_int;
    pub fn mincore(addr: *mut ::c_void, len: ::size_t, vec: *mut ::c_uchar)
                   -> ::c_int;
    #[cfg_attr(target_os = "macos", link_name = "realpath$DARWIN_EXTSN")]
    pub fn realpath(pathname: *const ::c_char, resolved: *mut ::c_char)
                    -> *mut ::c_char;

    pub fn ioctl(fd: ::c_int, request: ::c_ulong, ...) -> ::c_int;
    pub fn flock(fd: ::c_int, operation: ::c_int) -> ::c_int;
}
