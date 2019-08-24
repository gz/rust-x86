use hypervisor::{handle_ioexit, IoHandleStatus, PhysicalMemory, SerialPrinter, TestEnvironment};
use kvm::{Exit, System};
use x86::bits64::paging::VAddr;

use crate::X86TestFn;

/// Start the test harness.
pub fn test_start(ntests: usize) {
    println!("running {} tests (using x86test runner)", ntests)
}

/// Signals that given test is ignored.
pub fn test_ignored(name: &str) {
    println!("test {} ... ignored", name);
}

/// Output before a new test is run.
pub fn test_before_run(name: &str) {
    print!("test {} ... ", name);
}

/// Output when a test is fails.
pub fn test_failed(_name: &str) {
    println!("FAILED");
}

/// Output when a test succeeds.
pub fn test_success(_name: &str) {
    println!("OK");
}

/// Summary display at the end of the test run.
pub fn test_summary(passed: usize, failed: usize, ignored: usize) {
    println!(
        "\ntest result: {} {} passed; {} failed; {} ignored",
        if failed == 0 { "OK" } else { "FAILED" },
        passed,
        failed,
        ignored
    );

    if failed != 0 {
        std::process::exit(101);
    }
}

/// Actual logic to run a list of KVM tests.
pub fn runner(tests: &[&X86TestFn]) {
    test_start(tests.len());

    let mut failed = 0;
    let mut ignored = 0;
    let mut passed = 0;
    for test in tests {
        if test.ignore {
            ignored += 1;
            test_ignored(test.name);
        } else {
            test_before_run(test.name);

            let sys = System::initialize().unwrap();
            let mut stack = PhysicalMemory::new(0x3000000);
            let mut heap = PhysicalMemory::new(0x6000000);
            let mut ptables = PhysicalMemory::new(0x9000000);

            let mut test_environment = TestEnvironment::new(&sys, &mut stack, &mut heap, &mut ptables);
            let mut printer: SerialPrinter = SerialPrinter::new();

            let test_fn_vaddr = VAddr::from_usize(test.testfn.0 as *const () as usize);
            let mut vcpu = test_environment.create_vcpu(test_fn_vaddr);

            let mut vm_is_done = false;
            let mut test_panicked = false;

            while !vm_is_done {
                let run = unsafe { vcpu.run() }.unwrap();
                match run.exit_reason {
                    Exit::Io => {
                        match handle_ioexit(test, &mut vcpu, &run, &mut printer) {
                            Result::Ok(IoHandleStatus::Handled) => { /* Continue */ }
                            Result::Ok(IoHandleStatus::TestSuccessful) => vm_is_done = true,
                            Result::Ok(IoHandleStatus::TestPanic(code)) => {
                                if !test.should_panic {
                                    debug!("IoHandleStatus::TestPanic {} should_panic is {}", code, test.should_panic);
                                }
                                vm_is_done = true;
                                test_panicked = true;
                            }
                            Result::Err(err) => {
                                println!("Test failed due to unexpected IO: {:?}", err);
                                vm_is_done = true;
                                test_panicked = true;
                            }
                        }
                    }
                    Exit::Shutdown => {
                        println!("Exit::Shutdown cpu.get_regs() {:#x}", vcpu.get_regs().unwrap().rip);
                        println!("Exit::Shutdown cpu.get_sregs() {:#?}", vcpu.get_sregs());// 0x7ffff732fad0
                        vm_is_done = true;
                        test_panicked = true;
                    }
                    _ => {
                        test_panicked = true;
                        println!("Unknown exit reason: {:?}", run.exit_reason);
                        break;
                    }
                }
            }

            if test_panicked == test.should_panic {
                passed += 1;
                test_success(test.name);
            } else {
                failed += 1;
                test_failed(test.name);
            }
        }
    }

    test_summary(passed, failed, ignored);
}
