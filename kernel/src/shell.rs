use stack_vec::StackVec;
use console::{kprint, kprintln, CONSOLE};

/// Error type for `Command` parse failures.
#[derive(Debug)]
enum Error {
    Empty,
    TooManyArgs,
    ParseError,
}

/// A structure representing a single shell command.
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

fn line_read(buf: &mut[u8]) -> Result<&str, Error> {
    unimplemented!();
}

/// Starts a shell using `prefix` as the prefix for each line. This function
/// never returns: it is perpetually in a shell loop.
pub fn shell(prefix: &str) -> ! {
    unimplemented!();
    // // Loop for each executed command. Execution is signaled by CR or NL
    // 'outer: loop {
    //     let mut buf = [0u8; 512];                   // Max cmd len is 512 bytes
    //     let mut cmd_stack = StackVec::new(&mut buf);    // Initialize cmd stack
    //
    //     kprint!("{}", prefix);                      // Print console prefix
    //
    //     let parsed_cmd = match line_read(cmd_stack.as_mut_slice()) {
    //         Ok(s) => Command::parse(s, &mut[""; 64]),
    //         Err(_) => Err(Error::ParseError),
    //     };
    // }
}
