use super::*;
use symbol::*;

pub struct SymbolSet {
    symbol_set: *const zbar_symbol_set_s,
}
impl SymbolSet {
    pub fn from_raw(symbol_set: *const zbar_symbol_set_s) -> Option<Self> {
        match symbol_set.is_null() {
            true  => None,
            false => Some(Self { symbol_set })
        }
    }
    pub fn set_ref(&mut self, _refs: i32) {
        //TODO
        unimplemented!("TBD")
//        unsafe { zbar_symbol_set_ref(**self, refs) }
    }
    pub fn size(&self) -> i32 { unsafe { zbar_symbol_set_get_size(**self) } }
    pub fn first_symbol(&self) -> Option<Symbol> {
        Symbol::from_raw(unsafe { zbar_symbol_set_first_symbol(**self) })
    }
    pub fn iter(&self) -> SymbolIter { self.first_symbol().into() }
}
#[cfg(feature = "zbar_fork")]
impl SymbolSet {
    pub fn first_symbol_unfiltered(&self) -> Option<Symbol> {
        Symbol::from_raw(unsafe { zbar_symbol_set_first_unfiltered(**self) })
    }
}
impl Deref for SymbolSet {
    type Target = *const zbar_symbol_set_s;
    fn deref(&self) -> &Self::Target { &self.symbol_set }
}

pub struct SymbolIter {
    symbol: Option<Symbol>,
}
impl From<Option<Symbol>> for SymbolIter {
    fn from(symbol: Option<Symbol>) -> Self { Self { symbol } }
}
impl Iterator for SymbolIter {
    type Item = Symbol;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next = self.symbol.as_ref().and_then(Symbol::next);
        ::std::mem::swap(&mut self.symbol, &mut next);
        next
    }
}
