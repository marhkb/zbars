use {
    ffi,
    image::ZBarImage,
    symbol_set::ZBarSymbolSet,
    ZBarConfig,
    ZBarErrorType,
    ZBarResult,
    ZBarSymbolType
};
use std::ptr;

pub struct ZBarImageScanner {
    pub(crate) scanner: *mut ffi::zbar_image_scanner_s,
}
impl ZBarImageScanner {
    pub fn new() -> Self { Self::default() }
    pub fn builder() -> ImageScannerBuilder { ImageScannerBuilder::new() }
    pub fn set_config(
        &self, symbol_type: ZBarSymbolType,
        config: ZBarConfig,
        value: i32) -> ZBarResult<()>
    {
        match unsafe { ffi::zbar_image_scanner_set_config(self.scanner, symbol_type, config, value) } {
            0 => Ok(()),
            e => Err(e.into())
        }
    }
    pub fn enable_cache(&self, enable: bool) {
        unsafe { ffi::zbar_image_scanner_enable_cache(self.scanner, enable as i32); }
    }
    pub fn recycle_image<T>(&self, image: &ZBarImage<T>) {
        unsafe { ffi::zbar_image_scanner_recycle_image(self.scanner, image.image()) }
    }
    pub fn results(&self) -> Option<ZBarSymbolSet> {
        ZBarSymbolSet::from_raw(
            unsafe { ffi::zbar_image_scanner_get_results(self.scanner) }, ptr::null_mut()
        )
    }
    pub fn scan_image<T>(&self, image: &ZBarImage<T>) -> ZBarResult<ZBarSymbolSet> {
        match unsafe { ffi::zbar_scan_image(self.scanner, image.image()) } {
            -1 => Err(ZBarErrorType::Simple(-1)),
            // symbols can be unwrapped because image is surely scanned
            _  => Ok(image.symbols().unwrap()),
        }
    }
}

unsafe impl Send for ZBarImageScanner {}

impl Default for ZBarImageScanner {
    fn default() -> Self {
        let scanner = ZBarImageScanner { scanner: unsafe { ffi::zbar_image_scanner_create() } };
        // safe to unwrap here
        scanner.set_config(ZBarSymbolType::ZBAR_NONE, ZBarConfig::ZBAR_CFG_ENABLE, 0).unwrap();
        scanner
    }
}
impl Drop for ZBarImageScanner {
    fn drop(&mut self) { unsafe { ffi::zbar_image_scanner_destroy(self.scanner) } }
}

#[derive(Default)]
pub struct ImageScannerBuilder {
    cache: bool,
    config: Vec<(ZBarSymbolType, ZBarConfig, i32)>,
}
impl ImageScannerBuilder {
    pub fn new() -> Self {
        Self { cache: false, config: vec![], }
    }
    pub fn with_config(&mut self, symbol_type: ZBarSymbolType, config: ZBarConfig, value: i32) -> &mut Self {
        self.config.push((symbol_type, config, value)); self
    }
    pub fn with_cache(&mut self, cache: bool) -> &mut Self { self.cache = cache; self }

    pub fn build(&self) -> ZBarResult<ZBarImageScanner> {
        let scanner = ZBarImageScanner::new();

        self.config
            .iter()
            .try_for_each(|v| scanner.set_config(v.0, v.1, v.2))
            .map(|_| {
                scanner.enable_cache(self.cache);
                scanner
            })
    }
}

#[cfg(test)]
#[cfg(feature = "from_image")]
mod test {
    use super::*;
    use symbol::ZBarSymbol;

    #[test]
    fn test_qrcode() {
        let image = ZBarImage::from_path("test/qr_hello-world.png").unwrap();

        let scanner = ImageScannerBuilder::new()
            .with_config(ZBarSymbolType::ZBAR_QRCODE, ZBarConfig::ZBAR_CFG_ENABLE, 1)
            .build()
            .unwrap();
        scanner.scan_image(&image).unwrap();

        assert_qrcode(image.first_symbol().unwrap());

        let symbols = image.symbols().unwrap();
        let mut iter = symbols.iter();

        assert_qrcode(iter.next().unwrap());
        assert!(iter.next().is_none());
    }

    #[test]
    fn test_qrcode_disabled() {
        let image = ZBarImage::from_path("test/qr_hello-world.png").unwrap();

        let scanner = ImageScannerBuilder::new()
            .with_config(ZBarSymbolType::ZBAR_QRCODE, ZBarConfig::ZBAR_CFG_ENABLE, 0)
            .build()
            .unwrap();
        scanner.scan_image(&image).unwrap();

        assert!(image.first_symbol().is_none());
    }

    fn assert_qrcode(symbol: ZBarSymbol) {
        assert_eq!(symbol.symbol_type(), ZBarSymbolType::ZBAR_QRCODE);
        assert_eq!(symbol.data(), "Hello World");
        assert_eq!(symbol.next().is_none(), true);
    }

    #[test]
    fn test_code128() {
        let image = ZBarImage::from_path("test/code128.gif").unwrap();

        let scanner = ImageScannerBuilder::new()
            .with_config(ZBarSymbolType::ZBAR_CODE128, ZBarConfig::ZBAR_CFG_ENABLE, 1)
            .build()
            .unwrap();
        scanner.scan_image(&image).unwrap();

        assert_code128(image.first_symbol().unwrap());

        let symbols = image.symbols().unwrap();
        let mut iter = symbols.iter();

        assert_code128(iter.next().unwrap());
        assert!(iter.next().is_none());
    }

    #[test]
    fn test_code128_disabled() {
        let image = ZBarImage::from_path("test/code128.gif").unwrap();

        let scanner = ImageScannerBuilder::new()
            .with_config(ZBarSymbolType::ZBAR_CODE128, ZBarConfig::ZBAR_CFG_ENABLE, 0)
            .build()
            .unwrap();
        scanner.scan_image(&image).unwrap();

        assert!(image.first_symbol().is_none());
    }

    fn assert_code128(symbol: ZBarSymbol) {
        assert_eq!(symbol.symbol_type(), ZBarSymbolType::ZBAR_CODE128);
        assert_eq!(symbol.data(), "Screwdriver");
        assert_eq!(symbol.next().is_none(), true);
    }

    #[test]
    fn test_recycle_image() {
        let image = ZBarImage::from_path("test/code128.gif").unwrap();

        let scanner = ImageScannerBuilder::new()
            .with_config(ZBarSymbolType::ZBAR_CODE128, ZBarConfig::ZBAR_CFG_ENABLE, 1)
            .build()
            .unwrap();
        scanner.scan_image(&image).unwrap();

        scanner.recycle_image(&image);
        assert!(image.first_symbol().is_none());
    }

    #[test]
    fn test_get_results() {
        let scanner = ImageScannerBuilder::new()
            .with_config(ZBarSymbolType::ZBAR_CODE128, ZBarConfig::ZBAR_CFG_ENABLE, 1)
            .build()
            .unwrap();
        {
            let image = ZBarImage::from_path("test/code128.gif").unwrap();
            scanner.scan_image(&image).unwrap();
        }

        assert_code128(scanner.results().unwrap().first_symbol().unwrap());
    }
}
