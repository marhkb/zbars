# zbars
##### Renamed from zbar-rs

[![crates.io version][1]][2] [![build status][3]][4]
[![downloads][5]][6] [![docs.rs docs][7]][8]

High-level rust bindings zo the zbar library

Just started implementing a high-level rust binding to zbar barcode scanner library.
Some things already work, but there is still a lot to do. So don't expect this to work without flaws.
And expect things to break!

You need zbar native library in order to build zbars.

On Linux you can simply install zbar development package.

On Windows x64 you can build zbar from https://github.com/dani4/ZBarWin64
and set define 2 environment variables
1. ZBAR_PROJ_DIR => the root directory of the project
2. ZBAR_BUILD_DIR => where the output files from the build are put into

You also need to compile libiconv or download libiconv.dll.
In order to run you either need to copy libzbar-64-0.dll from output directory
and libiconv.dll to your working directory

Examples and documentation will follow in the next weeks.

[1]: https://img.shields.io/crates/v/zbars.svg?style=flat-square
[2]: https://crates.io/crates/zbars
[3]: https://img.shields.io/travis/marhkb/zbars.svg?style=flat-square
[4]: https://travis-ci.org/marhkb/zbars
[5]: https://img.shields.io/crates/d/zbars.svg?style=flat-square
[6]: https://crates.io/crates/zbars
[7]: https://docs.rs/zbars/badge.svg
[8]: https://docs.rs/crate/zbars
