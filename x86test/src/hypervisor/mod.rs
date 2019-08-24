//! Silly little hypervisor based on KVM
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Write};

use kvm::{Capability, IoDirection, Segment, System, Vcpu, VirtualMachine};
use mmap::{MapOption, MemoryMap};

use crate::X86TestFn;
use x86::bits64::paging::*;
use x86::controlregs::*;

mod vspace;

use crate::hypervisor::vspace::VSpace;

pub(crate) struct PhysicalMemory {
    offset: usize,
    allocated: usize,
    size: usize,
    #[allow(unused)]
    backing_memory: MemoryMap,
}

impl PhysicalMemory {
    /// Allocate a chunk of memory that is handed out as "physical memory"
    /// to allocate page-tables etc.
    pub(crate) fn new(offset: u64) -> PhysicalMemory {
        let size = 4 * (1 << 20);

        let options = [
            MapOption::MapAddr(offset as *const u8),
            MapOption::MapReadable,
            MapOption::MapWritable,
            MapOption::MapExecutable,
        ];
        let mm = MemoryMap::new(size, &options).unwrap();

        PhysicalMemory {
            offset: offset as usize,
            allocated: 0,
            size: size,
            backing_memory: mm,
        }
    }

    fn len(&self) -> usize {
        self.size
    }

    fn alloc_pages(&mut self, how_many: u64) -> *mut u8 {
        let to_allocate = how_many as usize * BASE_PAGE_SIZE;
        if self.allocated + to_allocate > self.size {
            panic!("OOM")
        }

        let ptr = (self.offset + self.allocated) as *mut u8;
        self.allocated += to_allocate;
        ptr
    }
}

pub(crate) struct TestEnvironment<'a> {
    sys: &'a System,
    #[allow(unused)]
    heap: &'a mut PhysicalMemory,
    stack: &'a mut PhysicalMemory,
    vspace: VSpace<'a>,
    vm: VirtualMachine<'a>,
}

impl<'a> TestEnvironment<'a> {
    pub(crate) fn new(
        sys: &'a System,
        stack: &'a mut PhysicalMemory,
        heap: &'a mut PhysicalMemory,
        ptables: &'a mut PhysicalMemory,
        //vspace: VSpace,
    ) -> TestEnvironment<'a> {
        let vspace = VSpace::new(ptables);
        let mut vm = VirtualMachine::create(sys).unwrap();
        // Ensure that the VM supports memory backing with user memory
        assert!(vm.check_capability(Capability::UserMemory) > 0);

        TestEnvironment {
            heap: heap,
            stack: stack,
            vspace: vspace,
            sys: sys,
            vm: vm,
        }
    }

    /// Map the page table memory and stack memory
    pub(crate) fn create_vcpu(&'a mut self, init_fn: VAddr) -> kvm::Vcpu {
        // Map the process
        let f = File::open("/proc/self/maps").unwrap();
        let reader = BufReader::new(f);

        for line in reader.lines() {
            let line = line.unwrap();
            let mut s = line.split(' ');
            let mut s2 = s.next().unwrap().split('-');
            let begin = usize::from_str_radix(s2.next().unwrap(), 16).unwrap();
            let end = usize::from_str_radix(s2.next().unwrap(), 16).unwrap();
            if end <= 0x800000000000 {
                //println!("{:#X} -- {:#X} {}", begin, end, s.next().unwrap());
                let slice = {
                    let begin_ptr: *mut u8 = begin as *const u8 as _;
                    unsafe { ::std::slice::from_raw_parts_mut(begin_ptr, end - begin) }
                };

                //Set-up hypervisor by making all our memory available to the "guest"-test
                self.vm
                    .set_user_memory_region(begin as _, slice, 0)
                    .expect("Can't set user memory region!");

                // Set-up guest page-table by 1:1 mapping everything
                self.vspace.map_identity(
                    PAddr::from(begin),
                    PAddr::from(end),
                    vspace::MapAction::ReadWriteExecuteKernel,
                );
            }
        }

        let mut vcpu = Vcpu::create(&mut self.vm).unwrap();
        // Set supported CPUID (KVM fails without doing this)
        let mut cpuid = self.sys.get_supported_cpuid().unwrap();
        vcpu.set_cpuid2(&mut cpuid).unwrap();

        // Setup the special registers
        let mut sregs = vcpu.get_sregs().unwrap();

        // Set the code segment to have base 0, limit 4GB (flat segmentation)
        let segment_template = Segment {
            base: 0x0,
            limit: 0xffffffff,
            selector: 0,
            _type: 0,
            present: 0,
            dpl: 0,
            db: 1,
            s: 0,
            l: 0,
            g: 1,
            avl: 0,
            ..Default::default()
        };

        sregs.cs = Segment {
            selector: 0x8,
            _type: 0xb,
            present: 1,
            db: 0,
            s: 1,
            l: 1,
            ..segment_template
        };
        sregs.ss = Segment { ..segment_template };
        sregs.ds = Segment { ..segment_template };
        sregs.es = Segment { ..segment_template };
        sregs.fs = Segment { ..segment_template };
        sregs.gs = Segment { ..segment_template };

        // We don't need to populate the GDT if we have our segments setup
        // cr0 - protected mode on, paging enabled
        sregs.cr0 = (Cr0::CR0_PROTECTED_MODE
            | Cr0::CR0_MONITOR_COPROCESSOR
            | Cr0::CR0_EXTENSION_TYPE
            | Cr0::CR0_ENABLE_PAGING
            | Cr0::CR0_NUMERIC_ERROR
            | Cr0::CR0_WRITE_PROTECT
            | Cr0::CR0_ALIGNMENT_MASK
            | Cr0::CR0_ENABLE_PAGING)
            .bits() as u64;
        sregs.cr3 = 0x9000000;
        sregs.cr4 = (Cr4::CR4_ENABLE_PSE
            | Cr4::CR4_ENABLE_PAE
            | Cr4::CR4_ENABLE_GLOBAL_PAGES
            | Cr4::CR4_ENABLE_SSE
            | Cr4::CR4_UNMASKED_SSE
            | Cr4::CR4_ENABLE_OS_XSAVE
            | Cr4::CR4_ENABLE_SMEP
            | Cr4::CR4_ENABLE_VME)
            .bits() as u64;
        sregs.efer = 0xd01; // XXX

        // Set the special registers
        vcpu.set_sregs(&sregs).unwrap();

        let mut regs = vcpu.get_regs().unwrap();

        // Set the instruction and stack pointer
        let stack_size = self.stack.len();
        regs.rip = init_fn.as_usize() as u64;
        regs.rflags = 0x246; // XXX
        regs.rsp = 0x3000000 + stack_size as u64 - 8;
        regs.rbp = regs.rsp;

        vcpu.set_regs(&regs).unwrap();

        vcpu
    }
}

pub(crate) struct SerialPrinter {
    buffer: String,
}

impl Write for SerialPrinter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        assert!(buf.len() == 1);
        self.buffer.push(buf[0] as char);
        match buf[0] as char {
            '\n' => {
                std::io::stdout().write(self.buffer.as_bytes())?;
                self.buffer.clear();
            }
            _ => {}
        }

        Ok(1)
    }

    fn flush(&mut self) -> io::Result<()> {
        std::io::stdout().write(self.buffer.as_bytes())?;
        self.buffer.clear();
        Ok(())
    }
}

impl SerialPrinter {
    pub(crate) fn new() -> SerialPrinter {
        SerialPrinter {
            buffer: String::new(),
        }
    }
}

#[derive(Debug)]
pub(crate) enum IoHandleError {
    UnexpectedWrite(u16, u32),
    UnexpectedRead(u16),
}

pub(crate) enum IoHandleStatus {
    Handled,
    TestSuccessful,
    TestPanic(u8),
}

pub(crate) fn handle_ioexit(
    meta: &X86TestFn,
    cpu: &mut Vcpu,
    run: &kvm::Run,
    printer: &mut SerialPrinter,
) -> Result<IoHandleStatus, IoHandleError> {
    let io = unsafe { *run.io() };

    match io.direction {
        IoDirection::In => {
            let mut regs = cpu.get_regs().unwrap();
            if io.port == 0x3fd {
                regs.rax = 0x20; // Mark serial line ready to write
                cpu.set_regs(&regs).unwrap();
                return Ok(IoHandleStatus::Handled);
            } else if io.port == 0x2fd {
                regs.rax = 0x20; // Mark serial line ready to write
                cpu.set_regs(&regs).unwrap();
                return Ok(IoHandleStatus::Handled);
            } else if io.port == meta.ioport_reads.0 {
                regs.rax = meta.ioport_reads.1 as u64;
                cpu.set_regs(&regs).unwrap();
                return Ok(IoHandleStatus::Handled);
            }
            return Err(IoHandleError::UnexpectedRead(io.port));
        }
        IoDirection::Out => {
            let regs = cpu.get_regs().unwrap();
            if io.port == 0x3f8 {
                printer.write(&[regs.rax as u8]).ok();
                return Ok(IoHandleStatus::Handled);
            } else if io.port == 0x2f8 {
                // ignore the other serial port that klogger outputs to by default
                return Ok(IoHandleStatus::Handled);
            } else if io.port == 0xf4 && regs.rax as u8 == 0x0 {
                // Magic shutdown command for exiting the test.
                // The line unsafe { x86::shared::io::outw(0xf4, 0x00); }
                // is automatically inserted at the end of every test!
                return Ok(IoHandleStatus::TestSuccessful);
            } else if io.port == 0xf4 {
                return Ok(IoHandleStatus::TestPanic(regs.rax as u8));
            }

            return Err(IoHandleError::UnexpectedWrite(io.port, regs.rax as u32));
        }
    };
}
