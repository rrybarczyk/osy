use console::{kprintln};

#[no_mangle]
#[cfg(not(test))]
#[lang = "panic_fmt"]
pub extern fn panic_fmt(fmt: ::std::fmt::Arguments, file: &'static str, line: u32, col: u32) -> ! {
    // Print `fmt`, `file`, and `line` to the console.
    kprintln!("");

    kprintln!("(-(-_(-_-)_-)-)");

    kprintln!("");

    kprintln!("FILE: {}", file);
    kprintln!("LINE: {}", line);
    kprintln!("COL:  {}", col);

    kprintln!("");

    kprintln!("{:?}", fmt);

    loop { unsafe { asm!("wfe") } }
}

#[cfg(not(test))] #[lang = "eh_personality"] pub extern fn eh_personality() {}
