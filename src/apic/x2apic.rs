//! x2APIC, the most recent APIC on x86 for large servers with more than 255 cores.
use bit_field::BitField;

use crate::msr::{
    rdmsr, wrmsr, IA32_APIC_BASE, IA32_TSC_DEADLINE, IA32_X2APIC_APICID, IA32_X2APIC_ESR,
    IA32_X2APIC_LVT_LINT0, IA32_X2APIC_LVT_TIMER, IA32_X2APIC_SELF_IPI, IA32_X2APIC_VERSION,
};

/// Represents an x2APIC driver instance.
#[derive(Debug)]
pub struct X2APIC {
    /// Initial BASE msr register value.
    base: u64,
}

impl X2APIC {
    /// Create a new x2APIC driver object for the local core.
    pub fn new() -> X2APIC {
        unsafe {
            X2APIC {
                base: rdmsr(IA32_APIC_BASE),
            }
        }
    }

    /// Attach to APIC (enable x2APIC mode, initialize LINT0)
    pub fn attach(&mut self) {
        // Enable
        unsafe {
            self.base = rdmsr(IA32_APIC_BASE);
            self.base.set_bit(10, true); // Enable x2APIC
            self.base.set_bit(11, true); // Enable xAPIC
            wrmsr(IA32_APIC_BASE, self.base);

            //TODO: let mut lint0 = rdmsr(IA32_X2APIC_LVT_LINT0);
            // TODO: Fix magic number
            let lint0 = 1 << 16 | (1 << 15) | (0b111 << 8) | 0x20;
            wrmsr(IA32_X2APIC_LVT_LINT0, lint0);

            let _esr = rdmsr(IA32_X2APIC_ESR);
        }
    }

    /// Detach from APIC (disable x2APIC and xAPIC mode).
    pub fn detach(&mut self) {
        unsafe {
            self.base = rdmsr(IA32_APIC_BASE);
            self.base.set_bit(10, false); // x2APIC
            self.base.set_bit(11, false); // xAPIC
            wrmsr(IA32_APIC_BASE, self.base);
        }
    }

    /// Are we the BSP core?
    pub fn bsp(&self) -> bool {
        (self.base & (1 << 8)) > 0
    }

    /// Read local APIC ID.
    pub fn id(&self) -> u32 {
        unsafe { rdmsr(IA32_X2APIC_APICID) as u32 }
    }

    /// Read APIC version.
    pub fn version(&self) -> u32 {
        unsafe { rdmsr(IA32_X2APIC_VERSION) as u32 }
    }

    /// Enable TSC timer
    pub unsafe fn tsc_enable(&self) {
        let mut lvt: u64 = rdmsr(IA32_X2APIC_LVT_TIMER);
        lvt |= 0 << 17;
        lvt |= 1 << 18;
        wrmsr(IA32_X2APIC_LVT_TIMER, lvt);
    }

    /// Set tsc deadline.
    pub unsafe fn tsc_set(&self, value: u64) {
        wrmsr(IA32_TSC_DEADLINE, value);
    }

    /// Send an IPI to yourself.
    pub unsafe fn send_self_ipi(&self, vector: u64) {
        wrmsr(IA32_X2APIC_SELF_IPI, vector);
    }
}
