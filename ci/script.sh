# This script takes care of testing your crate

set -ex

# This is the "test phase", tweak it as you see fit
main() {
    cross build --target $TARGET
    cross build --target $TARGET --release

    if [ ! -z $DISABLE_TESTS ]; then
        return
    fi

    # Run user-space tests
    cross test --target $TARGET --features utest
    cross test --target $TARGET --release --features utest

    # Run KVM tests: Disabled for now since travis doesn't support KVM :/ (https://travis-ci.community/t/add-kvm-support/1406)
    # RUSTFLAGS="-C relocation-model=dynamic-no-pic -C code-model=kernel" RUST_BACKTRACE=1 cross test --target $TARGET --features vmtest
}

# we don't run the "test phase" when doing deploys
if [ -z $TRAVIS_TAG ]; then
    main
fi
