# osy
Contains:
Phase 3 and 4 from [Assignment 1](https://cs140e.sergio.bz/assignments/1-shell/) and Phase 0 - 4 from [Assignment 2](https://cs140e.sergio.bz/assignments/2-fs/) from the [CS140 course](https://cs140e.sergio.bz/).

Phase 0 - 2 from Assignment 1 can be found [here](https://github.com/rrybarczyk/shelly).

The `assignment-1` branch contains the completed code for first assignment, and the `assignment-2` branch contains the completed code for second assignment.

Directory structure:

|-- `bootloader`  
|-- `files`  
|-- `kernel`  
|-- `pi`  
|-- `std`  
|-- `volatile`  

## Rust Versioning
```
$ rustup install nightly-2018-01-09
$ rustup default nightly-2018-01-09
$ rustup override set nightly-2018-01-09
$ rustup component add rust-src

$ cargo install xargo --version 0.3.10

$ rustc --version
rustc 1.25.0-nightly (b5392f545 2018-01-08)

$ xargo --version
xargo 0.3.10
cargo 0.25.0-nightly (a88fbace4 2017-12-29)
```

## Assignment 1: Shell

### Phase 3: *Not* a Seashell
- [x] Subphase A: Getting Started
- [x] Subphase B: System Timer
- [x] Subphase C: GPIO
- [x] Subphase D: UART
- [x] Subphase E: The Shell
     
### Phase 4: Boot 'em Up
- [x] Loading Binaries
- [x] Making Space
- [x] Implementing the Bootloader

## Assignment 2: File System

### Phase 0: Getting Started
- [x] Getting the Skeleton Code
- [x] Firmware Update
- [x] Installing ttywrite

### Phase 1: Memory Lane
- [x] Subphase A: Panic!
- [x] Subphase B: ATAGS
- [ ] Subphase C: Warming Up
- [ ] Subphase D: Bump Allocator
- [ ] Subphase E: Bin Allocator

### Phase 2: 32-bit Lipids
- [ ] Disks and File Systems
- [ ] Disk Layout
- [ ] Code Structure
- [ ] Implementation

### Phase 3: Saddle Up
- [ ] Subphase A: SD Driver FFI
- [ ] Subphase B: File System

### Phase 4: Mo'sh
- [ ] Working Directory
- [ ] Commands
- [ ]  Implementation
