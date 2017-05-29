export CARGO_TARGET_DIR=`pwd`/target

if [[ "$TRAVIS_RUST_VERSION" == "nightly" ]] && [[ "$TRAVIS_OS_NAME" == "linux" ]]; then
    RUST_BACKTRACE=1 travis-cargo build --features "clippy"
else
    RUST_BACKTRACE=1 travis-cargo build
fi

RUST_BACKTRACE=1 travis-cargo test

cargo doc --no-deps
