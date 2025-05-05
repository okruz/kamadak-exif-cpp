# kamadak-exif-cpp
This repository contains C/C++ bindings for kamadak-exif. 


## License
The contents of this repository are published under The Unlicense. Note, however, that the kamadak-exif crate and its dependencies are published under a BSD 2-Clause license, as of writing this (this is not legal advice, do your own research).

## How to build
Run `cargo build --profile=release` and copy the shared object form `target/release/libkamadak_exif_cpp.so` to the main directory. Then compile the test application via `g++ -std=c++17 -I . -L . -o test test.cpp -lkamadak_exif_cpp` and run via `LD_LIBRARY_PATH=$(pwd) ./test`.
