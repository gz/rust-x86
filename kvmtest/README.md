# kvmtest custom test runner

kvmtest is a custom test runner that allows you to write unit tests which use
privileged (x86) instructions. 

It achieves that as follows: for every unit test it creates a tiny VM (using
kvm) which mirrors the address space of the current test process inside the
guest VM. Next the VM is initialized and jumps to the unit test function which
is now executed in guest ring 0 (and here you can use all your fancy
instructions). Finally, once the test returns (or panics), control is
transferred back from the VM to our test runner.

Funky? Yes. 

Is it hard to use? No! It integrates neatly with rust thanks to the rust custom
test framework and procedural macros. See the example below. 

Does it work? It has limitations (this is expected you're running on bare-metal
x86), so don't expect much infrastructure. For panic and assert you have to use
special versions, also you can't use anything that does system calls (like
println!, but a custom sprintln! macro is provided).

## An example

This is particularly helpful to test the x86 crate. For example say we have a function
like this:

```rust
/// Read 16 bits from port
#[inline]
pub unsafe fn inw(port: u16) -> u16 {
    let ret: u16;
    asm!("inw %dx, %ax" : "={ax}"(ret) : "{dx}"(port) :: "volatile");
    ret
}
```

The problem with `inw` is that it needs IO privilege level in E/RFlags to not
cause an exception (and as a result crash the process). A regular Linux process
will not run with this privilege level, however we can now write a kvmtest:

```rust
#[kvmtest(ioport(0x1, 0xfe))]
fn check_inw_port_read() {
    unsafe {
        kassert!(
            x86::io::inw(0x1) == 0xfe,
            "`inw` instruction didn't read the correct value"
        );
    }
}
```

A few things are happening here that warrant some explaining:

First, instead of `#[test]` we used `#[kvmtest]` to tell the system we don't
want to use regular unit tests. `kvmtest` supports a few arguments (more on
that later), here we just tell the "hypervisor" of the test runner to install
an ioport with port number 1 that shall always return 0xfe when being read.
Next, comes our function declaration -- nothing special here -- followed by
unsafe, just because `inw` is unsafe. Finally, we use `kassert!`, a custom assert
macro that works in guest ring 0 for our hypervisor, to check that `inw` does
the right thing.

You'll find more example tests among the [x86 tests](../tests/kvm/bin.rs).
Note that running a kvmtest currently works only on Linux and requires some linking magic.
Setting `RUSTFLAGS="-C relocation-model=dynamic-no-pic -C code-model=kernel"` should do.
I expect the custom `RUSTFLAGS` to not be necessary in the future.

## kvmtest reference

The kvmtest attribute currently supports the following parameters:

* ioport(port, val): Reads to `port` will return `val`.
* ram(from, to): Adds physical memory in address range `from` -- `to`
* `#[should_panic]`: Can be added if a test is expected to panic.

## Code Organization

* [kvmtest_macro](kvmtest_macro): contains a procedural macro implementation of `kvmtest`.
* [kvmtest_types](kvmtest_types): contains implementations of kassert, kpanic and the KvmTestFn struct.
* [src](src): contains the custom test runner implementation.
