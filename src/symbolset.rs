use super::*;
use symbol::*;
use std::marker::PhantomData;

pub struct SymbolSet<'a> {
    symbol_set: *const zbar_symbol_set_s,
    phantom: PhantomData<&'a ()>,
}
impl<'a> SymbolSet<'a> {
    pub fn from_raw(symbol_set: *const zbar_symbol_set_s) -> Option<Self> {
        match symbol_set.is_null() {
            true  => None,
            false => Some(Self { symbol_set, phantom: PhantomData })
        }
    }
    pub fn size(&self) -> i32 { unsafe { zbar_symbol_set_get_size(**self) } }
    /// Returns the first `Symbol` if one is present.
    ///
    /// # Examples
    ///
    /// ```
    /// use zbars::prelude::*;
    /// use std::borrow::Cow;
    ///
    ///
    /// let mut image = ZBarImage::from_owned(1, 1, &Format::from_label(Cow::Borrowed("Y8")), vec![1]).unwrap();
    /// let mut scanner = ImageScanner::builder().build().unwrap();
    /// if let Ok(symbol_set) = scanner.scan_image(&mut image) {
    ///     match symbol_set.first_symbol() {
    ///         Some(symbol) => println!("{}", symbol.data()),
    ///         None         => println!("no symbols in symbol set"),
    ///     }
    /// };
    /// ```
    ///
    /// # Code that should not compile
    ///
    /// ```compile_fail
    /// use zbars::prelude::*;
    /// use std::borrow::Cow;
    ///
    /// let mut scanner = ImageScanner::builder().build().unwrap();
    ///
    /// let first = {
    ///     let mut image = ZBarImage::from_owned(1, 1, &Format::from_label(Cow::Borrowed("Y8")), vec![1]).unwrap();
    ///     scanner.scan_image(&mut image).unwrap();
    ///     let symbols = image.symbols().unwrap();
    ///     symbols.first_symbol()
    /// };
    /// ```
    ///
    /// ```compile_fail
    /// use zbars::prelude::*;
    /// use std::borrow::Cow;
    ///
    /// let mut scanner = ImageScanner::builder().build().unwrap();
    ///
    /// let first = {
    ///     let mut image = ZBarImage::from_owned(1, 1, &Format::from_label(Cow::Borrowed("Y8")), vec![1]).unwrap();
    ///     let symbols = scanner.scan_image(&mut image).unwrap();
    ///     symbols.first_symbol()
    /// };
    /// ```
    pub fn first_symbol(&'a self) -> Option<Symbol<'a>> {
        Symbol::from_raw(unsafe { zbar_symbol_set_first_symbol(**self) })
    }
    pub fn iter(&'a self) -> SymbolIter<'a> { self.first_symbol().into() }
}
#[cfg(feature = "zbar_fork")]
impl<'a> SymbolSet<'a> {
    pub fn first_symbol_unfiltered(&self) -> Option<Symbol> {
        Symbol::from_raw(unsafe { zbar_symbol_set_first_unfiltered(**self) })
    }
}
impl<'a> Deref for SymbolSet<'a> {
    type Target = *const zbar_symbol_set_s;
    fn deref(&self) -> &Self::Target { &self.symbol_set }
}

pub struct SymbolIter<'a> {
    symbol: Option<Symbol<'a>>,
}
impl<'a> From<Option<Symbol<'a>>> for SymbolIter<'a> {
    fn from(symbol: Option<Symbol<'a>>) -> Self { Self { symbol } }
}
impl<'a> Iterator for SymbolIter<'a> {
    type Item = Symbol<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next = self.symbol.as_ref().and_then(Symbol::next);
        ::std::mem::swap(&mut self.symbol, &mut next);
        next
    }
}
