#!/bin/sh

rm target/debug/kvm-1204d1237e8c62b6
export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER="$(dirname $0)/tests/kvm/linker.sh"
cargo test --test kvm -- "$@"
