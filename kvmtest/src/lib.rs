//! kvmtest infrastructure to run rust unit tests in guest-ring 0.
#![feature(lang_items, const_fn)]

extern crate kvm;
extern crate mmap;
extern crate x86;

#[macro_use]
extern crate log;

extern crate kvmtest_macro;
extern crate kvmtest_types;

mod hypervisor;
pub mod runner;

pub use kvmtest_macro::kvmtest;
pub use kvmtest_types::*;
