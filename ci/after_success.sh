if [[ "$TRAVIS_RUST_VERSION" == "stable" ]] && [[ "$TRAVIS_OS_NAME" == "linux" ]]; then
    travis-cargo doc-upload;
fi

if [[ "$TRAVIS_RUST_VERSION" == "nightly" ]] && [[ "$TRAVIS_OS_NAME" == "linux" ]]; then
    cargo kcov --print-install-kcov-sh | sh;
fi

if [[ "$TRAVIS_RUST_VERSION" == "nightly" ]] && [[ "$TRAVIS_OS_NAME" == "linux" ]]; then
    cargo kcov --coveralls --kcov ./kcov-33/build/src/kcov;
fi
