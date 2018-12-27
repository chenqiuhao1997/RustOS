use arch::interrupt::{TrapFrame, Context as ArchContext};
use memory::{MemoryArea, MemoryAttr, MemorySet, KernelStack, swap_table, alloc_frame, active_table, NormalMemoryHandler, SwapMemoryHandler, CowMemoryHandler, SWAP_TABLE, COW_TABLE};
use xmas_elf::{ElfFile, header, program::{Flags, ProgramHeader, Type}};
use core::fmt::{Debug, Error, Formatter};
use ucore_process::Context;
use alloc::{boxed::Box, collections::BTreeMap, vec::Vec, sync::Arc, string::String};
use ucore_memory::{Page, VirtAddr};
use ::memory::{InactivePageTable0};
use ucore_memory::memory_set::*;
use spin::Mutex;



// TODO: avoid pub
pub struct ContextImpl {
    pub arch: ArchContext,
    pub memory_set: Box<MemorySet>,
    pub kstack: KernelStack,
    pub cwd: String,
}

impl Context for ContextImpl {
    unsafe fn switch_to(&mut self, target: &mut Context) {
        use core::mem::transmute;
        let (target, _): (&mut ContextImpl, *const ()) = transmute(target);
        self.arch.switch(&mut target.arch);
    }
}

impl ContextImpl {
    pub unsafe fn new_init() -> Box<Context> {
        Box::new(ContextImpl {
            arch: ArchContext::null(),
            memory_set: Box::new(MemorySet::new()),
            kstack: KernelStack::new(),
            cwd: String::new(),
        })
    }

    pub fn new_kernel(entry: extern fn(usize) -> !, arg: usize) -> Box<Context> {
        let memory_set = Box::new(MemorySet::new());
        let kstack = KernelStack::new();
        Box::new(ContextImpl {
            arch: unsafe { ArchContext::new_kernel_thread(entry, arg, kstack.top(), memory_set.token()) },
            memory_set,
            kstack,
            cwd: String::new(),
        })
    }

    /// Fork
    pub fn fork(&self, tf: &TrapFrame) -> Box<Context> {
        info!("COME into fork!");
        // Clone memory set, make a new page table
        let mut memory_set = self.memory_set.clone();
        info!("finish mmset clone in fork!");
        // add the new memory set to the recorder
        info!("fork! new page table token: {:x?}", memory_set.token());
        //let mmset_ptr = Box::leak(memory_set) as *mut MemorySet as usize;
        //memory_set_record().push_back(mmset_ptr);

        info!("before copy data to temp space");
        
        // Copy data to temp space
        /*
        use alloc::vec::Vec;
        let datas: Vec<Vec<u8>> = memory_set.iter().map(|area| {
            Vec::from(unsafe { area.as_slice() })
        }).collect();

        info!("Finish copy data to temp space.");

        // Temporarily switch to it, in order to copy data
        unsafe {
            memory_set.with(|| {
                for (area, data) in memory_set.iter().zip(datas.iter()) {
                    area.as_slice_mut().copy_from_slice(data.as_slice())
                }
            });
        }
        */

        info!("temporary copy data!");
        let kstack = KernelStack::new();

        /*
        // remove the raw pointer for the memory set in memory_set_record
        {
            let mut mmset_record = memory_set_record();
            let id = mmset_record.iter()
                .position(|x| x.clone() == mmset_ptr).expect("id not exist");
            mmset_record.remove(id);
        }
        */

        let mut ret = Box::new(ContextImpl {
            arch: unsafe { ArchContext::new_fork(tf, kstack.top(), memory_set.token()) },
            memory_set,
            kstack,
            cwd: String::new(),
        });

        //memory_set_map_swappable(ret.get_memory_set_mut());
        info!("FORK() finsihed!");
        ret
    }

    pub fn get_memory_set_mut(&mut self) -> &mut Box<MemorySet> {
        &mut self.memory_set
    }

}

/*
impl Drop for ContextImpl{
    fn drop(&mut self){
        info!("come in to drop for ContextImpl");
        //set the user Memory pages in the memory set unswappable
        let Self {ref mut arch, ref mut memory_set, ref mut kstack, ..} = self;
        let pt = {
            memory_set.get_page_table_mut() as *mut InactivePageTable0
        };
        for area in memory_set.iter(){
            for page in Page::range_of(area.get_start_addr(), area.get_end_addr()) {
                let addr = page.start_address();
                unsafe {
                    // here we should get the active_table's lock before we get the swap_table since in memroy_set's map function
                    // we get pagetable before we get the swap table lock
                    // otherwise we may run into dead lock
                    let mut temp_table = active_table();
                    swap_table().remove_from_swappable(temp_table.get_data_mut(), pt, addr, || alloc_frame().expect("alloc frame failed"));
                }
            }
        }
        debug!("Finishing setting pages unswappable");
    }
}
*/

impl Debug for ContextImpl {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{:x?}", self.arch)
    }
}

/// Push a slice at the stack. Return the new sp.
unsafe fn push_slice<T: Copy>(mut sp: usize, vs: &[T]) -> usize {
    use core::{mem::{size_of, align_of}, slice};
    sp -= vs.len() * size_of::<T>();
    sp -= sp % align_of::<T>();
    slice::from_raw_parts_mut(sp as *mut T, vs.len())
        .copy_from_slice(vs);
    sp
}

unsafe fn push_args_at_stack<'a, Iter>(args: Iter, stack_top: usize) -> usize
    where Iter: Iterator<Item=&'a str>
{
    use core::{ptr, slice};
    let mut sp = stack_top;
    let mut argv = Vec::new();
    for arg in args {
        sp = push_slice(sp, &[0u8]);
        sp = push_slice(sp, arg.as_bytes());
        argv.push(sp);
    }
    sp = push_slice(sp, argv.as_slice());
    sp = push_slice(sp, &[argv.len()]);
    sp
}


/*
* @param:
*   elf: the source ELF file
* @brief:
*   generate a memory set according to the elf file
* @retval:
*   the new memory set
*/
fn memory_set_from<'a>(elf: &'a ElfFile<'a>) -> Box<MemorySet> {
    debug!("come in to memory_set_from");
    let mut set = Box::new(MemorySet::new());
    //let pt_ptr = set.get_page_table_mut() as *mut InactivePageTable0;
    for ph in elf.program_iter() {
        if ph.get_type() != Ok(Type::Load) {
            continue;
        }
        let (virt_addr, mem_size, flags) = match ph {
            ProgramHeader::Ph32(ph) => (ph.virtual_addr as usize, ph.mem_size as usize, ph.flags),
            ProgramHeader::Ph64(ph) => (ph.virtual_addr as usize, ph.mem_size as usize, ph.flags),
        };
        info!("virtaddr: {:x?}, memory size: {:x?}, flags: {}", virt_addr, mem_size, flags);
        // for SwapMemoryHandler
        //set.push(MemoryArea::new(virt_addr, virt_addr + mem_size, Box::new(SwapMemoryHandler::new(SWAP_TABLE.clone(), memory_attr_from(flags), Vec::<VirtAddr>::new())), ""));
        // for CowMemoryHandler
        set.push(MemoryArea::new(virt_addr, virt_addr + mem_size, Box::new(CowMemoryHandler::new(COW_TABLE.clone(), memory_attr_from(flags))), ""));

    }
    set
}

fn memory_attr_from(elf_flags: Flags) -> MemoryAttr {
    let mut flags = MemoryAttr::default().user();
    // TODO: handle readonly
    if elf_flags.is_execute() { flags = flags.execute(); }
    flags
}

/*
* @param:
*   memory_set: the target MemorySet to set swappable
* @brief:
*   map the memory area in the memory_set swappalbe, specially for the user process
*/
/*
pub fn memory_set_map_swappable(memory_set: &mut MemorySet){
    
    info!("COME INTO memory set map swappable!");
    let pt = unsafe {
        memory_set.get_page_table_mut() as *mut InactivePageTable0
    };
    for area in memory_set.iter(){
        for page in Page::range_of(area.get_start_addr(), area.get_end_addr()) {
            let addr = page.start_address();
            unsafe { swap_table().set_swappable(active_table().get_data_mut(), pt, addr); }
        }
    }
    
    info!("Finishing setting pages swappable");
}
*/