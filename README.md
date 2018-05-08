# zbars
##### Renamed from zbar-rs

[![crates.io version][1]][2] [![build status][3]][4]
[![downloads][5]][6] [![docs.rs docs][7]][8]

# High-level rust bindings zo the zbar library
Just started implementing a high-level rust binding to zbar barcode scanner library.  
Some things already work, but there is still a lot to do. So don't expect this to work without flaws.
And expect things to break!

# Building and Running
You need `zbar native library` in order to build `zbars`.

## Linux
On Linux you can simply install zbar development package. The build script uses 
`pkg-config` to probe for zbar native library.

### Ubuntu:
  
    # apt install libzbar-dev

### Arch Linux

    # pacman -S zbar

Feature `zbar_fork_if_available` is enabled by default and builds the crate against
zbar `0.2` which is a more recent fork (https://github.com/procxx/zbar) if found by `pkg-config`.

Nothing special to consider when running your binary on Linux.

## Windows
Building on Windows is a little bit awkward. I only tested it on x64 with MSVC toolchain.  
You can build [ZBar Visual Studio project](https://github.com/dani4/ZBarWin64) and set
following environment variables in order to build:
```
1. ZBAR_PROJ_DIR="project directory"
2. ZBAR_BUILD_DIR="build output directory"
```

In order to run you also need to compile [libiconv](https://www.gnu.org/software/libiconv/) or download `libiconv.dll` from somewhere.
Both `libzbar64-0.dll` from output directory and `libiconv.dll` have to be copied to the directory where
your binary is.

# Usage
Scan an image for QR codes:
```
extern crate zbars;

use zbars::prelude::*;

pub fn main() {
    let mut image = ZBarImage::from_path("test/qrcode.png")
        .expect("unable to create image");

    let image_scanner = ImageScanner::builder()
        .with_config(ZBarSymbolType::ZBAR_QRCODE, ZBarConfig::ZBAR_CFG_ENABLE, 1)
        .build();

    let symbol_set = image_scanner.scan_image(&mut image)
        .expect("error on scanning image");

    symbol_set.iter()
        .for_each(|symbol| {
            println!("symbol decoded: {}", symbol.data());
            symbol.polygon().iter()
                .enumerate()
                .for_each(|(i, point)| {
                    println!("{}. point: {:?}", i, point);
                })
        });
}
```

[1]: https://img.shields.io/crates/v/zbars.svg?style=flat-square
[2]: https://crates.io/crates/zbars
[3]: https://img.shields.io/travis/marhkb/zbars.svg?style=flat-square
[4]: https://travis-ci.org/marhkb/zbars
[5]: https://img.shields.io/crates/d/zbars.svg?style=flat-square
[6]: https://crates.io/crates/zbars
[7]: https://docs.rs/zbars/badge.svg
[8]: https://docs.rs/crate/zbars
