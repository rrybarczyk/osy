#![feature(asm, lang_items)]

extern crate xmodem;
extern crate pi;

pub mod lang_items;

/// Start address of the binary to load and of the bootloader.
const BINARY_START_ADDR: usize = 0x80000;
const BOOTLOADER_START_ADDR: usize = 0x4000000;

/// Pointer to where the loaded binary expects to be laoded.
const BINARY_START: *mut u8 = BINARY_START_ADDR as *mut u8;

/// Free space between the bootloader and the loaded binary's start address.
const MAX_BINARY_SIZE: usize = BOOTLOADER_START_ADDR - BINARY_START_ADDR;

/// Branches to the address `addr` unconditionally.
fn jump_to(addr: *mut u8) -> ! {
    unsafe {
        asm!("br $0" : : "r"(addr as usize));
        loop { asm!("nop" :::: "volatile")  }
    }
}

fn blinky(pin: &mut pi::gpio::Gpio<pi::gpio::Output>, count: u8, interval: u64) {

    for _ in 0..count {
        pi::timer::spin_sleep_ms(interval);
        pin.clear();
        pi::timer::spin_sleep_ms(interval);
        pin.set();
    }
}

#[no_mangle]
pub extern "C" fn kmain() {
    // FIXME: Implement the bootloader.
    // init MUart with timeout = 750ms
    // allocate space for binary (use from_raw_parts_mut)
    // call xmodem receive
    // when xmodem returns, jump_to the start of the binary
    // use blinky to indicate transmission success/failure

    // Construct MiniUart
    let mut mu = pi::uart::MiniUart::new();
    mu.set_read_timeout(750);                   // timeout after 750 ms

    // Indicate transmission is ready by flahing 5 times and turning on
    let mut gpio16 = pi::gpio::Gpio::new(16).into_output();
    blinky(&mut gpio16, 5, 500);

    let mut space_buf = unsafe { std::slice::from_raw_parts_mut(BINARY_START, MAX_BINARY_SIZE) };

    loop {

        match xmodem::Xmodem::receive(&mut mu, &mut space_buf) {
            Ok(_) => {
                // Blink when data is transferred properly
                blinky(&mut gpio16, 3, 300);
		jump_to(BINARY_START);
            },
            Err(_) => continue,
        }
    }
}
