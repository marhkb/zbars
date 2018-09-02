extern crate zbars;

use zbars::prelude::*;

pub fn main() {
    let mut image = Image::from_path("test/qr_hello-world.png")
        .expect("unable to create image");

    let mut scanner = ImageScanner::builder()
        .with_config(ZBarSymbolType::ZBAR_QRCODE, ZBarConfig::ZBAR_CFG_ENABLE, 1)
        .build()
        .unwrap();

    let symbol_set = scanner.scan_image(&mut image)
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
