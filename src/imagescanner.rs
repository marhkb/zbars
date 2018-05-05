use super::*;
use image::*;
use symbolset::*;

pub struct ImageScanner {
    scanner: *mut zbar_image_scanner_s,
}
impl ImageScanner {
    pub fn new() -> Self { Self::default() }
    pub fn builder() -> ImageScannerBuilder { ImageScannerBuilder::new() }
    pub fn set_config(&mut self, symbol_type: ZBarSymbolType, config: ZBarConfig, value: i32) -> ZBarResult<()> {
        let result = unsafe {
            zbar_image_scanner_set_config(**self, symbol_type, config, value)
        };
        match result == 0 {
            true  => Ok(()),
            false => Err(result.into())
        }
    }
    pub fn enable_cache(&mut self, enable: bool) {
        unsafe { zbar_image_scanner_enable_cache(**self, enable as i32); }
    }
    pub fn recycle_image(&mut self, image: Option<&mut ZbarImage>) {
        unsafe {
            zbar_image_scanner_recycle_image(**self, image.map_or(::std::ptr::null_mut(), |i| **i))
        }
    }
    pub fn results(&self) -> Option<SymbolSet> {
        SymbolSet::from_raw(unsafe { zbar_image_scanner_get_results(**self) })
    }
    pub fn scan_image(&self, image: &ZbarImage) -> ZBarSimpleResult<Option<SymbolSet>> {
        let result: i32 = unsafe { zbar_scan_image(**self, **image) };
        match result >= 0 {
            true  => Ok(image.symbols()),
            false => Err(result),
        }
    }
}
impl Default for ImageScanner {
    fn default() -> Self {
        let mut scanner = ImageScanner {
            scanner: unsafe { zbar_image_scanner_create() }
        };
        scanner.set_config(ZBarSymbolType::ZBAR_NONE, ZBarConfig::ZBAR_CFG_ENABLE, 0);
        scanner
    }
}
impl Deref for ImageScanner {
    type Target = *mut zbar_image_scanner_s;
    fn deref(&self) -> &<Self as Deref>::Target { &self.scanner }
}
impl Drop for ImageScanner {
    fn drop(&mut self) { unsafe { zbar_image_scanner_destroy(**self) } }
}

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

    pub fn build(&self) -> ImageScanner {
        let mut scanner = ImageScanner::new();
        scanner.enable_cache(self.cache);
        self.config.iter().for_each(|values| {
            scanner.set_config(values.0, values.1, values.2);
        });
        scanner
    }
}


#[cfg(test)]
#[cfg(feature = "from_image")]
mod test {
    extern crate image;

    use symbol::Symbol;
    use super::*;

    #[test]
    fn test_qrcode() {
        let image = ZbarImage::from_path("test/qrcode.png").unwrap();

        let scanner = ImageScannerBuilder::new()
            .with_config(ZBarSymbolType::ZBAR_QRCODE, ZBarConfig::ZBAR_CFG_ENABLE, 1)
            .build();
        scanner.scan_image(&image).unwrap();

        assert_qrcode(image.first_symbol().unwrap());

        let mut iter = image.symbols().unwrap().iter();

        assert_qrcode(iter.next().unwrap());
        assert!(iter.next().is_none());
    }

    #[test]
    fn test_qrcode_disabled() {
        let image = ZbarImage::from_path("test/qrcode.png").unwrap();

        let scanner = ImageScannerBuilder::new()
            .with_config(ZBarSymbolType::ZBAR_QRCODE, ZBarConfig::ZBAR_CFG_ENABLE, 0)
            .build();
        scanner.scan_image(&image).unwrap();

        assert!(image.first_symbol().is_none());
    }

    fn assert_qrcode(symbol: Symbol) {
        assert_eq!(symbol.symbol_type(), ZBarSymbolType::ZBAR_QRCODE);
        assert_eq!(symbol.data(), "https://www.ikimuni.de/");
        assert_eq!(symbol.next().is_none(), true);
    }

    #[test]
    fn test_code128() {
        let image = ZbarImage::from_path("test/code128.gif").unwrap();

        let scanner = ImageScannerBuilder::new()
            .with_config(ZBarSymbolType::ZBAR_CODE128, ZBarConfig::ZBAR_CFG_ENABLE, 1)
            .build();

        scanner.scan_image(&image).unwrap();

        assert_code128(image.first_symbol().unwrap());

        let mut iter = image.symbols().unwrap().iter();

        assert_code128(iter.next().unwrap());
        assert!(iter.next().is_none());
    }

    #[test]
    fn test_code128_disabled() {
        let image = ZbarImage::from_path("test/code128.gif").unwrap();

        let scanner = ImageScannerBuilder::new()
            .with_config(ZBarSymbolType::ZBAR_CODE128, ZBarConfig::ZBAR_CFG_ENABLE, 0)
            .build();

        scanner.scan_image(&image).unwrap();

        assert!(image.first_symbol().is_none());
    }

    fn assert_code128(symbol: Symbol) {
        assert_eq!(symbol.symbol_type(), ZBarSymbolType::ZBAR_CODE128);
        assert_eq!(symbol.data(), "Screwdriver");
        assert_eq!(symbol.next().is_none(), true);
    }

    #[test]
    fn test_recycle_image() {
        let mut image = ZbarImage::from_path("test/code128.gif").unwrap();

        let mut scanner = ImageScannerBuilder::new()
            .with_config(ZBarSymbolType::ZBAR_CODE128, ZBarConfig::ZBAR_CFG_ENABLE, 1)
            .build();

        scanner.scan_image(&image).unwrap();

        scanner.recycle_image(Some(&mut image));
        assert!(image.first_symbol().is_none());
    }
}
