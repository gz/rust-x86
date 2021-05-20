//! x86test infrastructure to run rust unit tests in guest-ring 0.
#![feature(lang_items)]

extern crate klogger;
extern crate kvm_sys as kvm;
extern crate mmap;
extern crate x86;

#[macro_use]
extern crate log;

extern crate x86test_macro;
extern crate x86test_types;

mod hypervisor;
pub mod runner;

pub use x86test_macro::x86test;
pub use x86test_types::*;

pub use klogger::{sprint, sprintln};
pub use x86::io::outw;
