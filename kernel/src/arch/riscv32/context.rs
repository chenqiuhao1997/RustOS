use super::super::riscv::register::*;

#[derive(Clone)]
#[repr(C)]
pub struct TrapFrame {
    pub x: [usize; 32], // general registers
    pub sstatus: sstatus::Sstatus, // Supervisor Status Register
    pub sepc: usize, // Supervisor exception program counter, save the trap virtual address (here is used to save the process program entry addr?)
    pub sbadaddr: usize, // Supervisor bad address
    pub scause: scause::Scause, // scause register: record the cause of exception/interrupt/trap
}

use core::fmt::{Debug, Formatter, Error};
impl Debug for TrapFrame {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        struct Regs<'a>(&'a [usize; 32]);
        impl<'a> Debug for Regs<'a> {
            fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
                const REG_NAME: [&str; 32] = [
                    "zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2",
                    "s0", "s1", "a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7",
                    "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11",
                    "t3", "t4", "t5", "t6"];
                f.debug_map().entries(REG_NAME.iter().zip(self.0)).finish()
            }
        }
        f.debug_struct("TrapFrame")
            .field("regs", &Regs(&self.x))
            .field("sstatus", &self.sstatus)
            .field("sepc", &self.sepc)
            .field("sbadaddr", &self.sbadaddr)
            .field("scause", &self.scause)
            .finish()
    }
}



