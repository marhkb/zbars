use super::*;
use symbolset::*;

pub struct Symbol   {
    symbol: *const zbar_symbol_s,
}
impl Symbol  {
    /// Creates a new `SymbolSet` from raw data.
    pub(crate) fn from_raw(symbol: *const zbar_symbol_s) -> Option<Self> {
        match !symbol.is_null() {
            true  => {
                let mut symbol = Self { symbol };
                symbol.set_ref(1);
                Some(symbol)
            },
            false => None
        }
    }
    /// Increases or decreases the reference count.
    fn set_ref(&mut self, refs: i32) { unsafe { zbar_symbol_ref(**self, refs) } }

    pub fn symbol_type(&self) -> ZBarSymbolType {
        unsafe { mem::transmute(zbar_symbol_get_type(**self)) }
    }

    /// Returns the decoded data for this `Symbol`
    ///
    /// # Examples
    ///
    /// ```
    /// use zbars::prelude::*;
    ///
    /// let mut image = ZBarImage::from_owned(1, 1, Format::from_label("Y8"), vec![1]).unwrap();
    /// let mut scanner = ImageScanner::builder().build().unwrap();
    /// if let Ok(symbol_set) = scanner.scan_image(&mut image) {
    ///     if let Some(symbol) = symbol_set.first_symbol() {
    ///         println!("{}", symbol.data());
    ///     }
    /// };
    /// ```
    pub fn data(&self) -> &str { unsafe { from_cstr(zbar_symbol_get_data(**self)) } }
    pub fn quality(&self) -> i32 { unsafe { zbar_symbol_get_quality(**self) } }
    /// Retrieve the current cache count
    pub fn count(&self) -> i32 {
        //TODO: Specify what count is
        /*
         * @returns < 0 if symbol is still uncertain.
         * @returns 0 if symbol is newly verified.
         * @returns > 0 for duplicate symbols
        */
        unsafe { zbar_symbol_get_count(**self) }
    }
    pub fn loc_size(&self) -> u32 { unsafe { zbar_symbol_get_loc_size(**self) } }
    pub fn loc_x(&self, index: u32) -> Option<u32> {
        match unsafe { zbar_symbol_get_loc_x(**self, index) } {
            -1 => None,
            x  => Some(x as u32),
        }
    }
    pub fn loc_y(&self, index: u32) -> Option<u32> {
        match unsafe { zbar_symbol_get_loc_y(**self, index) } {
            -1 => None,
            y  => Some(y as u32),
        }
    }
    fn loc(&self, index: u32) -> Option<(u32, u32)> {
        self.loc_x(index).map(|x| (x, self.loc_y(index).unwrap()))
    }
    pub fn next(&self) -> Option<Self> { Self::from_raw(unsafe { zbar_symbol_next(**self) }) }
    pub fn components(&self) -> Option<SymbolSet> {
        SymbolSet::from_raw(unsafe { zbar_symbol_get_components(**self) } )
    }
    pub fn first_component(&self) -> Option<Self> {
        Self::from_raw(unsafe { zbar_symbol_first_component(**self) } )
    }
    /// Returns a xml representation of the `Symbol`.
    pub fn xml(&self) -> &str {
        unsafe {
            let cstr_buf = CString::new("").unwrap();
            from_cstr(
                zbar_symbol_xml(
                    **self,
                    cstr_buf.as_ptr() as *mut *mut i8,
                    &mut 0_u32 as *mut u32
                )
            )
        }
    }

    pub fn polygon(&self) -> Polygon { self.clone().into() }

    #[cfg(feature = "zbar_fork")]
    pub fn configs(&self) -> u32 { unsafe { zbar_symbol_get_configs(**self) } }

    #[cfg(feature = "zbar_fork")]
    pub fn modifiers(&self) -> u32 { unsafe { zbar_symbol_get_modifiers(**self) } }

    #[cfg(feature = "zbar_fork")]
    pub fn orientation(&self) -> ZBarOrientation {
        unsafe { zbar_symbol_get_orientation (**self) }
    }
}
impl Clone for Symbol {
    fn clone(&self) -> Self {
        let mut symbol = Self { symbol: self.symbol };
        symbol.set_ref(1);
        symbol
    }
}
impl Deref for Symbol {
    type Target = *const zbar_symbol_s;
    fn deref(&self) -> &Self::Target { &self.symbol }
}
impl Drop for Symbol {
    fn drop(&mut self) { self.set_ref(-1); }
}

pub struct Polygon {
    symbol: Symbol
}
impl Polygon {
    pub fn point(&self, index: u32) -> Option<(u32, u32)> { self.symbol.loc(index) }
    pub fn iter(&self) -> PolygonIter { self.symbol.clone().into() }
}
impl From<Symbol> for Polygon  {
    fn from(symbol: Symbol) -> Self { Self { symbol } }
}

pub struct PolygonIter {
    symbol: Symbol,
    index: u32,
}
impl From<Symbol> for PolygonIter   {
    fn from(symbol: Symbol) -> Self { PolygonIter { symbol, index: 0 } }
}
impl Iterator for PolygonIter  {
    type Item = (u32, u32);
    fn next(&mut self) -> Option<Self::Item> {
        let next = self.symbol.loc(self.index);
        self.index += 1;
        next
    }
}

#[cfg(test)]
mod test {
    use std::path::Path;
    use super::*;

    #[cfg(feature = "zbar_fork")]
    const XML: &'static str =
        "<symbol type='QR-Code' quality='1' orientation='UP'>\
            <data>\
                <![CDATA[Hello World]]>\
            </data>\
        </symbol>";
    #[cfg(not(feature = "zbar_fork"))]
    const XML: &'static str =
        "<symbol type='QR-Code' quality='1'>\
            <data>\
                <![CDATA[Hello World]]>\
            </data>\
        </symbol>";

    #[test]
    fn test_from_raw() { create_symbol_en(); }

    #[test]
    fn test_from_raw_null() { assert!(Symbol::from_raw(ptr::null()).is_none()); }

    #[test]
    fn test_symbol_type() {
        assert_eq!(create_symbol_en().symbol_type(), ZBarSymbolType::ZBAR_QRCODE);
    }

    #[test]
    fn test_data() { assert_eq!(create_symbol_en().data(), "Hello World"); }

    #[test]
    fn test_quality() { assert!(create_symbol_en().quality() > 0); }

    #[test]
    fn test_count() { assert_eq!(create_symbol_en().count(), 0); }

    #[test]
    fn test_loc_size() {
        assert_eq!(create_symbol_en().loc_size(), 4);
    }

    #[test]
    fn test_loc_x_y() {
        let symbol = create_symbol_en();
        assert_eq!((symbol.loc_x(0).unwrap(), symbol.loc_y(0).unwrap()), (6, 6));
        assert_eq!((symbol.loc_x(1).unwrap(), symbol.loc_y(1).unwrap()), (6, 142));
        assert_eq!((symbol.loc_x(2).unwrap(), symbol.loc_y(2).unwrap()), (142, 142));
        assert_eq!((symbol.loc_x(3).unwrap(), symbol.loc_y(3).unwrap()), (142, 6));
        assert!(symbol.loc_x(4).is_none());
        assert!(symbol.loc_y(4).is_none());
    }

    #[test]
    fn test_loc() {
        let symbol = create_symbol_en();
        assert_eq!(symbol.loc(0).unwrap(), (6, 6));
        assert_eq!(symbol.loc(1).unwrap(), (6, 142));
        assert_eq!(symbol.loc(2).unwrap(), (142, 142));
        assert_eq!(symbol.loc(3).unwrap(), (142, 6));
        assert!(symbol.loc(4).is_none());

    }

    #[test]
    fn test_next() {
        let symbol = create_symbol_multi();
        assert!(symbol.next().is_some());
        assert!(symbol.next().unwrap().next().is_none());
    }

    #[test]
    fn test_components() {
        // TODO: Better Test
        assert!(create_symbol_multi().components().is_none());
    }

    #[test]
    fn test_first_component() {
        // TODO: Better Test
        assert!(create_symbol_multi().first_component().is_none());
    }

    #[test]
    fn test_xml() { assert_eq!(create_symbol_en().xml(), XML); }

    #[test]
    fn test_polygon() {
        let polygon = create_symbol_en().polygon();
        assert_eq!(polygon.point(0).unwrap(), (6, 6));
        assert_eq!(polygon.point(1).unwrap(), (6, 142));
        assert_eq!(polygon.point(2).unwrap(), (142, 142));
        assert_eq!(polygon.point(3).unwrap(), (142, 6));
        assert!(polygon.point(4).is_none());
    }

    #[test]
    fn test_polygon_iter() {
        let mut iter = create_symbol_en().polygon().iter();
        assert_eq!(iter.next().unwrap(), (6, 6));
        assert_eq!(iter.next().unwrap(), (6, 142));
        assert_eq!(iter.next().unwrap(), (142, 142));
        assert_eq!(iter.next().unwrap(), (142, 6));
        assert!(iter.next().is_none());
    }

    #[test]
    #[cfg(feature = "zbar_fork")]
    fn test_configs() {
        // TODO: Better testing
        assert_eq!(create_symbol_en().configs(), 0);
    }

    #[test]
    #[cfg(feature = "zbar_fork")]
    fn test_modifiers() {
        // TODO: Better testing
        assert_eq!(create_symbol_en().modifiers(), 0);
    }

    #[test]
    #[cfg(feature = "zbar_fork")]
    fn orientation() {
        assert_eq!(create_symbol_en().orientation(), ZBarOrientation::ZBAR_ORIENT_UP);
    }

    fn create_symbol_en() -> Symbol {
        create_symbol_set_from("test/qr_hello-world.png").first_symbol().unwrap()
    }

    fn create_symbol_multi() -> Symbol {
        create_symbol_set_from("test/greetings.png").first_symbol().unwrap()
    }

    fn create_symbol_set_from(path: impl AsRef<Path>) -> SymbolSet{
        use prelude::{
            ZBarImage,
            ImageScanner
        };

        let mut image = ZBarImage::from_path(&path).unwrap();

        let mut scanner = ImageScanner::builder()
            .with_cache(false)
            .with_config(ZBarSymbolType::ZBAR_QRCODE, ZBarConfig::ZBAR_CFG_ENABLE, 1)
            .with_config(ZBarSymbolType::ZBAR_CODE128, ZBarConfig::ZBAR_CFG_ENABLE, 1)
            .build()
            .unwrap();
        scanner.scan_image(&mut image).unwrap()
    }
}
