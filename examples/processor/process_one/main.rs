extern crate zbar_rs;

use zbar_rs::prelude::*;

pub fn main() {
    let mut processor = Processor::builder()
        .threaded(true)
        //enable qrcode decoding
        .with_config(ZBarSymbolType::ZBAR_QRCODE, ZBarConfig::ZBAR_CFG_ENABLE, 1)
        //enable code128 decoding
        .with_config(ZBarSymbolType::ZBAR_CODE128, ZBarConfig::ZBAR_CFG_ENABLE, 1)
        .build();

    // initialize video
    processor.init("/dev/video0", true).unwrap();

    // show video
    processor.set_visible(true).unwrap();

    // wait forever (-1) until symbol is decoded
    processor.process_one(-1);

    // retrieve decoded results
    let symbol = processor.get_results().unwrap().first_symbol().unwrap();

    // print symbol data
    println!("{}", symbol.data());
}
