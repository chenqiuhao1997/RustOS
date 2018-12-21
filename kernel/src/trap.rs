use arch::interrupt::TrapFrame;
use arch::cpu;

pub static mut TICK: usize = 0;

pub fn timer() {
    if cpu::id() == 0 {
        unsafe { TICK += 1; }
    }
    unsafe{  println!("ticks {:?}", TICK);}
}

pub fn error(tf: &TrapFrame) -> ! {
    unimplemented!()
}

pub fn serial(c: char) {
    ::fs::STDIN.push(c);
}