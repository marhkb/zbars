extern crate zbar_rs;

use zbar_rs::prelude::*;

pub fn main() {
    let mut image = ZBarImage::from_path("test/qrcode.png")
        .expect("unable to create image");

    let image_scanner = ImageScanner::builder()
        .with_config(ZBarSymbolType::ZBAR_QRCODE, ZBarConfig::ZBAR_CFG_ENABLE, 1)
        .build();

    let symbol_set = image_scanner.scan_image(&mut image)
        .expect("error on scanning image");

    match symbol_set {
        Some(symbol_set) => {
            symbol_set.iter()
                .for_each(|symbol| {
                    println!("symbol decoded: {}", symbol.data());
                    symbol.polygon().iter()
                        .enumerate()
                        .for_each(|(i, point)| {
                            println!("{}. point: {:?}", i, point);
                        })
                })
        }
        None => println!("no symbols decoded")
    }

}
