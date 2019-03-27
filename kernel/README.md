# kernel
Phase 3 of Assignment 1 from the [CS140 course](https://cs140e.sergio.bz/assignments/1-shell/).

The main OS kernel.

## Rust Versioning
```
$ rustup override set nightly-2018-01-09
$ cargo install xargo --version 0.3.10

$ rustc --version
rustc 1.25.0-nightly (b5392f545 2018-01-08)

$ xargo --version
xargo 0.3.10
cargo 0.25.0-nightly (a88fbace4 2017-12-29)
```


## `blinky` branch
Tests system timer in `pi::timer` to blink LED.
