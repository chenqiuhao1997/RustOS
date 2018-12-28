use spin::Mutex;
pub use self::context::ContextImpl;
pub use ucore_process::*;
use consts::{MAX_CPU_NUM, MAX_PROCESS_NUM};
use arch::cpu;
use alloc::{boxed::Box, sync::Arc, vec::Vec};
use core::sync::atomic::*;


fn start<F, T>(f: F)
    where
        F: Send + 'static + FnOnce() -> T,
        T: Send + 'static,
{
    trace!("spawn:");

    // 注意到下面的问题：
    // Processor只能从入口地址entry+参数arg创建新线程
    // 而我们现在需要让它执行一个未知类型的（闭包）函数f

    // 首先把函数本体（代码数据）置于堆空间中
    let f = Box::into_raw(Box::new(f));

    // 定义一个静态函数作为新线程的入口点
    // 其参数是函数f在堆上的指针
    // 这样我们就把函数f传到了一个静态函数内部
    //
    // 注意到它具有泛型参数，因此对每一次spawn调用，
    // 由于F类型是独特的，因此都会生成一个新的kernel_thread_entry
    extern fn kernel_thread_entry<F, T>(f: usize) -> !
        where
            F: Send + 'static + FnOnce() -> T,
            T: Send + 'static,
    {
        // 在静态函数内部：
        // 根据传进来的指针，恢复f
        let f = unsafe { Box::from_raw(f as *mut F) };
        // 调用f，并将其返回值也放在堆上
        let ret = Box::new(f());
        // 清理本地线程存储
        //   unsafe { LocalKey::<usize>::get_map() }.clear();
        // 让Processor退出当前线程
        // 把f返回值在堆上的指针，以线程返回码的形式传递出去
        let exit_code = Box::into_raw(ret) as usize;
        processor().manager().exit(processor().pid(), exit_code);
        processor().yield_now();
        // 再也不会被调度回来了
        unreachable!()
    }

    // 在Processor中创建新的线程
    let context = new_kernel_context(kernel_thread_entry::<F, T>, f as usize);
    let pid = processor().manager().add(context, processor().pid());
}

fn init_proc() {
    println!("init proc start correctly");
}

fn idle_proc() {
    println!("idle proc start correctly");

    start(init_proc);
    processor().yield_now();
    println!("init proc exited correctly");
}

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
    start(idle_proc);

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