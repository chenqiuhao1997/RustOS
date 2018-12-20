use spin::Mutex;
use consts::{MAX_CPU_NUM, MAX_PROCESS_NUM};
use arch::cpu;
use alloc::{boxed::Box, sync::Arc, vec::Vec};
use core::sync::atomic::*;
use arch::interrupt;

pub fn init() {
    // NOTE: max_time_slice <= 5 to ensure 'priority' test pass

    extern fn idle(_arg: usize) -> ! {
        loop { cpu::halt(); }
    }
    interrupt::restore_now(2);

    ::kmain();
}
