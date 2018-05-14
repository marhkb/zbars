#[cfg(target_os = "linux")]
#[cfg(test)]
mod test_mem {

    extern crate zbars;
    extern crate procinfo;

    use self::zbars::prelude::*;

    const N: usize = 100000;

    #[test]
    fn test_mem_image_from_buf() {
        let mem_before = mem();
        for _ in 0..N {
            ZBarImage::from_owned(
                500, 500, Format::from_label("Y800"), vec![0; 500 * 500]
            ).unwrap();
        }
        assert_mem(mem_before, N);
    }

    #[test]
    fn test_mem_image_from_slice() {
        let mem_before = mem();
        for _ in 0..N {
            let buf = vec![0; 500 * 500];
            let buf_slice = buf.as_slice();
            ZBarImage::from_borrowed(
                500, 500, Format::from_label("Y800"), &buf_slice
            ).unwrap();
        }
        assert_mem(mem_before, N);
    }

    #[test]
    fn test_mem_decode_image() {
        assert_eq!(loop_decode().first_symbol().unwrap().data(), "Hello World")
    }

    #[test]
    fn test_symbol_xml() {
        let mut image = ZBarImage::from_path("test/qr_hello-world.png").unwrap();
        let mut scanner = ImageScanner::builder()
            .with_config(ZBarSymbolType::ZBAR_QRCODE, ZBarConfig::ZBAR_CFG_ENABLE, 1)
            .build()
            .unwrap();
        let symbols = scanner.scan_image(&mut image).unwrap();

        let mem_before = mem();

        for _ in 0..N*10 {
            let xml = symbols.first_symbol().unwrap().xml();
        }

        assert_mem(mem_before, N);
    }

    fn loop_decode() -> SymbolSet {
        let mut image = ZBarImage::from_path("test/qr_hello-world.png").unwrap();
        let mut scanner = ImageScanner::builder()
            .with_config(ZBarSymbolType::ZBAR_QRCODE, ZBarConfig::ZBAR_CFG_ENABLE, 1)
            .build()
            .unwrap();

        let mem_before = mem();

        for _ in 0..N / 1000 {
            let symbols = scanner.scan_image(&mut image).unwrap();
        }

        assert_mem(mem_before, N);

        scanner.scan_image(&mut image).unwrap()
    }

    fn mem() -> usize { procinfo::pid::statm_self().unwrap().resident }

    fn assert_mem(mem_before: usize, n: usize) {
        let mem_after = mem();
        // Allow memory to grow by 8MB, but not more.
        assert!(
            mem_after < mem_before + 8 * 1024,
            "Memory usage at start is {}KB, memory usage after {} loops is {}KB",
            mem_before, n, mem_after
        );
    }
}
