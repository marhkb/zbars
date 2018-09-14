use {
    format::Format,
    image::Image,
    symbol_set::SymbolSet,
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

    //Tested
    pub fn init(&self, video_device: impl AsRef<str>, enable_display: bool) -> ZBarResult<()> {
        match unsafe { zbar_processor_init(**self, as_char_ptr(video_device), enable_display as i32) } {
            0 => Ok(()),
            e => Err(ZBarErrorType::Simple(e)),
        }
    }
    //Tested
    pub fn request_size(&self, width: u32, height: u32) -> ZBarResult<()> {
        match unsafe { zbar_processor_request_size(**self, width, height) } {
            0 => Ok(()),
            e => Err(ZBarErrorType::Simple(e)),
        }
    }
    //Tested
    pub fn request_interface(&self, version: i32) -> ZBarResult<()> {
        match unsafe { zbar_processor_request_interface(**self, version) } {
            0 => Ok(()),
            e => Err(ZBarErrorType::Simple(e)),
        }
    }
    //Tested
    pub fn request_iomode(&self, iomode: i32) -> ZBarResult<()> {
        match unsafe { zbar_processor_request_iomode(**self, iomode) } {
            0 => Ok(()),
            e => Err(ZBarErrorType::Simple(e)),
        }
    }
    pub fn force_format(&self, input_format: Format, output_format: Format) -> ZBarResult<()> {
        match unsafe {
            zbar_processor_force_format(
                **self,
                input_format.value().into(),
                output_format.value().into()
            )
        } {
            0 => Ok(()),
            e => Err(ZBarErrorType::Simple(e)),
        }
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
        match unsafe { zbar_processor_set_config(**self, symbol_type, config, value) }  {
            0 => Ok(()),
            e => Err(e.into())
        }
    }

    pub fn is_visible(&self) -> ZBarResult<bool> {
        match unsafe { zbar_processor_is_visible(**self) } {
            0 => Ok(false),
            1 => Ok(true),
            e => Err(ZBarErrorType::Simple(e)),
        }
    }
    pub fn set_visible(&self, visible: bool) -> ZBarResult<bool> {
        match unsafe { zbar_processor_set_visible(**self, visible as i32) } {
            0 => Ok(false),
            1 => Ok(true),
            e => Err(ZBarErrorType::Simple(e)),
        }
    }
    pub fn set_active(&self, active: bool) -> ZBarResult<bool> {
        match unsafe { zbar_processor_set_active(**self, active as i32) } {
            0 => Ok(false),
            1 => Ok(true),
            e => Err(ZBarErrorType::Simple(e)),
        }
    }
    pub fn get_results(&self) -> Option<SymbolSet> {
        SymbolSet::from_raw(unsafe { zbar_processor_get_results(**self) })
    }

    // Tested
    pub fn user_wait(&self, timeout: i32) -> ZBarResult<i32> {
        match unsafe { zbar_processor_user_wait(**self, timeout) } {
            -1 => Err(ZBarErrorType::Simple(-1)),
            o  => Ok(o),
        }
    }

    // Tested
    pub fn process_one(&self, timeout: i32) -> ZBarResult<Option<SymbolSet>> {
        match unsafe { zbar_process_one(**self, timeout) } {
            -1 => Err(ZBarErrorType::Simple(-1)),
            0  => Ok(None),
            _  => Ok(self.get_results())
        }
    }

    // Tested
    pub fn process_image<T>(&self, image: &Image<T>) -> ZBarResult<SymbolSet> where T: AsRef<[u8]> + Clone {
        match unsafe { zbar_process_image(**self, **image) } {
            -1 => Err(ZBarErrorType::Simple(-1)),
            _  => Ok(image.symbols().unwrap()), // symbols can be unwrapped because image is surely scanned
        }
    }

    /// Set V4L2 Controls.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use zbars::prelude::*;
    ///
    /// let processor = Processor::builder().build().unwrap();
    /// processor.init("/dev/video0", false).unwrap();
    /// processor.set_control("brightness", 75).unwrap();
    /// processor.set_control("contrast", 50).unwrap();
    /// ```
    #[cfg(feature = "zbar_fork")]
    pub fn set_control(&self, control_name: impl AsRef<str>, value: i32) -> ZBarResult<()> {
        match unsafe { zbar_processor_set_control(**self, as_char_ptr(control_name), value) } {
            0 => Ok(()),
            e => Err(ZBarErrorType::Simple(e))
        }
    }

    /// Get V4L2 Controls.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use zbars::prelude::*;
    ///
    /// let processor = Processor::builder().build().unwrap();
    /// processor.init("/dev/video0", false).unwrap();
    /// println!("brightness: {}", processor.control("brightness").unwrap());
    /// println!("contrast: {}", processor.control("contrast").unwrap());
    /// ```
    #[cfg(feature = "zbar_fork")]
    pub fn control(&self, control_name: impl AsRef<str>) -> ZBarResult<i32> {
        let mut value = 0;
        match unsafe {
            zbar_processor_get_control(**self, as_char_ptr(control_name), &mut value as *mut i32)
        } {
            0 => Ok(value),
            e => Err(ZBarErrorType::Simple(e))
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
    pub fn build<'a>(&self) -> ZBarResult<Processor<'a>> {
        let mut processor = Processor::new(self.threaded);
        if let Some(size) = self.size {
            processor.request_size(size.0, size.1)?;
        }
        if let Some(interface_version) = self.interface_version {
            processor.request_interface(interface_version)?;
        }
        if let Some(iomode) = self.iomode {
            processor.request_iomode(iomode)?;
        }
        if let Some(ref format) = self.format {
            processor.force_format(format.0, format.1)?;
        }
        self.config
            .iter()
            .try_for_each(|v| processor.set_config(v.0, v.1, v.2))
            .map(|_| processor)
    }
}

#[cfg(test)]
mod test {
    use std::sync::{
        Arc,
        Mutex,
    };
    use super::*;

    lazy_static! { static ref VIDEO_LOCK: Arc<Mutex<()>> = { Arc::new(Mutex::new(())) }; }

    #[test]
    fn test_wrong_video_device() {
        let processor = Processor::builder()
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
    fn test_process_image() {
        let image = Image::from_path("test/qr_hello-world.png").unwrap();

        let processor = Processor::builder()
            .threaded(true)
            .with_config(ZBarSymbolType::ZBAR_QRCODE, ZBarConfig::ZBAR_CFG_ENABLE, 1)
            .with_config(ZBarSymbolType::ZBAR_CODE128, ZBarConfig::ZBAR_CFG_ENABLE, 1)
            .build()
            .unwrap();

        processor.process_image(&image).unwrap();

        let symbol = image.first_symbol().unwrap();

        assert_eq!(symbol.symbol_type(), ZBarSymbolType::ZBAR_QRCODE);
        assert_eq!(symbol.data(), "Hello World");
        assert_eq!(symbol.next().is_none(), true);
    }

    #[test]
    #[cfg(feature = "zbar_fork")]
    fn test_set_control() {
        let processor = Processor::builder()
            .threaded(true)
            .with_config(ZBarSymbolType::ZBAR_QRCODE, ZBarConfig::ZBAR_CFG_ENABLE, 1)
            .with_config(ZBarSymbolType::ZBAR_CODE128, ZBarConfig::ZBAR_CFG_ENABLE, 1)
            .build()
            .unwrap();

        let _lock = VIDEO_LOCK.lock().unwrap();

        processor.init("/dev/video0", false).unwrap();
        assert!(processor.set_control("brightness", 100).is_ok());
        assert!(processor.set_control("contrast", 100).is_ok());
    }

    #[test]
    #[cfg(feature = "zbar_fork")]
    fn test_control() {
        let processor = Processor::builder()
            .threaded(true)
            .with_config(ZBarSymbolType::ZBAR_QRCODE, ZBarConfig::ZBAR_CFG_ENABLE, 1)
            .with_config(ZBarSymbolType::ZBAR_CODE128, ZBarConfig::ZBAR_CFG_ENABLE, 1)
            .build()
            .unwrap();

        let _lock = VIDEO_LOCK.lock().unwrap();

        processor.init("/dev/video0", false).unwrap();
        assert!(processor.control("brightness").is_ok());
        assert!(processor.control("contrast").is_ok());
    }
}
