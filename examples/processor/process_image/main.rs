extern crate zbars;

use std::{
    thread,
    time::Duration,
};
use zbars::prelude::*;

pub fn main() {

    // TODO: Image must be created after Processor::init. Investigate!
//    let mut image = ZBarImage::from_path("test/qrcode.png").unwrap();

    let mut processor = Processor::builder()
        .threaded(true)
        //enable qrcode decoding
        .with_config(ZBarSymbolType::ZBAR_QRCODE, ZBarConfig::ZBAR_CFG_ENABLE, 1)
        //enable code128 decoding
        .with_config(ZBarSymbolType::ZBAR_CODE128, ZBarConfig::ZBAR_CFG_ENABLE, 1)
        .build();

    // initialize video (system dependent!)
    processor.init("/dev/video0", true).unwrap();

    // TODO: Image must be created after Processor::init. Investigate!
    let mut image = ZBarImage::from_path("test/qrcode.png").unwrap();

    // set processor visible in order display the image to process
    processor.set_visible(true).unwrap();

    let symbol = processor.process_image(&mut image).unwrap().first_symbol().unwrap();
    println!("{}", symbol.data());

    // display image for 2 seconds
    thread::sleep(Duration::from_millis(2000));
}
