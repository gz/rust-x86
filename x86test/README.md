# x86test custom test runner

x86test is a custom test runner that allows you to write unit tests which use
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

This is particularly helpful to test the [x86 crate](https://github.com/gz/rust-x86). 
For example say we have a function like this:

```rust
/// Read 16 bits from port
#[inline]
pub unsafe fn inw(port: u16) -> u16 {
    let ret: u16;
    asm!("inw %dx, %ax", in("dx") port, out("ax") ret, options(att_syntax));
    ret
}
```

The problem with `inw` is that it needs IO privilege level in E/RFlags to not
cause an exception (and as a result crash the process). A regular Linux process
will not run with this privilege level, however we can now write a x86test:

```rust
#[x86test(ioport(0x1, 0xfe))]
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

First, instead of `#[test]` we used `#[x86test]` to tell the system we don't
want to use regular unit tests. `x86test` supports a few arguments (more on
that later), here we just tell the "hypervisor" of the test runner to install
an ioport with port number 1 that shall always return 0xfe when being read.
Next, comes our function declaration -- nothing special here -- followed by
unsafe, just because `inw` is unsafe. Finally, we use `kassert!`, a custom assert
macro that works in guest ring 0 for our hypervisor, to check that `inw` does
the right thing.

You'll find more example tests among the [x86 tests](../tests/kvm/bin.rs).
Note that running a x86test currently works only on Linux and requires some linking magic.
Setting `RUSTFLAGS="-C relocation-model=dynamic-no-pic -C code-model=kernel"` should do.
I expect the custom `RUSTFLAGS` to not be necessary in the future.

## x86test reference

The x86test attribute currently supports the following parameters:

* `ioport(port, val)`: Reads to `port` will return `val`, writes to `port` other than `val` will fail the test.
* `ram(from, to)`: Adds physical memory in address range `from` -- `to`
* `should_halt`: To tell the hypervisor that the test will halt (note: use like this `#[x86test(should_halt)]`).
* `#[should_panic]`: Can be added if a test is expected to panic.

## Code Organization

* [x86test_macro](x86test_macro): contains a procedural macro implementation of `x86test`.
* [x86test_types](x86test_types): contains implementations of kassert, kpanic and the X86TestFn struct.
* [src](src): contains the custom test runner implementation.

## Updating

Should be done in the following order:

* Release new version of `x86test-types`
* Release new version of `x86test-macro` (adjust version dependency of x86test-types)
* Release new version of `x86test` (adjust version dependency of x86test-types and x86test-macro)
* Tag with `git tag x86test-0.0.x`
