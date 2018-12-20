use arch::interrupt::{TrapFrame, Context as ArchContext};
use memory::{MemoryArea, MemoryAttr, MemorySet, KernelStack, swap_table, alloc_frame, active_table, NormalMemoryHandler, SwapMemoryHandler, CowMemoryHandler, SWAP_TABLE, COW_TABLE};
use xmas_elf::{ElfFile, header, program::{Flags, ProgramHeader, Type}};
use core::fmt::{Debug, Error, Formatter};
use alloc::{boxed::Box, collections::BTreeMap, vec::Vec, sync::Arc, string::String};
use ucore_memory::{Page, VirtAddr};
use ::memory::{InactivePageTable0};
use ucore_memory::memory_set::*;
use simple_filesystem::file::File;
use spin::Mutex;

