//! x2APIC, the most recent APIC on x86 for large servers with more than 255 cores.
use bit_field::BitField;

use super::{ApicControl, ApicId, Icr};
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

    /// Send an IPI to yourself.
    pub unsafe fn send_self_ipi(&self, vector: u64) {
        wrmsr(IA32_X2APIC_SELF_IPI, vector);
    }
}

/// Abstracts common interface of APIC (x2APIC, xAPIC) hardware devices.
impl ApicControl for X2APIC {
    /// Is a bootstrap processor?
    fn bsp(&self) -> bool {
        (self.base & (1 << 8)) > 0
    }

    /// Read local APIC ID.
    fn id(&self) -> u32 {
        unsafe { rdmsr(IA32_X2APIC_APICID) as u32 }
    }

    /// Read APIC version.
    fn version(&self) -> u32 {
        unsafe { rdmsr(IA32_X2APIC_VERSION) as u32 }
    }

    /// Enable TSC timer
    fn tsc_enable(&mut self, vector: u8) {
        unsafe {
            let mut lvt: u64 = rdmsr(IA32_X2APIC_LVT_TIMER);
            lvt &= !(1 << 17);
            lvt |= 1 << 18;
            wrmsr(IA32_X2APIC_LVT_TIMER, lvt);
        }
    }

    /// Set tsc deadline.
    fn tsc_set(&self, value: u64) {
        unsafe {
            wrmsr(IA32_TSC_DEADLINE, value);
        }
    }

    /// End Of Interrupt -- Acknowledge interrupt delivery.
    fn eoi(&mut self) {
        unreachable!("NYI");
    }

    /// Send a INIT IPI to a core.
    unsafe fn ipi_init(&mut self, _core: ApicId) {
        unreachable!("NYI");
    }

    /// Deassert INIT IPI.
    unsafe fn ipi_init_deassert(&mut self) {
        unreachable!("NYI");
    }

    /// Send a STARTUP IPI to a core.
    unsafe fn ipi_startup(&mut self, _core: ApicId, _start_page: u8) {
        unreachable!("NYI");
    }

    /// Send a generic IPI.
    unsafe fn send_ipi(&mut self, _icr: Icr) {
        unreachable!("NYI");
    }
}
