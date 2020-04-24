//! Helpers to program the task state segment.
//! See Intel 3a, Chapter 7

pub use crate::segmentation;

/// Returns the current value of the task register.
pub fn tr() -> segmentation::SegmentSelector {
    let segment: u16;
    unsafe { llvm_asm!("str $0" : "=r" (segment) ) };
    segmentation::SegmentSelector::from_raw(segment)
}

/// Loads the task register.
pub unsafe fn load_tr(sel: segmentation::SegmentSelector) {
    llvm_asm!("ltr $0" :: "r" (sel.bits()));
}
