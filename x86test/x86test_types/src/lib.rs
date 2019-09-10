#[macro_export]
macro_rules! kassert {
    ($test:expr) => ({
        if !$test {
            sprintln!("kassertion failed: {}, {}:{}:{}", stringify!($test), file!(), line!(), column!());
            unsafe { x86test::outw(0xf4, 0x01); } // exit failure
        }
    });
    ($test:expr, $($arg:tt)+) => ({
        if !$test {
            sprintln!("kassertion failed: {}, {}:{}:{}", format_args!($($arg)+), file!(), line!(), column!());
            #[allow(unused_unsafe)]
            unsafe { x86test::outw(0xf4, 0x01); } // exit failure
        }
    });
}

#[macro_export]
macro_rules! kpanic {
    ($test:expr) => ({
        sprintln!("kpanic: {}, {}:{}:{}", stringify!($test), file!(), line!(), column!());
        unsafe { x86test::outw(0xf4, 0x02); } // exit failure
    });
    ($test:expr, $($arg:tt)+) => ({
        if !$test {
            sprintln!("kpanic: {}, {}:{}:{}", format_args!($($arg)+), file!(), line!(), column!());
            #[allow(unused_unsafe)]
            unsafe { x86test::outw(0xf4, 0x02); } // exit failure
        }
    });
}

pub struct StaticTestFn(pub fn());

pub struct X86TestFn {
    /// Name of test.
    pub name: &'static str,
    /// Ignore this test?
    pub ignore: bool,
    /// Create an identify map of process inside the VM?
    pub identity_map: bool,
    /// Add guest physical memory in this range.
    pub physical_memory: (u64, u64),
    /// When read on ioport_enable.0 return ioport_enable.1 as value.
    /// When write on ioport_enable.0 abort if value was not ioport_enable.1.
    pub ioport_enable: (u16, u32),
    /// Test has a #[should_panic] attribute
    pub should_panic: bool,
    /// Test has a #[should_halt] attribute
    pub should_halt: bool,
    /// Test function we need to execute (in a VM).
    pub testfn: StaticTestFn,
}
