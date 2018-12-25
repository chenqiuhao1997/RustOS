pub use arch::paging::*;
use bit_allocator::{BitAlloc, BitAlloc4K, BitAlloc64K};
use consts::MEMORY_OFFSET;
use spin;
use super::HEAP_ALLOCATOR;
use ucore_memory::{*, paging::PageTable};
pub use ucore_memory::memory_set::{InactivePageTable};
use sync::{SpinNoIrqLock, SpinNoIrq, MutexGuard};
use alloc::vec::Vec;
use alloc::sync::Arc;
use alloc::boxed::Box;
use core::slice;

// RISCV only have 8M memory
#[cfg(target_arch = "riscv32")]
pub type FrameAlloc = BitAlloc4K;


lazy_static! {
    pub static ref FRAME_ALLOCATOR: SpinNoIrqLock<FrameAlloc> = SpinNoIrqLock::new(FrameAlloc::default());
}

lazy_static! {
    static ref ACTIVE_TABLE: SpinNoIrqLock<ActivePageTable> = SpinNoIrqLock::new(unsafe {
        ActivePageTable::new()
    });
}

/// The only way to get active page table
pub fn active_table() -> MutexGuard<'static, ActivePageTable, SpinNoIrq> {
    ACTIVE_TABLE.lock()
}
/*
* @brief:
*   allocate a free physical frame, if no free frame, then swap out one page and reture mapped frame as the free one
* @retval:
*   the physical address for the allocated frame
*/
pub fn alloc_frame() -> Option<usize> {
    // get the real address of the alloc frame
    let ret = FRAME_ALLOCATOR.lock().alloc().map(|id| id * PAGE_SIZE + MEMORY_OFFSET);
    trace!("Allocate frame: {:x?}", ret);
    ret
}

pub fn dealloc_frame(target: usize) {
    trace!("Deallocate frame: {:x}", target);
    FRAME_ALLOCATOR.lock().dealloc((target - MEMORY_OFFSET) / PAGE_SIZE);
}

pub struct KernelStack(usize);
const STACK_SIZE: usize = 0x8000;

impl KernelStack {
    pub fn new() -> Self {
        use alloc::alloc::{alloc, Layout};
        let bottom = unsafe{ alloc(Layout::from_size_align(STACK_SIZE, STACK_SIZE).unwrap()) } as usize;
        KernelStack(bottom)
    }
    pub fn top(&self) -> usize {
        self.0 + STACK_SIZE
    }
}

impl Drop for KernelStack {
    fn drop(&mut self) {
        use alloc::alloc::{dealloc, Layout};
        unsafe{ dealloc(self.0 as _, Layout::from_size_align(STACK_SIZE, STACK_SIZE).unwrap()); }
    }
}


/*
* @param:
*   addr: the virtual address of the page fault
* @brief:
*   handle page fault
* @retval:
*   Return true to continue, false to halt
*/
pub fn page_fault_handler(addr: usize) -> bool {
    //since we don't have any process now, we don't need to handle the page fault
    false
}

pub fn init_heap() {
    use consts::KERNEL_HEAP_SIZE;
    static mut HEAP: [u8; KERNEL_HEAP_SIZE] = [0; KERNEL_HEAP_SIZE];
    unsafe { HEAP_ALLOCATOR.lock().init(HEAP.as_ptr() as usize, KERNEL_HEAP_SIZE); }
    info!("heap init end");
}

pub mod test{
    pub fn lab_test(){  
        println!("memory lab_test finished");
    } 
    
}

