#![no_std]
#![no_main]

use catos::api::syscall;
use catos::entry_point;

entry_point!(main);

fn main(_args: &[&str]) {
    syscall::write(1, b"\x1b[93m"); // Yellow
    syscall::write(1, b"CATOS has reached its fate, ");
    syscall::write(1, b"the system is now halting.");
    syscall::write(1, b"\x1b[0m"); // Reset
    syscall::write(1, b"\n");
    syscall::sleep(0.5);
    syscall::halt();
    loop {
        syscall::sleep(1.0)
    }
}
