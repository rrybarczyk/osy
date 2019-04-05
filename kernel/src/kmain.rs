#![feature(lang_items)]
#![feature(core_intrinsics)]
#![feature(const_fn)]
#![feature(asm)]
#![feature(optin_builtin_traits)]
#![feature(decl_macro)]
#![feature(repr_align)]
#![feature(attr_literals)]
#![feature(never_type)]
#![feature(ptr_internals)]
#![feature(pointer_methods)]

extern crate pi;
extern crate stack_vec;

pub mod lang_items;
pub mod mutex;
pub mod console;
pub mod shell;

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
    mu.write_str("hello world");
    mu.set_read_timeout(10000);
    loop {
        mu.write_str("$ ");
        let mut buf = [0u8; 1];
        match mu.read(&mut buf) {
            Ok(n) => {
                mu.write(&buf[0..n]);
                mu.write_str("\n").unwrap();
            },
            Err(_) => mu.write_str("you took to long").unwrap(),
        };
    }
}

#[no_mangle]
pub extern "C" fn kmain() {
    blinky(16, 1000);
    echo();
}
