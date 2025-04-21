# Rust <> C++ binding sample

Check add-cpp for C++ side code, just implements two add functions. This setup:
- Uses CMake for build in C++ side
- Bindgen for Rust wrapper generation (needs Rust 1.82+)

For C++ side, you can run a sample program, as usual:

~~~~
mkdir build
cd build
cmake ..
make
~~~~

You can also run the rust side:

~~~~
cargo build
cargo run
~~~~

Note: For some reason bindgen targetting is broken (?), so you will get errors
in code generation if your rust version is below 1.82 (they added an additional keyword)