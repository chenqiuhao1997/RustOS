use alloc::{boxed::Box, sync::Arc, string::String, collections::VecDeque, vec::Vec};
#[cfg(target_arch = "x86_64")]
use arch::driver::ide;
use core::any::Any;
use core::slice;

use ::memory::{InactivePageTable0};
use memory::MemorySet;
use spin::Mutex;

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
                None => {},
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
