//! System call

use arch::interrupt::TrapFrame;
use util;
use core::{slice, str};
use alloc::sync::Arc;
use spin::Mutex;
use alloc::vec::Vec;
use alloc::string::String;

/// System call dispatcher
pub fn syscall(id: usize, args: [usize; 6], tf: &mut TrapFrame) -> i32 {
    let ret = match id {
        // file
        100 => sys_open(args[0] as *const u8, args[1]),
        101 => sys_close(args[0]),
        102 => sys_read(args[0], args[1] as *mut u8, args[2]),
        103 => sys_write(args[0], args[1] as *const u8, args[2]),
        030 => sys_putc(args[0] as u8 as char),
//        104 => sys_seek(),
        110 => sys_fstat(args[0], args[1] as *mut Stat),
//        111 => sys_fsync(),
//        121 => sys_getcwd(),
        128 => sys_getdirentry(args[0], args[1] as *mut DirEntry),
        130 => sys_dup(args[0], args[1]),

        // process
        001 => sys_exit(args[0] as i32),
        002 => sys_fork(tf),
        003 => sys_wait(args[0], args[1] as *mut i32),
        004 => sys_exec(args[0] as *const u8, args[1] as usize, args[2] as *const *const u8, tf),
//        005 => sys_clone(),
        010 => sys_yield(),
        011 => sys_sleep(args[0]),
        012 => sys_kill(args[0]),
        017 => sys_get_time(),
        018 => sys_getpid(),
        255 => sys_lab6_set_priority(args[0]),

        // memory
//        020 => sys_mmap(),
//        021 => sys_munmap(),
//        022 => sys_shmem(),
//        031 => sys_pgdir(),

        _ => {
            error!("unknown syscall id: {:#x?}, args: {:x?}", id, args);
            ::trap::error(tf);
        }
    };
    match ret {
        Ok(code) => code,
        Err(_) => -1,
    }
}

fn sys_read(fd: usize, base: *mut u8, len: usize) -> SysResult {
    unimplemented!()
}

fn sys_write(fd: usize, base: *const u8, len: usize) -> SysResult {
    unimplemented!()
}

fn sys_open(path: *const u8, flags: usize) -> SysResult {
    unimplemented!()
}

fn sys_close(fd: usize) -> SysResult {
    unimplemented!()
}

fn sys_fstat(fd: usize, stat_ptr: *mut Stat) -> SysResult {
    unimplemented!()
}

/// entry_id = dentry.offset / 256
/// dentry.name = entry_name
/// dentry.offset += 256
fn sys_getdirentry(fd: usize, dentry_ptr: *mut DirEntry) -> SysResult {
    unimplemented!()
}

fn sys_dup(fd1: usize, fd2: usize) -> SysResult {
    unimplemented!()
}

/// Fork the current process. Return the child's PID.
fn sys_fork(tf: &TrapFrame) -> SysResult {
    unimplemented!()
}

/// Wait the process exit.
/// Return the PID. Store exit code to `code` if it's not null.
fn sys_wait(pid: usize, code: *mut i32) -> SysResult {
    unimplemented!()
}

fn sys_exec(name: *const u8, argc: usize, argv: *const *const u8, tf: &mut TrapFrame) -> SysResult {
    unimplemented!()
}

fn sys_yield() -> SysResult {
    unimplemented!()
}

/// Kill the process
fn sys_kill(pid: usize) -> SysResult {
    unimplemented!()
}

/// Get the current process id
fn sys_getpid() -> SysResult {
    unimplemented!()
}

/// Exit the current process
fn sys_exit(exit_code: i32) -> SysResult {
    unimplemented!()
}

fn sys_sleep(time: usize) -> SysResult {
    unimplemented!()
}

fn sys_get_time() -> SysResult {
    unsafe { Ok(::trap::TICK as i32) }
}

fn sys_lab6_set_priority(priority: usize) -> SysResult {
    unimplemented!()
}

fn sys_putc(c: char) -> SysResult {
    unimplemented!()
}


pub type SysResult = Result<i32, SysError>;

#[repr(i32)]
pub enum SysError {
    VfsError,
    InvalidFile,
    InvalidArgument,
}

impl From<()> for SysError {
    fn from(_: ()) -> Self {
        SysError::VfsError
    }
}

bitflags! {
    struct VfsFlags: usize {
        // WARNING: different from origin uCore
        const READABLE = 1 << 0;
        const WRITABLE = 1 << 1;
        /// create file if it does not exist
        const CREATE = 1 << 2;
        /// error if O_CREAT and the file exists
        const EXCLUSIVE = 1 << 3;
        /// truncate file upon open
        const TRUNCATE = 1 << 4;
        /// append on each write
        const APPEND = 1 << 5;
    }
}

impl VfsFlags {
    fn from_ucore_flags(f: usize) -> Self {
        assert_ne!(f & 0b11, 0b11);
        Self::from_bits_truncate(f + 1)
    }
}

#[repr(C)]
struct DirEntry {
    offset: u32,
    name: [u8; 256],
}

impl DirEntry {
    fn check(&self) -> bool {
        self.offset % 256 == 0
    }
    fn entry_id(&self) -> usize {
        (self.offset / 256) as usize
    }
    fn set_name(&mut self, name: &str) {
        self.name[..name.len()].copy_from_slice(name.as_bytes());
        self.name[name.len()] = 0;
        self.offset += 256;
    }
}

#[repr(C)]
struct Stat {
    /// protection mode and file type
    mode: StatMode,
    /// number of hard links
    nlinks: u32,
    /// number of blocks file is using
    blocks: u32,
    /// file size (bytes)
    size: u32,
}

bitflags! {
    struct StatMode: u32 {
        const NULL  = 0;
        /// ordinary regular file
        const FILE  = 0o10000;
        /// directory
        const DIR   = 0o20000;
        /// symbolic link
        const LINK  = 0o30000;
        /// character device
        const CHAR  = 0o40000;
        /// block device
        const BLOCK = 0o50000;
    }
}
