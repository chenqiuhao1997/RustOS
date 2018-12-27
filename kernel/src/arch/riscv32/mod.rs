extern crate riscv;
extern crate bbl;

pub mod io;
pub mod interrupt;
pub mod timer;
pub mod compiler_rt;
pub mod consts;
pub mod cpu;

#[no_mangle]
pub extern fn rust_main(hartid: usize, dtb: usize, hart_mask: usize) -> ! {
    unsafe { cpu::set_cpu_id(hartid); }
    println!("Hello RISCV! in hart {}, {}, {}", hartid, dtb, hart_mask);

    if hartid != 0 {
        while unsafe { !cpu::has_started(hartid) }  { }
        others_main();
        unreachable!();
    }

    ::logging::init();
    interrupt::init();
    timer::init();
    unsafe{interrupt::restore(2);}
    unsafe { cpu::start_others(hart_mask); }

    #[cfg(feature = "lab_test")]
    lab_test();
    
    ::kmain();
}

fn others_main() -> ! {
    interrupt::init();
    timer::init();
    
    ::kmain();
}

fn lab_test(){
    println!("kernel lab_test finished");
}

#[cfg(feature = "no_bbl")]
global_asm!(include_str!("boot/boot.asm"));
global_asm!(include_str!("boot/entry.asm"));
global_asm!(include_str!("boot/trap.asm"));