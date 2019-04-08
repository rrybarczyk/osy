#![feature(lang_items)]
#![feature(core_intrinsics)]
#![feature(const_fn)]
#![feature(asm)]
#![feature(optin_builtin_traits)]
#![feature(decl_macro)]
#![feature(repr_align)]
#![feature(attr_literals)]
#![feature(exclusive_range_pattern)]
#![feature(alloc, allocator_api, global_allocator)]
#![feature(never_type)]
#![feature(ptr_internals)]
#![feature(pointer_methods)]

#[macro_use]
#[allow(unused_imports)]
extern crate alloc;
extern crate pi;
extern crate stack_vec;
extern crate fat32;

pub mod allocator;
pub mod lang_items;
pub mod mutex;
pub mod console;
pub mod shell;
pub mod fs;

#[cfg(not(test))]
use allocator::Allocator;
use fs::FileSystem;

#[cfg(not(test))]
#[global_allocator]
pub static ALLOCATOR: Allocator = Allocator::uninitialized();

pub static FILE_SYSTEM: FileSystem = FileSystem::uninitialized();

const GPIO_BASE: usize = 0x3F000000 + 0x200000;

const GPIO_FSEL1: *mut u32 = (GPIO_BASE + 0x04) as *mut u32;
const GPIO_SET0: *mut u32 = (GPIO_BASE + 0x1C) as *mut u32;
const GPIO_CLR0: *mut u32 = (GPIO_BASE + 0x28) as *mut u32;

use std::fmt::Write as FmtWrite;
use std::io::{Write, Read };

/// Test system timer and gpio driver
fn blinky(pin: u8, interval: u64) {
    let mut gpiopin = pi::gpio::Gpio::new(pin).into_output();

    for _ in 0..10 {
        gpiopin.set();
        pi::timer::spin_sleep_ms(interval);
        gpiopin.clear();
        pi::timer::spin_sleep_ms(interval);
    }
}

/// Test uart driver
fn echo() {
    let mut mu = pi::uart::MiniUart::new();
    mu.write_str("hello world").expect("write str err");
    mu.set_read_timeout(10000);
    loop {
        mu.write_str("$ ").expect("write str err");;
        let mut buf = [0u8; 1];
        match mu.read(&mut buf) {
            Ok(n) => {
                mu.write(&buf[0..n]).expect("write err");
                mu.write_str("\n").expect("write str err");;
            },
            Err(_) => mu.write_str("you took to long").unwrap(),
        };
    }
}

#[no_mangle]
#[cfg(not(test))]
pub extern "C" fn kmain() {
    ALLOCATOR.initialize();
    blinky(16, 1000);
    shell::shell("$ ");
    // echo();
}
