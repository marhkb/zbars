use {
    format::Format,
    image::ZBarImage,
    symbolset::SymbolSet,
};
use super::*;

pub struct Processor<'a> {
    processor: *mut zbar_processor_s,
    userdata: Option<Cow<'a, [u8]>>,
}
impl<'a> Processor<'a> {
    pub fn new(threaded: bool) -> Self {
        let mut processor = Processor {
            processor: unsafe { zbar_processor_create(threaded as i32) },
            userdata: None,
        };
        processor.set_config(ZBarSymbolType::ZBAR_NONE, ZBarConfig::ZBAR_CFG_ENABLE, 0)
            // save to unwrap here
            .unwrap();
        processor
    }
    pub fn builder() -> ProcessorBuilder { ProcessorBuilder::new() }

    pub fn init(&mut self, video_device: impl AsRef<str>, enable_display: bool) -> ZBarSimpleResult<()> {
        let result = unsafe {
            zbar_processor_init(
                **self,
                OsString::from(video_device.as_ref()).to_str().unwrap().as_ptr() as *const i8,
                enable_display as i32
            )
        };
        match result == 0 {
            true  => Ok(()),
            false => Err(result),
        }
    }
    //TODO: bool or Result?
    pub fn request_size(&mut self, width: u32, height: u32) -> i32 {
        unsafe { zbar_processor_request_size(**self, width, height) }
    }
    //TODO: bool or Result?
    pub fn request_interface(&mut self, version: i32) -> i32 {
        unsafe { zbar_processor_request_interface(**self, version) }
    }
    //TODO: bool or Result?
    pub fn request_iomode(&mut self, iomode: i32) -> i32 {
        unsafe { zbar_processor_request_iomode(**self, iomode) }
    }
    pub fn force_format(&mut self, input_format: Format, output_format: Format) -> i32 {
        unsafe  { zbar_processor_force_format(**self, input_format.value().into(), output_format.value().into()) }
    }

    /// Sets borrowed user data for `Processor`.
    ///
    /// User data can be shared across different `Processors`.
    ///
    /// # Examples
    ///
    /// ```
    /// use zbars::prelude::*;
    ///
    /// let userdata = "Hello World".as_bytes();
    /// let mut processor1 = Processor::builder().build().unwrap();
    /// let mut processor2 = Processor::builder().build().unwrap();
    /// processor1.set_userdata_borrowed(Some(&userdata));
    /// processor2.set_userdata_borrowed(Some(&userdata));
    /// assert_eq!(processor1.userdata().unwrap(), processor1.userdata().unwrap());
    /// ```
    pub fn set_userdata(&mut self, userdata: Option<Cow<'a, [u8]>>) {
        unsafe {
            zbar_processor_set_userdata(
                **self,
                userdata.as_ref().map_or(ptr::null(), |s| s.as_ptr()) as *mut u8 as *mut c_void)
        }
        self.userdata = userdata;
    }
    pub fn set_userdata_owned(&mut self, userdata: Option<Vec<u8>>) {
        self.set_userdata(userdata.map(Cow::Owned))
    }
    pub fn set_userdata_borrowed(&mut self, userdata: Option<&'a (impl AsRef<[u8]> + Clone)>) {
        self.set_userdata(userdata.map(AsRef::as_ref).map(Cow::Borrowed))
    }

    /// Returns assigned user data of `Processor`.
    ///
    /// # Examples
    ///
    /// ```
    /// use zbars::prelude::*;
    ///
    /// let userdata = "Hello World".as_bytes();
    /// let mut processor1 = Processor::builder().build().unwrap();
    /// let mut processor2 = Processor::builder().build().unwrap();
    /// processor1.set_userdata_borrowed(Some(&userdata));
    /// processor2.set_userdata_owned(Some("Hello World".as_bytes().to_owned()));
    /// assert_eq!(processor1.userdata().unwrap(), processor1.userdata().unwrap());
    /// ```
    pub fn userdata(&self) -> Option<&Cow<'a, [u8]>> { self.userdata.as_ref() }
    pub fn set_config(&mut self, symbol_type: ZBarSymbolType, config: ZBarConfig, value: i32) -> ZBarResult<()> {
        let result = unsafe {
            zbar_processor_set_config(**self, symbol_type, config, value)
        };
        match result == 0 {
            true  => Ok(()),
            false => Err(result.into())
        }
    }
    pub fn set_control(&mut self, control_name: impl AsRef<str>, value: i32) -> ZBarSimpleResult<()> {
        //TODO
        unimplemented!("TBD")
//        let result = unsafe {
//            zbar_processor_set_control(
//                **self,
//                OsString::from(control_name.as_ref()).to_str().unwrap().as_ptr() as *const i8,
//                value
//            )
//        };
//        println!("{}", result);
//        match result == 0 {
//            true  => Ok(()),
//            false => Err(result),
//        }
    }
    pub fn get_control(&self, control_name: impl AsRef<str>) -> ZBarResult<i32> {
        //TODO
        unimplemented!("TBD")
//        let mut value = 0;
//        let result = unsafe {
//            zbar_processor_get_control(
//                **self,
//                control_name.as_ref().as_ptr() as *const i8,
//                &mut value as *mut i32
//            )
//        };
//        match result == 0 {
//            true  => Ok(value),
//            false => Err(result.into()),
//        }
    }
    pub fn is_visible(&self) -> ZBarSimpleResult<bool> {
        let result = unsafe { zbar_processor_is_visible(**self) };
        match result > 0 {
            true  => Ok(true),
            false => match result == 0 {
                true  => Ok(false),
                false => Err(result),
            }
        }
    }
    pub fn set_visible(&mut self, visible: bool) -> ZBarSimpleResult<bool> {
        let result = unsafe { zbar_processor_set_visible(**self, visible as i32) };
        match result > 0 {
            true  => Ok(true),
            false => match result == 0 {
                true  => Ok(false),
                false => Err(result),
            }
        }
    }
    pub fn set_active(&mut self, active: bool) -> ZBarSimpleResult<bool> {
        let result = unsafe { zbar_processor_set_active(**self, active as i32) };
        match result > 0 {
            true  => Ok(true),
            false => match result == 0 {
                true  => Ok(false),
                false => Err(result),
            }
        }
    }
    pub fn get_results(&self) -> Option<SymbolSet> {
        SymbolSet::from_raw(unsafe { zbar_processor_get_results(**self) })
    }

    //TODO: special type as return
    pub fn user_wait(&self, timeout: i32) -> i32 {
        unsafe { zbar_processor_user_wait(**self, timeout) }
    }
    //TODO: special type as return
    pub fn process_one(&self, timeout: i32) -> i32 {
        unsafe { zbar_process_one(**self, timeout) }
    }
    pub fn process_image(&mut self, image: &mut ZBarImage) -> ZBarSimpleResult<SymbolSet> {
        let result = unsafe { zbar_process_image(**self, **image) };
        match result >= 0 {
            true  => Ok(image.symbols().unwrap()), // symbols can be unwrapped because image is surely scanned
            false => Err(result),
        }
    }
}

unsafe impl<'a> Send for Processor<'a> {}

unsafe impl<'a> Sync for Processor<'a> {}

impl<'a> Deref for Processor<'a> {
    type Target = *mut zbar_processor_s;
    fn deref(&self) -> &Self::Target { &self.processor }
}
impl<'a> Drop for Processor<'a> {
    fn drop(&mut self) { unsafe { zbar_processor_destroy(**self) } }
}

pub struct ProcessorBuilder {
    threaded: bool,
    size: Option<(u32, u32)>,
    interface_version: Option<i32>,
    iomode: Option<i32>,
    format: Option<(Format, Format)>,
    config: Vec<(ZBarSymbolType, ZBarConfig, i32)>,
}
impl ProcessorBuilder {
    pub fn new() -> Self {
        Self {
            threaded: false,
            size: None,
            interface_version: None,
            iomode: None,
            format: None,
            config: vec![],
        }
    }
    pub fn threaded(&mut self, threaded: bool) -> &mut Self { self.threaded = threaded; self }
    pub fn with_size(&mut self, size: Option<(u32, u32)>) -> &mut Self { self.size = size; self }
    pub fn with_interface_version(&mut self, interface_version: Option<i32>) -> &mut Self {
        self.interface_version = interface_version; self
    }
    pub fn with_iomode(&mut self, iomode: Option<i32>) -> &mut Self { self.iomode = iomode; self }
    pub fn with_format(&mut self, format: Option<(Format, Format)>) -> &mut Self {
        self.format = format; self
    }
    pub fn with_config(&mut self, symbol_type: ZBarSymbolType, config: ZBarConfig, value: i32) -> &mut Self {
        self.config.push((symbol_type, config, value)); self
    }
    pub fn build<'b>(&self) -> ZBarResult<Processor<'b>> {
        let mut processor = Processor::new(self.threaded);
        if let Some(size) = self.size {
            processor.request_size(size.0, size.1);
        }
        if let Some(interface_version) = self.interface_version {
            processor.request_interface(interface_version);
        }
        if let Some(iomode) = self.iomode {
            processor.request_iomode(iomode);
        }
        if let Some(ref format) = self.format {
            processor.force_format(format.0, format.1);
        }
        for values in &self.config {
            processor.set_config(values.0, values.1, values.2)?;
        }
        Ok(processor)
    }
}



#[cfg(test)]
mod test {
    use super::*;


//    #[test]
//    fn test_set_control() {
////        let mut processor = Processor::builder()
////            .threaded(true)
////            .build();
////        processor.init("/dev/video0", true).is_err();
////        processor.set_control("/dev/video0", 0).unwrap();
//    }

    #[test]
    fn test_wrong_video_device() {
        let mut processor = Processor::builder()
            .threaded(true)
            .build()
            .unwrap();
        assert!(processor.init("nothing", true).is_err())
    }

    #[test]
    fn test_userdata_set_and_get() {
        let userdata = "Hello World".as_bytes().to_owned();

        let mut processor1 = Processor::builder().build().unwrap();
        let mut processor2 = Processor::builder().build().unwrap();
        let mut processor3 = Processor::builder().build().unwrap();

        assert!(processor1.userdata().is_none());

        processor1.set_userdata_borrowed(Some(&userdata));
        processor2.set_userdata_owned(Some(userdata.clone()));
        processor3.set_userdata_borrowed(Some(&userdata));

        assert_eq!(processor1.userdata().unwrap(), processor2.userdata().unwrap());
        assert_eq!(processor1.userdata().unwrap(), processor3.userdata().unwrap());
    }

    #[test]
    #[cfg(feature = "from_image")]
    fn test_scan_image() {
        let mut image = ZBarImage::from_path("test/qr_hello-world.png").unwrap();

        let mut processor = Processor::builder()
            .threaded(true)
            .with_config(ZBarSymbolType::ZBAR_QRCODE, ZBarConfig::ZBAR_CFG_ENABLE, 1)
            .with_config(ZBarSymbolType::ZBAR_CODE128, ZBarConfig::ZBAR_CFG_ENABLE, 1)
            .build()
            .unwrap();

        processor.process_image(&mut image).unwrap();

        let symbol = image.first_symbol().unwrap();

        assert_eq!(symbol.symbol_type(), ZBarSymbolType::ZBAR_QRCODE);
        assert_eq!(symbol.data(), "Hello World");
        assert_eq!(symbol.next().is_none(), true);
    }
}
