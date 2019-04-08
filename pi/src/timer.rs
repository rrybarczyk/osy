use common::IO_BASE;
use volatile::prelude::*;
use volatile::{Volatile, ReadVolatile};

/// The base address for the ARM system timer registers.
const TIMER_REG_BASE: usize = IO_BASE + 0x3000;

#[repr(C)]
#[allow(non_snake_case)]
struct Registers {
    CS: Volatile<u32>,              // SysTick Control/Status
    CLO: ReadVolatile<u32>,         // SysTick Counter Lower 32
    CHI: ReadVolatile<u32>,         // SysTick Counter Higher 32
    COMPARE: [Volatile<u32>; 4]     // SysTick Compare 0-3
}

/// The Raspberry Pi ARM system timer.
pub struct Timer {
    registers: &'static mut Registers
}

impl Timer {
    /// Returns a new instance of `Timer`.
    pub fn new() -> Timer {
        Timer {
            registers: unsafe { &mut *(TIMER_REG_BASE as *mut Registers) },
        }
    }

    /// Reads the system timer's counter and returns the 64-bit counter value.
    /// The returned value is the number of elapsed microseconds.
    pub fn read(&self) -> u64 {
        let clo = self.registers.CLO.read() as u64;
        let chi  = (self.registers.CHI.read() as u64) << 32;
        (chi | clo)
    }
}

/// Returns the current time in microseconds.
pub fn current_time() -> u64 {
    Timer::new().read()
}

/// Spins until `us` microseconds have passed.
pub fn spin_sleep_us(us: u64) {
    let end = current_time() + us;
    while current_time() <= end { }
}

/// Spins until `ms` milliseconds have passed.
pub fn spin_sleep_ms(ms: u64) {
    spin_sleep_us(ms * 1000)
}
