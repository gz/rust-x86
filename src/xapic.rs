//! Information about the xAPIC and x2APIC mode for the local APIC.
//!
//! Table 10-1 Local APIC Register Address Map
//! the MMIO base values are found in this file, for x2APIC MSR see msr.rs.

///	Local APIC ID register. Read-only. See Section 10.12.5.1 for initial values.
pub const XAPIC_ID: u32 = 0x020;

///	Local APIC Version register. Read-only. Same version used in xAPIC mode and x2APIC mode.
pub const XAPIC_VERSION: u32 = 0x030;

///	Task Priority Register (TPR). Read/write. Bits 31:8 are reserved.
pub const XAPIC_TPR: u32 = 0x080;

///	Processor Priority Register (PPR). Read-only.
pub const XAPIC_PPR: u32 = 0x0A0;

///	EOI register. Write-only.
pub const XAPIC_EOI: u32 = 0x0B0;

///	Logical Destination Register (LDR). Read/write in xAPIC mode.
pub const XAPIC_LDR: u32 = 0x0D0;

/// Spurious Interrupt Vector Register (SVR). Read/write. See Section 10.9 for reserved bits.
pub const XAPIC_SVR: u32 = 0x0F0;

/// In-Service Register (ISR); bits 31:0. Read-only.
pub const XAPIC_ISR0: u32 = 0x100;

/// ISR bits 63:32. Read-only.
pub const XAPIC_ISR1: u32 = 0x110;

/// ISR bits 95:64. Read-only.
pub const XAPIC_ISR2: u32 = 0x120;

/// ISR bits 127:96. Read-only.
pub const XAPIC_ISR3: u32 = 0x130;

/// ISR bits 159:128. Read-only.
pub const XAPIC_ISR4: u32 = 0x140;

/// ISR bits 191:160. Read-only.
pub const XAPIC_ISR5: u32 = 0x150;

/// ISR bits 223:192. Read-only.
pub const XAPIC_ISR6: u32 = 0x160;

/// ISR bits 255:224. Read-only.
pub const XAPIC_ISR7: u32 = 0x170;

/// Trigger Mode Register (TMR); bits 31:0. Read-only.
pub const XAPIC_TMR0: u32 = 0x180;

/// TMR bits 63:32. Read-only.
pub const XAPIC_TMR1: u32 = 0x190;

/// TMR bits 95:64. Read-only.
pub const XAPIC_TMR2: u32 = 0x1A0;

/// TMR bits 127:96. Read-only.
pub const XAPIC_TMR3: u32 = 0x1B0;

/// TMR bits 159:128. Read-only.
pub const XAPIC_TMR4: u32 = 0x1C0;

/// TMR bits 191:160. Read-only.
pub const XAPIC_TMR5: u32 = 0x1D0;

/// TMR bits 223:192. Read-only.
pub const XAPIC_TMR6: u32 = 0x1E0;

/// TMR bits 255:224. Read-only.
pub const XAPIC_TMR7: u32 = 0x1F0;

/// Interrupt Request Register (IRR); bits 31:0. Read-only.
pub const XAPIC_IRR0: u32 = 0x200;

/// IRR bits 63:32. Read-only.
pub const XAPIC_IRR1: u32 = 0x210;

/// IRR bits 95:64. Read-only.
pub const XAPIC_IRR2: u32 = 0x220;

/// IRR bits 127:96. Read-only.
pub const XAPIC_IRR3: u32 = 0x230;

/// IRR bits 159:128. Read-only.
pub const XAPIC_IRR4: u32 = 0x240;

/// IRR bits 191:160. Read-only.
pub const XAPIC_IRR5: u32 = 0x250;

/// IRR bits 223:192. Read-only.
pub const XAPIC_IRR6: u32 = 0x260;

/// IRR bits 255:224. Read-only.
pub const XAPIC_IRR7: u32 = 0x270;

/// Error Status Register (ESR). Read/write. See Section 10.5.3.
pub const XAPIC_ESR: u32 = 0x280;

/// LVT CMCI register. Read/write. See Figure 10-8 for reserved bits.
pub const XAPIC_LVT_CMCI: u32 = 0x2F0;

/// Interrupt Command Register (ICR). Read/write. See Figure 10-28 for reserved bits
pub const XAPIC_ICR0: u32 = 0x300;

/// Interrupt Command Register (ICR). Read/write. See Figure 10-28 for reserved bits
pub const XAPIC_ICR1: u32 = 0x310;

/// LVT Timer register. Read/write. See Figure 10-8 for reserved bits.
pub const XAPIC_LVT_TIMER: u32 = 0x320;

/// LVT Thermal Sensor register. Read/write. See Figure 10-8 for reserved bits.
pub const XAPIC_LVT_THERMAL: u32 = 0x330;

/// LVT Performance Monitoring register. Read/write. See Figure 10-8 for reserved bits.
pub const XAPIC_LVT_PMI: u32 = 0x340;

/// LVT LINT0 register. Read/write. See Figure 10-8 for reserved bits.
pub const XAPIC_LVT_LINT0: u32 = 0x350;

/// LVT LINT1 register. Read/write. See Figure 10-8 for reserved bits.
pub const XAPIC_LVT_LINT1: u32 = 0x360;

/// LVT Error register. Read/write. See Figure 10-8 for reserved bits.
pub const XAPIC_LVT_ERROR: u32 = 0x370;

/// Initial Count register (for Timer). Read/write.
pub const XAPIC_TIMER_INIT_COUNT: u32 = 0x380;

/// Current Count register (for Timer). Read-only.
pub const XAPIC_TIMER_CURRENT_COUNT: u32 = 0x390;

/// Divide Configuration Register (DCR; for Timer). Read/write. See Figure 10-10 for reserved bits.
pub const XAPIC_TIMER_DIV_CONF: u32 = 0x3E0;
