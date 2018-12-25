use core::{slice, mem};
use memory::{active_table, FRAME_ALLOCATOR, init_heap};
use super::riscv::{addr::*, register::sstatus};
use ucore_memory::PAGE_SIZE;
use alloc::boxed::Box;

/*
* @brief:
*   Init the mermory management module, allow memory access and set up page table and init heap and frame allocator
*/
pub fn init() {
    #[repr(align(4096))]  // align the PageData struct to 4096 bytes
    struct PageData([u8; PAGE_SIZE]);
    static PAGE_TABLE_ROOT: PageData = PageData([0; PAGE_SIZE]);

    unsafe { sstatus::set_sum(); }  // Allow user memory access
    let frame = Frame::of_addr(PhysAddr::new(&PAGE_TABLE_ROOT as *const _ as u32));
    super::paging::setup_page_table(frame); // set up page table
    // initialize heap and Frame allocator
    init_frame_allocator();
    init_heap();
}

pub fn init_other() {
    unsafe {
        sstatus::set_sum();         // Allow user memory access
        asm!("csrw 0x180, $0; sfence.vma" :: "r"(SATP) :: "volatile");
    }
}

/*
* @brief:
*   Init frame allocator, here use a BitAlloc implemented by segment tree.
*/
fn init_frame_allocator() {
    use bit_allocator::BitAlloc;
    use core::ops::Range;
    use consts::{MEMORY_OFFSET, MEMORY_END};

    let mut ba = FRAME_ALLOCATOR.lock();
    ba.insert(to_range(end as usize + PAGE_SIZE, MEMORY_END));
    info!("FrameAllocator init end");

    /*
    * @param:
    *   start: start address
    *   end: end address
    * @brief:
    *   transform the memory address to the page number
    * @retval:
    *   the page number range from start address to end address
    */
    fn to_range(start: usize, end: usize) -> Range<usize> {
        let page_start = (start - MEMORY_OFFSET) / PAGE_SIZE;
        let page_end = (end - MEMORY_OFFSET - 1) / PAGE_SIZE + 1;
        page_start..page_end
    }
}

// First core stores its SATP here.
// Other cores load it later.
static mut SATP: usize = 0;

// Symbols provided by linker script
extern {
    fn stext();
    fn etext();
    fn sdata();
    fn edata();
    fn srodata();
    fn erodata();
    fn sbss();
    fn ebss();
    fn start();
    fn end();
    fn bootstack();
    fn bootstacktop();
}