curl -sSf https://static.rust-lang.org/dist/rust-nightly-%TARGET%.exe -o rust-nightly-%TARGET%.exe

rust-nightly-%TARGET%.exe /VERYSILENT /NORESTART /DIR="C:\Program Files (x86)\Rust"

set PATH=%PATH%;C:\Program Files (x86)\Rust\bin
if defined MSYS_BITS set PATH=C:\msys64\mingw%MSYS_BITS%\bin;C:\msys64\usr\bin;%PATH%

set CARGO_TARGET_DIR=%APPVEYOR_BUILD_FOLDER%\target
rustc -V
cargo -V

git submodule update --init --recursive
