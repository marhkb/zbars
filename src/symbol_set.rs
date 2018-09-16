use {
    ffi,
    symbol::ZBarSymbol
};
use std::mem;

pub struct ZBarSymbolSet {
    symbol_set: *const ffi::zbar_symbol_set_s,
    image: *mut ffi::zbar_image_s
}
impl ZBarSymbolSet {
    /// Creates a new `SymbolSet` from raw data.
    pub(crate) fn from_raw(
        symbol_set: *const ffi::zbar_symbol_set_s,
        image: *mut ffi::zbar_image_s) -> Option<Self>
    {
        if !symbol_set.is_null() {
            let symbol_set = Self { symbol_set, image };
            if !image.is_null() {
                unsafe { ffi::zbar_image_ref(image, 1) }
            }
            Some(symbol_set)
        } else {
            None
        }
    }

    pub(crate) fn symbol_set(&self) -> *const ffi::zbar_symbol_set_s { self.symbol_set }

    pub fn size(&self) -> i32 { unsafe { ffi::zbar_symbol_set_get_size(self.symbol_set) } }
    /// Returns the first `Symbol` if one is present.
    ///
    /// # Examples
    ///
    /// ```
    /// use zbars::prelude::*;
    ///
    /// let image = ZBarImage::new(1, 1, Format::from_label("Y8"), vec![1]).unwrap();
    /// let scanner = ZBarImageScanner::builder().build().unwrap();
    /// if let Ok(symbol_set) = scanner.scan_image(&image) {
    ///     match symbol_set.first_symbol() {
    ///         Some(symbol) => println!("{}", symbol.data()),
    ///         None         => println!("no symbols in symbol set"),
    ///     }
    /// };
    /// ```
    pub fn first_symbol(&self) -> Option<ZBarSymbol> {
        ZBarSymbol::from_raw(
            unsafe { ffi::zbar_symbol_set_first_symbol(self.symbol_set) }, self.image
        )
    }

    pub fn iter(&self) -> SymbolIter { self.first_symbol().into() }

    #[cfg(feature = "zbar_fork")]
    pub fn first_symbol_unfiltered(&self) -> Option<ZBarSymbol> {
        ZBarSymbol::from_raw(
            unsafe { ffi::zbar_symbol_set_first_unfiltered(self.symbol_set) }, self.image
        )
    }
}

impl Clone for ZBarSymbolSet {
    fn clone(&self) -> Self { Self::from_raw(self.symbol_set, self.image).unwrap() }
}

impl Drop for ZBarSymbolSet {
    fn drop(&mut self) {
        if !self.image.is_null() {
            unsafe { ffi::zbar_image_ref(self.image, -1) }
        }
    }
}

pub struct SymbolIter {
    symbol: Option<ZBarSymbol>,
}
impl From<Option<ZBarSymbol>> for SymbolIter {
    fn from(symbol: Option<ZBarSymbol>) -> Self { Self { symbol } }
}
impl Iterator for SymbolIter {
    type Item = ZBarSymbol;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next = self.symbol.as_ref().and_then(ZBarSymbol::next);
        mem::swap(&mut self.symbol, &mut next);
        next
    }
}

#[cfg(test)]
mod test {
    use prelude::*;
    use std::path::Path;
    use super::*;

    #[test]
    fn test_size() { assert_eq!(create_symbol_set().size(), 2); }

    #[test]
    fn test_first_symbol() {
        assert_eq!(create_symbol_set().first_symbol().unwrap().data(), "Hello World");
    }

    #[test]
    fn test_iter() {
        let mut iter = create_symbol_set().iter();
        assert_eq!(iter.next().unwrap().data(), "Hello World");
        assert_eq!(iter.next().unwrap().data(), "Hallo Welt");
        assert!(iter.next().is_none());
    }

    #[test]
    #[cfg(feature = "zbar_fork")]
    fn test_first_symbol_unfiltered() {
        assert_eq!(create_symbol_set().first_symbol_unfiltered().unwrap().data(), "Hello World");
    }

    fn create_symbol_set() -> ZBarSymbolSet {
        create_symbol_from("test/greetings.png").symbols().unwrap()
    }

    fn create_symbol_from(path: impl AsRef<Path>) -> ZBarImage<Vec<u8>> {
        let image = ZBarImage::from_path(&path).unwrap();

        let scanner = ZBarImageScanner::builder()
            .with_cache(false)
            .with_config(ZBarSymbolType::ZBAR_QRCODE, ZBarConfig::ZBAR_CFG_ENABLE, 1)
            .with_config(ZBarSymbolType::ZBAR_CODE128, ZBarConfig::ZBAR_CFG_ENABLE, 1)
            .build()
            .unwrap();
        scanner.scan_image(&image).unwrap();
        image
    }
}
