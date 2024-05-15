# Akame

Akame is a programming lanugage I made to learn more about compilers and LLVM

There's no release for the compiler, so you will need cargo to build it. You can install it here: https://www.rust-lang.org/tools/install

You'll also need a `clang` compiler available on your path

To run the example from the rust folder:
```
cargo run -- compile -o hello.out --  examples/codegen.ak
./hello.out "Hello World"
```
