use {
    ffi,
    ZBarConfig,
    ZBarResult,
    ZBarSymbolType
};

pub struct Decoder {
    pub(crate) decoder: *mut ffi::zbar_decoder_s,
}

impl Decoder {
    pub fn new() -> Self { Self::default() }
    pub fn set_config(&self, symbol_type: ZBarSymbolType, config: ZBarConfig, value: i32) -> ZBarResult<()> {
        match unsafe { ffi::zbar_decoder_set_config(self.decoder, symbol_type, config, value) } {
            0 => Ok(()),
            e => Err(e.into())
        }
    }
}

impl Default for Decoder {
    fn default() -> Self { Decoder { decoder: unsafe {ffi::zbar_decoder_create() } } }
}

impl Drop for Decoder {
    fn drop(&mut self) { unsafe { ffi::zbar_decoder_destroy(self.decoder) } }
}
