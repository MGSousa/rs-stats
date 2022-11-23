#[macro_use]
extern crate prettytable;

mod top;
mod ps;
mod mem;

use std::{
    env,
    process::exit
};
use heim::process::ProcessResult;

#[cfg(unix)]
use heim::memory::os::macos::MemoryExt;

fn main() -> ProcessResult<()> {
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        if args[1].as_str() == "top" {
            show_top()
        } else if args[1].as_str() == "ps" {
            show_ps()
        } else if args[1].as_str() == "mem" {
            show_mem()
        } else {
            println!("Valid args for system stat: {}", "top, ps, mem");
            exit(1)
        }
    } else {
        show_mem()
    }
}

fn show_top() -> ProcessResult<()> {
    top::htop()
}

fn show_ps() -> ProcessResult<()> {
    ps::process()
}

fn show_mem() -> ProcessResult<()> {
    mem::free()
}