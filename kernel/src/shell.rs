use std::str;
use std::io::Write;
use stack_vec::StackVec;
use console::{kprint, kprintln, CONSOLE};

/// Error type for `Command` parse failures.
#[derive(Debug)]
enum Error {
    Empty,
    TooManyArgs,
}

/// A structure representing a single shell command.
#[derive(Debug)]
struct Command<'a> {
    args: StackVec<'a, &'a str>
}

impl<'a> Command<'a> {
    /// Parse a command from a string `s` using `buf` as storage for the
    /// arguments.
    ///
    /// # Errors
    ///
    /// If `s` contains no arguments, returns `Error::Empty`. If there are more
    /// arguments than `buf` can hold, returns `Error::TooManyArgs`.
    fn parse(s: &'a str, buf: &'a mut [&'a str]) -> Result<Command<'a>, Error> {
        let mut args = StackVec::new(buf);
        for arg in s.split(' ').filter(|a| !a.is_empty()) {
            args.push(arg).map_err(|_| Error::TooManyArgs)?;
        }

        if args.is_empty() {
            return Err(Error::Empty);
        }

        Ok(Command { args })
    }

    /// Returns this command's path. This is equivalent to the first argument.
    fn path(&self) -> &str {
        self.args[0]
    }
}

/// Starts a shell using `prefix` as the prefix for each line. This function
/// never returns: it is perpetually in a shell loop.
pub fn shell(prefix: &str) -> ! {
    // \r, \n are ENTER
    // ASCII 8 and 127 are backspace and delete which erase a single char
    // if invalid ASCII -> ring BELL
    // receive a stream of incoming bytes
    // as each byte is received, check its value
    // if it if a valid value, put it on the stack
    // if it is an invalid value right BELL and do not change the stack
    // if it is a backspace/del, remove last item on the stack
    // send final stack array to the parse fun as a string slice
    // get back a Command with args arg
    // look at first arg, if echo then call echo and pass in remaining parameters
    // if not echo, then print"unknown command"

    // unimplemented!();

    const BEL: u8   = 0x7;      // Bell
    const BS: u8    = 0x8;      // Backspace
    const DEL: u8   = 0x7F;     // Delete
    const LF: u8    = 0x0A;     // Line Feed
    const CR: u8    = 0x0D;     // Carriage Return

    'cmd: loop {
        let mut stack_buf = [0u8; 512];                 // Max command length is 512 bytes
        let mut stack = StackVec::new(&mut stack_buf);

        kprint!("{}", prefix);                          // Indicate accepting new command

        // Loop for each input character
        'arg: loop {
            let mut console = CONSOLE.lock();

            let input = console.read_byte();

            // Ring BELL if invalid ascii
            if !input.is_ascii() {
                console.write_byte(BEL);
                continue;
            } 

            // If backspace or delete char received. Pop last char off stack.
            // Rings bell if stack is empty.
            // If line feed or carriage return char received, cmd finished. Send stack buf to be parsed
            // If valid char received. Push to stack and print to screen
            // Rings bell if stack is full
            if input == BS || input == DEL {
                // Backspace or delete received
                match stack.pop() {
                    Some(_) => {
                        console.write(&[BS, b' ', BS]).expect("ehh");
                    },
                    None => {
                        console.write_byte(BEL);    //  Empty stack, ring bell
                    }
                };

            } else if input == LF || input == CR {
                // Line feed or carriage return received
                let mut cmd_buf: [&str; 64] = [""; 64];

                // Parse completed command
                match Command::parse(str::from_utf8(stack.into_slice()).unwrap(), &mut cmd_buf) {
                    Ok(_cmd) => {
                        console.write_byte(LF);
                        console.write_byte(CR);
                        continue 'cmd;      // Command parsed, continue to 'cmd loop to start new command
                    },
                    Err(Error::Empty) => {
                        console.write_byte(LF);     // Empty stack, go to next line
                        console.write_byte(CR);
                        break 'arg;
                    },
                    Err(Error::TooManyArgs) => {
                        console.write_byte(BEL);    //  Too many arguements, ring bell
                        console.write_byte(LF);      // Go to next line
                        console.write_byte(CR);
                        break 'arg;
                    },
                }
            } else {
                // Char in command received, push to stack and write to screen
                match stack.push(input) {
                    Ok(_) => console.write_byte(input),
                    Err(_) => console.write_byte(BEL),
                };
            }
        }
    }
}
