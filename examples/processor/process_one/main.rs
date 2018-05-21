extern crate zbars;

use zbars::prelude::*;

pub fn main() {
    let mut processor = Processor::builder()
        .threaded(true)
        //enable qrcode decoding
        .with_config(ZBarSymbolType::ZBAR_QRCODE, ZBarConfig::ZBAR_CFG_ENABLE, 1)
        //enable code128 decoding
        .with_config(ZBarSymbolType::ZBAR_CODE128, ZBarConfig::ZBAR_CFG_ENABLE, 1)
        .with_config(ZBarSymbolType::ZBAR_EAN13, ZBarConfig::ZBAR_CFG_ENABLE, 1)
        .with_size(Some((700, 700)))
        .build()
        .unwrap();

    let r = processor.request_iomode(0).unwrap();
//    println!("{}", r);

    // initialize video (system dependent!)
    processor.init("/dev/video0", true).unwrap();

    // show video
    processor.set_visible(true).unwrap();

    match processor.process_one(5000) {
        Ok(result) => match result {
            Some(symbols) => println!("{}", symbols.first_symbol().unwrap().data()),
            None          => println!("timeout expired"),
        }
        Err(e)     => println!("error while processing: {}", e),
    }
}
