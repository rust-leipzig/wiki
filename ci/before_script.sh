pip install 'travis-cargo<0.2' --user --verbose

export PATH=$HOME/.local/bin:$PATH
export PATH=$HOME/Library/Python/2.7/bin:$PATH

cargo install cargo-kcov

if [[ "$TRAVIS_RUST_VERSION" == "nightly" ]] then
  rustup component add clippy --toolchain=nightly || cargo install --git https://github.com/rust-lang/rust-clippy/ --force clippy
fi
