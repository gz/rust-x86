//! Data structures and functions used by 32-bit mode.

pub mod eflags;
pub mod segmentation;
pub mod task;

#[cfg(target_arch = "x86")]
#[inline(always)]
pub unsafe fn stack_jmp(stack: *mut (), ip: *const ()) -> ! {
    llvm_asm!("mov esp, $0; jmp $1" :: "rg"(stack), "r"(ip) :: "volatile", "intel");
    loop {}
}
