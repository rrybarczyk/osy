/// A raw `ATAG` as laid out in memory.
#[repr(C)]
pub struct Atag {
    pub dwords: u32,
    pub tag: u32,
    pub kind: Kind
}

impl Atag {
    pub const NONE: u32 = 0x00000000;       // Empty rag used to end lst
    pub const CORE: u32 = 0x54410001;       // Start tag used to begin list
    pub const MEM: u32 = 0x54410002;        // Tag used to describe a physical area of memory
    pub const VIDEOTEXT: u32 = 0x54410003;  // Tag used to describe VGA text type displays
    pub const RAMDISK: u32 = 0x54410004;    // Tag describing how the ramdisk will be used by the kernel
    pub const INITRD2: u32 = 0x54420005;    // Tag describing the physical location of the compressed ramdisk
    pub const SERIAL: u32 = 0x54410006;     // Tag with 64 bit serial number of the board
    pub const REVISION: u32 = 0x54410007;   // Tag for the board revision
    pub const VIDEOLFB: u32 = 0x54410008;   // Tag describing parameters for a framebuffer type display
    pub const CMDLINE: u32 = 0x54410009;    // Tag used to pass the commandline to the kernel

    /// Returns the ATAG following `self`, if there is one.
    /// #define tag_next(t)     ((struct tag *)((u32 *)(t) + (t)->hdr.size))
    pub fn next(&self) -> Option<&Atag> {

        // Cast Atag pointer as u32
        let tag_current = self as *const Atag as *const u32;

        // Return next ATAG
        unsafe {
            // Get next ATAG, returns *const u32 cast as *const Atag
            let tag_next = tag_current.offset(self.dwords as isize) as *const Atag;
            tag_next.as_ref()
        }
    }
}

/// The possible variant of an ATAG.
#[repr(C)]
pub union Kind {
    pub core: Core,
    pub mem: Mem,
    pub cmd: Cmd
}

/// A `CORE` ATAG.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Core {
    pub flags: u32,
    pub page_size: u32,
    pub root_dev: u32
}

/// A `MEM` ATAG.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Mem {
    pub size: u32,
    pub start: u32
}

/// A `CMDLINE` ATAG.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Cmd {
    /// The first byte of the command line string.
    pub cmd: u8
}
