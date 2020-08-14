export CARGO_TARGET_DIR=`pwd`/target

if [[ "$TRAVIS_RUST_VERSION" == "nightly" ]] && [[ "$TRAVIS_OS_NAME" == "linux" ]]; then
  cargo clippy
  RUST_BACKTRACE=1 cargo build
else
    RUST_BACKTRACE=1 cargo build
fi

RUST_BACKTRACE=1 cargo test

cargo doc --no-deps
