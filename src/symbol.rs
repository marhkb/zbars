use super::*;
use symbolset::*;
use std::mem;

pub struct Symbol   {
    symbol: *const zbar_symbol_s,
}
impl  Symbol  {
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
    /// Increases the reference count.
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
    /// let mut image = ZBarImage::from_owned(1, 1, Format::from_label_borrowed("Y8"), vec![1]).unwrap();
    /// let mut scanner = ImageScanner::builder().build().unwrap();
    /// if let Ok(symbol_set) = scanner.scan_image(&mut image) {
    ///     if let Some(symbol) = symbol_set.first_symbol() {
    ///         println!("{}", symbol.data());
    ///     }
    /// };
    /// ```
    pub fn data(&self) -> &str {
        unsafe { CStr::from_ptr(zbar_symbol_get_data(**self)).to_str().unwrap() }
    }
    pub fn quality(&self) -> i32 { unsafe { zbar_symbol_get_quality(**self) } }
    // TODO: Test
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
    pub fn next(&self) -> Option<Symbol> { Self::from_raw(unsafe { zbar_symbol_next(**self) }) }
    // TODO: Test
    pub fn components(&self) -> Option<SymbolSet> {
        SymbolSet::from_raw(unsafe { zbar_symbol_get_components(**self) } )
    }
    // TODO: Test
    pub fn first_component(&self) -> Option<Symbol> {
        Self::from_raw(unsafe { zbar_symbol_first_component(**self) } )
    }
    /// Returns a xml representation of the `Symbol`.
    pub fn xml(&self) -> String {
        unsafe {
            let mut cstr_buf = CString::new("").unwrap();
            CStr::from_ptr(
                zbar_symbol_xml(
                    **self,
                    cstr_buf.as_ptr() as *mut *mut i8,
                    &mut 0_u32 as *mut u32
                )
            ).to_str().unwrap().to_owned()
        }
    }

    pub fn polygon(&self) -> SymbolPolygon { SymbolPolygon::from(self) }
}

impl Deref for Symbol {
    type Target = *const zbar_symbol_s;
    fn deref(&self) -> &Self::Target { &self.symbol }
}

impl Drop for Symbol {
    fn drop(&mut self) { self.set_ref(-1); }
}

#[cfg(feature = "zbar_fork")]
pub mod zbar_fork {
    use super::*;

    impl  Symbol   {
        // TODO: Test bitmask
        pub fn configs(&self) -> u32 { unsafe { zbar_symbol_get_configs(**self) } }
        // TODO: Test bitmask
        pub fn modifiers(&self) -> ZBarModifier {
            //TODO: zbar.h says a bitmask is returned but zbar_modifier_e is not a bitmask
            unsafe { ::std::mem::transmute(zbar_symbol_get_modifiers(**self)) }
        }
        // TODO: Test
        pub fn orientation(&self) -> ZBarOrientation { unsafe { zbar_symbol_get_orientation (**self) } }
    }
}

pub struct SymbolPolygon<'a>   {
    symbol: &'a Symbol
}
impl<'a> SymbolPolygon<'a> {
    pub fn point(&self, index: u32) -> Option<(u32, u32)> {
        self.symbol.loc_x(index).map(|x| (x, self.symbol.loc_y(index).unwrap()))
    }
    pub fn iter(&'a self) -> SymbolPolygonIter { self.into() }
}
impl<'a> From<&'a Symbol> for SymbolPolygon<'a>  {
    fn from(symbol: &'a Symbol ) -> Self { Self { symbol } }
}

pub struct SymbolPolygonIter<'a> {
    polygon: &'a SymbolPolygon<'a>,
    index: u32,
}
impl<'a> From<&'a SymbolPolygon<'a>> for SymbolPolygonIter<'a>   {
    fn from(polygon: &'a SymbolPolygon ) -> Self {
        SymbolPolygonIter { polygon, index: 0 }
    }
}
impl<'a> Iterator for SymbolPolygonIter<'a>  {
    type Item = (u32, u32);
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        let next = self.polygon.point(self.index);
        self.index += 1;
        next
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use prelude::*;

    #[test]
    fn test_xml() {
        let mut image = ZBarImage::from_path("test/qrcode.png").unwrap();
        let mut scanner = ImageScanner::builder()
            .with_config(ZBarSymbolType::ZBAR_QRCODE, ZBarConfig::ZBAR_CFG_ENABLE, 1)
            .build()
            .unwrap();

        assert_eq!(
            scanner.scan_image(&mut image).unwrap().first_symbol().unwrap().xml(),
            if cfg!(feature = "zbar_fork") {
                "<symbol type='QR-Code' quality='1' orientation='UP'><data><![CDATA[https://www.ikimuni.de/]]></data></symbol>"
            } else {
                "<symbol type=\'QR-Code\' quality=\'1\'><data><![CDATA[https://www.ikimuni.de/]]></data></symbol>"
            }
        );
    }
}
