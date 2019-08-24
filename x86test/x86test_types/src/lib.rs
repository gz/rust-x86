#[macro_export]
macro_rules! kassert {
    ($test:expr) => ({
        if !$test {
            sprintln!("kassertion failed: {}, {}:{}:{}", stringify!($test), file!(), line!(), column!());
            unsafe { x86::io::outw(0xf4, 0x01); } // exit failure
        }
    });
    ($test:expr, $($arg:tt)+) => ({
        if !$test {
            sprintln!("kassertion failed: {}, {}:{}:{}", format_args!($($arg)+), file!(), line!(), column!());
            #[allow(unused_unsafe)]
            unsafe { x86::io::outw(0xf4, 0x01); } // exit failure
        }
    });
}

#[macro_export]
macro_rules! kpanic {
    ($test:expr) => ({
        sprintln!("kpanic: {}, {}:{}:{}", stringify!($test), file!(), line!(), column!());
        unsafe { x86::io::outw(0xf4, 0x02); } // exit failure
    });
    ($test:expr, $($arg:tt)+) => ({
        if !$test {
            sprintln!("kpanic: {}, {}:{}:{}", format_args!($($arg)+), file!(), line!(), column!());
            #[allow(unused_unsafe)]
            unsafe { x86::io::outw(0xf4, 0x02); } // exit failure
        }
    });
}

pub struct StaticTestFn(pub fn());

pub struct X86TestFn {
    pub name: &'static str,
    pub ignore: bool,
    pub identity_map: bool,
    pub physical_memory: (u64, u64),
    pub ioport_reads: (u16, u32),
    pub should_panic: bool,
    pub testfn: StaticTestFn,
}
