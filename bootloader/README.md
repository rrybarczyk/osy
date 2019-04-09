# bootloader
Phase 4 of Assignment 1 from the [CS140 course](https://cs140e.sergio.bz/assignments/1-shell/).

The OS bootloader.

Send binaries to pi with

```
$ ttywrite -i my-new-binary.bin /dev/<your-device>
```

Very exciting.


## ARM Bootloader Linux Notes
Notes are from the [Booting ARM Linux](http://www.simtec.co.uk/products/SWLINUX/files/booting_article.html#d0e428) document.

The minimum functions that ARM Linux requires of the bootloader code are:

1. Configure the memory system
2. Load the kernel image at the correct memory address
3. Optionally load an initial RAM disk at the correct memory address
4. Initialize the boot parameters to pass to the kernel
5. Obtain the ARM Linux machine type
6. Enter the kernel with the appropriate register values
7. Initialize a serial or video console for the kernel (optional, but very commonly included)

### Configure the memory system
The bootloader find and initializes the RAM that the kernel uses for volatile data storage.
The physical memory layout is passed to the kernel with the `ATAG MEM` parameter (describes a physical area of memory). 
Memory can be allocated in discontiguous chunks, but it is best to keep those chunks to a minimum or, better yet, have a single contigious chunk.

### Load the kernel image at the correct memory address
The kernel build process generates an uncompressed Image file or a compressed zImage file.
zImage files are more common, but this project uses an Image file.

One of the reasons that zImage files are nice is because they have useful information in their header.
This information includes the 
- magic number that identifies the file as a zImage
- the address where the zImage starts
- the address where the zImage ends
The start and end addresses are used to determine the length of the compressed zImage file.

The kernel image file (zImage or Image) file can be placed anywhere in memory (it is Position Independent Code (PIC)), but by convention it is placed at the base of the physical RAM plus and offset of `0x8000` (`32K`).
The extra space at the beginning can be used for the parameter block, zero page exception vectors, and page tables.

The maximize size for the kernel image is 4MiB.

For this project, the kernel image is called `kernel8.img`.

### Loading an initial RAM disk at the correct memory address
An initial RAM disk (`initrd`) lets us have a root filesystem without using other drivers or configurations.

There are two ways to initialize the RAM disk:
1. A special build target boopImage is used. This takes an initial RAM disk at build time and appends it to the zImage. The bootloader doesn't do anything here, but the kernel must know the physical address of the ramdisk must place it there.
1. (Most common method) The bootloader places a given initial ramdisk image into memory at a set location that is passed to the kernel via the `ATAG_INITRD2` (describes the physical location of the compressed ramdisk image) and `ATAG_RAMDISK` (describes how the ramdisk will be used by the kernel).

The `initrd` must be put in a place with enough memory post boot to decompress the `initrd` into a real ramdisk (zImage + decompressed zImage + `initrd` + uncompressed ramdisk).
The compressed `initrd` is dreed after decompression.

The ramdisk position limitations are that it must:
- Lie completely within a single memory region (must not cross between areas degined by different `ATAG_MEM` parameters
- Be aligned to a page boundary (typically 4Kib)
- Not conflict with the memory the zImage head code uses to decompress the kernel or it will be overwritten

### Initilize a serial or video console for the kernel
The console is used for receiving feedback from the kernel about which actions it is performing. 
The bootloader initializes and enables a serial port on the target and the kernel serial driver detects which serial port it should use for the kernel console.


### Initialize boot parameters to pass to the kernel
The bootloader needs to let the kernel know the setup that the bootloader performed, like the size and memory as well as other parameters.

The taggest list should conform to the following constraints:
- It must be stored in RAM and placed in a region of memory that it cannot be overwritten. By convention is in the first 16KiB of RAM, usually the start of physical RAM plus `0x100` (which avoids zero page exception vectors)
- The physical address of the tagged list must be placed in R2 on entry to the kernel
- The list must not exceed past the `0x4000` boundary where the kernel's initial translation page table is created or else it will be overwritten by the kernel
- The list must be aligned to a word (32 bit, 4 byte) boundary (if not using the recommended location)
- The list must begin with an `ATAG_CORE` (start tag used to begin list) and end with an `ATAG_NONE` (empty tag used to end list)
- The list must contain at least one `ATAG_MEM` (describes a physical area of memory)


### Obtain the ARM Linux machine type
The bootloader needs to tell the kernel the machine type (a unique identifier indicating which machine it is running on) by using the `MACH_TYPE` parameter.


### Start the kernel
After executing all the steps, the bootloader finally starts the kernel with the correct values in the CPU registers:
- The CPU must be in SVC (supervisor) more with both IRQ and FIQ interrupts disabled
- The MMU must be off i.e. code running from physical RAM with no translated addressing
- Data cache must be off
- Instruction cache may be either on or off
- CPU register 0 must be 0
- CPU register 1 must be the ARM Linux machine type
- CPU register 2 must be the physical address of the parameter list

The bootloader is expected to call the kernel image by jumping directly to the first instruction of the kernel image.
