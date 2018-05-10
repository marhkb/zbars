extern crate zbars;

use zbars::prelude::*;

pub fn main() {
    let mut processor = Processor::builder()
        .threaded(true)
        //enable qrcode decoding
        .with_config(ZBarSymbolType::ZBAR_QRCODE, ZBarConfig::ZBAR_CFG_ENABLE, 1)
        //enable code128 decoding
        .with_config(ZBarSymbolType::ZBAR_CODE128, ZBarConfig::ZBAR_CFG_ENABLE, 1)
        .build()
        .unwrap();

    // initialize video (system dependent!)
    processor.init("/dev/video0", true).unwrap();

    // show video
    processor.set_visible(true).unwrap();

    // wait forever (-1) until symbol is decoded
    processor.process_one(-1);

    // retrieve decoded results
    let symbols = processor.get_results().unwrap();
    println!("{}", symbols.first_symbol().unwrap().data());
}
