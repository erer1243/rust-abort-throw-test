A Rust program that helps to test the behavior of aborting vs throwing in C++ code called from Rust.

```sh
export LD_LIBRARY_PATH=$PWD/target
cargo run -- abort
# OR
cargo run -- throw
rust-gdb target/debug/x <path to coredump>
# in gdb, run bt to see the backtrace info
```
