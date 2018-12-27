use process::*;
use arch::interrupt::TrapFrame;
use arch::cpu;

pub static mut TICK: usize = 0;

pub fn timer() {
    if cpu::id() == 0 {
        unsafe { TICK += 1; }
    }
    processor().tick();
}

pub fn error(tf: &TrapFrame) -> ! {
    error!("{:#x?}", tf);
    let pid = processor().pid();
    error!("On CPU{} Process {}", cpu::id(), pid);

    processor().manager().exit(pid, 0x100);
    processor().yield_now();
    unreachable!();
}

pub fn serial(c: char) {
    info!("serial in {}", c);
    ::fs::STDIN.push(c);
}