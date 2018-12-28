use spin::Mutex;
pub use self::context::ContextImpl;
pub use ucore_process::*;
use consts::{MAX_CPU_NUM, MAX_PROCESS_NUM};
use arch::cpu;
use alloc::{boxed::Box, sync::Arc, vec::Vec};
use core::sync::atomic::*;

pub mod context;
pub fn init() {
    // NOTE: max_time_slice <= 5 to ensure 'priority' test pass
    let manager = Arc::new(ProcessManager::new(MAX_PROCESS_NUM));

    unsafe {
        for cpu_id in 0..MAX_CPU_NUM {
            PROCESSORS[cpu_id].init(cpu_id, ContextImpl::new_init(), manager.clone());
        }
    }
    extern fn idle(_arg: usize) -> ! {
        loop { cpu::halt(); processor().yield_now();}
    }
    manager.add(ContextImpl::new_kernel(idle, 0), 0);
    
    ::shell::run_user_shell();

    info!("process init end");
}

static PROCESSORS: [Processor; MAX_CPU_NUM] = [Processor::new(), Processor::new(), Processor::new(), Processor::new(), Processor::new(), Processor::new(), Processor::new(), Processor::new()];

/// Get current thread struct
pub fn process() -> &'static mut ContextImpl {
    use core::mem::transmute;
    let (process, _): (&mut ContextImpl, *const ()) = unsafe {
        transmute(processor().context())
    };
    process
}


// Implement dependencies for std::thread

#[no_mangle]
pub fn processor() -> &'static Processor {
    &PROCESSORS[cpu::id()]
}

#[no_mangle]
pub fn new_kernel_context(entry: extern fn(usize) -> !, arg: usize) -> Box<Context> {
    ContextImpl::new_kernel(entry, arg)
}