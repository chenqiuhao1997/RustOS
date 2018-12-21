extern crate cc;

use std::fs::File;
use std::io::{Write, Result};

fn main() {
	if std::env::var("TARGET").unwrap().find("riscv32").is_some() {
		cc::Build::new()
			.file("src/arch/riscv32/compiler_rt.c")
			.flag("-march=rv32ia")
			.flag("-mabi=ilp32")
			.compile("atomic_rt");
	}
}