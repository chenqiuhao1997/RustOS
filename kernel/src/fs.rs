use simple_filesystem::*;
use alloc::{boxed::Box, sync::Arc, string::String, collections::VecDeque, vec::Vec};
use sync::SpinNoIrqLock as Mutex;
use core::any::Any;
use core::slice;
use process::*;

use ::memory::{InactivePageTable0};
use memory::MemorySet;
//use process::context::memory_set_map_swappable;

// Hard link user program
#[cfg(target_arch = "riscv32")]
global_asm!(r#"
    .section .rodata
    .align 12
    .global _user_img_start
    .global _user_img_end
_user_img_start:
    .incbin "../user/user-riscv.img"
_user_img_end:
"#);

lazy_static! {
    pub static ref ROOT_INODE: Arc<INode> = {
        #[cfg(target_arch = "riscv32")]
        let device = {
            extern {
                fn _user_img_start();
                fn _user_img_end();
            }
            Box::new(unsafe { MemBuf::new(_user_img_start, _user_img_end) })
        };

        let sfs = SimpleFileSystem::open(device).expect("failed to open SFS");
        sfs.root_inode()
    };
}

struct MemBuf(&'static [u8]);

impl MemBuf {
    unsafe fn new(begin: unsafe extern fn(), end: unsafe extern fn()) -> Self {
        use core::slice;
        MemBuf(slice::from_raw_parts(begin as *const u8, end as usize - begin as usize))
    }
}

impl Device for MemBuf {
    fn read_at(&mut self, offset: usize, buf: &mut [u8]) -> Option<usize> {
        let slice = self.0;
        let len = buf.len().min(slice.len() - offset);
        buf[..len].copy_from_slice(&slice[offset..offset + len]);
        Some(len)
    }
    fn write_at(&mut self, offset: usize, buf: &[u8]) -> Option<usize> {
        None
    }
}


#[derive(Default)]
pub struct Stdin {
    buf: Mutex<VecDeque<char>>,
}

impl Stdin {
    pub fn push(&self, c: char) {
        self.buf.lock().push_back(c);
    }
    pub fn pop(&self) -> char {
        loop {
            let ret = self.buf.lock().pop_front();
            match ret {
                Some(c) => return c,
                None => processor().yield_now(),
            }
        }
    }
}

#[derive(Default)]
pub struct Stdout;

lazy_static! {
    pub static ref STDIN: Arc<Stdin> = Arc::new(Stdin::default());
    pub static ref STDOUT: Arc<Stdout> = Arc::new(Stdout::default());
}

// TODO: better way to provide default impl?
macro_rules! impl_inode {
    () => {
        fn info(&self) -> Result<FileInfo> { unimplemented!() }
        fn sync(&self) -> Result<()> { unimplemented!() }
        fn resize(&self, len: usize) -> Result<()> { unimplemented!() }
        fn create(&self, name: &str, type_: FileType) -> Result<Arc<INode>> { unimplemented!() }
        fn unlink(&self, name: &str) -> Result<()> { unimplemented!() }
        fn link(&self, name: &str, other: &Arc<INode>) -> Result<()> { unimplemented!() }
        fn rename(&self, old_name: &str, new_name: &str) -> Result<()> { unimplemented!() }
        fn move_(&self, old_name: &str, target: &Arc<INode>, new_name: &str) -> Result<()> { unimplemented!() }
        fn find(&self, name: &str) -> Result<Arc<INode>> { unimplemented!() }
        fn get_entry(&self, id: usize) -> Result<String> { unimplemented!() }
        fn fs(&self) -> Arc<FileSystem> { unimplemented!() }
        fn as_any_ref(&self) -> &Any { self }
    };
}

impl INode for Stdin {
    fn read_at(&self, offset: usize, buf: &mut [u8]) -> Result<usize> {
        buf[0] = self.pop() as u8;
        Ok(1)
    }
    fn write_at(&self, offset: usize, buf: &[u8]) -> Result<usize> { unimplemented!() }
    impl_inode!();
}

impl INode for Stdout {
    fn read_at(&self, offset: usize, buf: &mut [u8]) -> Result<usize> { unimplemented!() }
    fn write_at(&self, offset: usize, buf: &[u8]) -> Result<usize> {
        use core::str;
        let s = str::from_utf8(buf).map_err(|_| ())?;
        print!("{}", s);
        Ok(buf.len())
    }
    impl_inode!();
}

pub trait INodeExt {
    fn read_as_vec(&self) -> Result<Vec<u8>>;
}

impl INodeExt for INode {
    fn read_as_vec(&self) -> Result<Vec<u8>> {
        let size = self.info()?.size;
        let mut buf = Vec::with_capacity(size);
        unsafe { buf.set_len(size); }
        self.read_at(0, buf.as_mut_slice())?;
        Ok(buf)
    }
}