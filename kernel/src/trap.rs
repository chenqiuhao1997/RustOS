use arch::interrupt::TrapFrame;
use arch::cpu;

pub static mut TICK: usize = 0;

pub fn timer() {
    if cpu::id() == 0 {
        unsafe { TICK += 1; }
    }
    #[cfg(not(feature = "lab_test"))]
    unsafe{  if (TICK & 63)==0 {println!("ticks 0x{tick:08x}", tick=TICK);}}
}

pub fn error(tf: &TrapFrame) -> ! {
    unimplemented!()
}

pub fn serial(c: char) {
    ::fs::STDIN.push(c);
}