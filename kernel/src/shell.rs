//! Kernel shell

use alloc::string::String;
use alloc::vec::Vec;
use fs::{ROOT_INODE, INodeExt};
use process::*;


pub fn run_user_shell() {
    #[cfg(feature = "no_test")]
    let inode = ROOT_INODE.lookup("sh").unwrap();

    #[cfg(feature = "priority_test")]
    let inode = ROOT_INODE.lookup("priority").unwrap();

    let data = inode.read_as_vec().unwrap();

    #[cfg(feature = "no_test")]
    processor().manager().add(ContextImpl::new_user(data.as_slice(), "sh".split(' ')), 0);

    #[cfg(feature = "priority_test")]
    processor().manager().add(ContextImpl::new_user(data.as_slice(), "priority".split(' ')), 3);
}

pub fn shell() {
    let files = ROOT_INODE.list().unwrap();
    println!("Available programs: {:?}", files);

    loop {
        print!(">> ");
        let cmd = get_line();
        if cmd == "" {
            continue;
        }
        let name = cmd.split(' ').next().unwrap();
        if let Ok(file) = ROOT_INODE.lookup(name) {
            let data = file.read_as_vec().unwrap();
            let pid = processor().manager().add(ContextImpl::new_user(data.as_slice(), cmd.split(' ')), thread::current().id());
            unsafe { thread::JoinHandle::<()>::_of(pid) }.join().unwrap();
        } else {
            println!("Program not exist");
        }
    }
}

fn get_line() -> String {
    let mut s = String::new();
    loop {
        let c = get_char();
        match c {
            '\u{7f}' /* '\b' */ => {
                if s.pop().is_some() {
                    print!("\u{7f}");
                }
            }
            ' '...'\u{7e}' => {
                s.push(c);
                print!("{}", c);
            }
            '\n' | '\r' => {
                print!("\n");
                return s;
            }
            _ => {}
        }
    }
}

fn get_char() -> char {
    ::fs::STDIN.pop()
}
