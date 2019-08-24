#![feature(custom_test_frameworks)]
#![test_runner(kvmtest::runner::runner)]

// Run with:
// RUSTFLAGS="-C relocation-model=dynamic-no-pic -C code-model=kernel" RUST_BACKTRACE=1 cargo test --verbose --test kvm -- --nocapture

extern crate core;
extern crate x86;
#[macro_use]
extern crate klogger;

extern crate kvmtest;
use self::kvmtest::kassert;
use self::kvmtest::kpanic;
use self::kvmtest::kvmtest;
use self::kvmtest::KvmTestFn;

#[kvmtest(ioport(0x1, 0xfe))]
fn use_the_port() {
    unsafe {
        kassert!(
            x86::io::inw(0x1) == 0xfe,
            "`inw` instruction didn't read the correct value"
        );
    }
}

#[kvmtest(ram(0x30000000, 0x31000000))]
fn print_works() {
    sprint!("sprint!, ");
    sprintln!("sprintln! works");
}

#[kvmtest]
#[should_panic]
fn panic_test() {
    kpanic!("failed");
}
