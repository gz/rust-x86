//! Functions and data-structures to load descriptor tables.

use core::mem::size_of;
use std::fmt;

use current::irq::IdtEntry;
use shared::segmentation::SegmentDescriptor;

/// A struct describing a pointer to a descriptor table (GDT / IDT).
/// This is in a format suitable for giving to 'lgdt' or 'lidt'.
#[repr(C, packed)]
pub struct DescriptorTablePointer<Entry> {
    /// Size of the DT.
    pub limit: u16,
    /// Pointer to the memory region containing the DT.
    pub base: *const Entry,
}

impl<T> DescriptorTablePointer<T> {
    pub fn new(slice: &[T]) -> Self {
        // GDT, LDT, and IDT all expect the limit to be set to "one less".
        // See Intel 3a, Section 3.5.1 "Segment Descriptor Tables" and
        // Section 6.10 "Interrupt Descriptor Table (IDT)".
        let len = slice.len() * size_of::<T>() - 1;
        assert!(len < 0x10000);
        DescriptorTablePointer {
            base: slice.as_ptr(),
            limit: len as u16,
        }
    }
}

impl<Entry> fmt::Debug for DescriptorTablePointer<Entry> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let limit = self.limit;
        let base = self.base;
        write!(f, "DescriptorTablePointer {{ limit: {:?}, base: {:?}", limit, base)
    }
}

/// Load GDT table.
pub unsafe fn lgdt(gdt: &DescriptorTablePointer<SegmentDescriptor>) {
    asm!("lgdt ($0)" :: "r" (gdt) : "memory");
}

/// Load LDT table.
pub unsafe fn lldt(ldt: &DescriptorTablePointer<SegmentDescriptor>) {
    asm!("lldt ($0)" :: "r" (ldt) : "memory");
}

/// Load IDT table.
pub unsafe fn lidt(idt: &DescriptorTablePointer<IdtEntry>) {
    asm!("lidt ($0)" :: "r" (idt) : "memory");
}
