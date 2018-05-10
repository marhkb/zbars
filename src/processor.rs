use format::*;
use image::ZBarImage;
use super::*;
use symbolset::SymbolSet;
use std::{
    marker::PhantomData,
    ffi::OsString
};


pub struct Processor<'a> {
    processor: *mut zbar_processor_s,
    userdata_len: Option<usize>,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Processor<'a> {
    pub fn new(threaded: bool) -> Self {
        let mut processor = Processor {
            processor: unsafe { zbar_processor_create(threaded as i32) },
            userdata_len: None,
            phantom: PhantomData,
        };
        processor.set_config(ZBarSymbolType::ZBAR_NONE, ZBarConfig::ZBAR_CFG_ENABLE, 0)
            // save to unwrap here
            .unwrap();
        processor
    }
    pub fn builder<'b>() -> ProcessorBuilder<'b> { ProcessorBuilder::new() }

    pub fn init<T>(&mut self, video_device: T, enable_display: bool) -> ZBarSimpleResult<()> where T: AsRef<str> {
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
    pub fn force_format(&mut self, input_format: &Format, output_format: &Format) -> i32 {
        unsafe  { zbar_processor_force_format(**self, input_format.fourcc().into(), output_format.fourcc().into()) }
    }

    /// Sets userdata for `Processor`.
    ///
    /// # Code that should not compile
    ///
    /// ```compile_fail
    /// use zbars::prelude::*;
    /// use std::borrow::Cow;
    ///
    /// let mut processor = Processor::builder().build().unwrap();
    /// {
    ///     processor.set_userdata(&vec![1]);
    /// }
    /// ```
    pub fn set_userdata<T>(&mut self, userdata: &'a T) where T: AsRef<[u8]> {
        let userdata = userdata.as_ref();
        self.userdata_len = Some(userdata.len());
        unsafe { zbar_processor_set_userdata(**self, userdata.as_ref().as_ptr() as *mut u8 as *mut c_void) }
    }
    pub fn userdata(&self) -> Option<&'a [u8]>{
        self.userdata_len
            .map(|len| unsafe {
                ::std::slice::from_raw_parts(zbar_processor_get_userdata(**self) as *mut u8, len)
            })
    }
    pub fn set_config(&mut self, symbol_type: ZBarSymbolType, config: ZBarConfig, value: i32) -> ZBarResult<()> {
        let result = unsafe {
            zbar_processor_set_config(**self, symbol_type, config, value)
        };
        match result == 0 {
            true  => Ok(()),
            false => Err(result.into())
        }
    }
    pub fn set_control<T>(&mut self, control_name: T, value: i32) -> ZBarSimpleResult<()> where T: AsRef<str> {
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
    pub fn get_control<T>(&self, control_name: T) -> ZBarResult<i32> where T: AsRef<str> {
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
    pub fn process_image(&self, image: &mut ZBarImage) -> ZBarSimpleResult<SymbolSet> {
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

pub struct ProcessorBuilder<'a> {
    threaded: bool,
    size: Option<(u32, u32)>,
    interface_version: Option<i32>,
    iomode: Option<i32>,
    format: Option<(Format<'a>, Format<'a>)>,
    config: Vec<(ZBarSymbolType, ZBarConfig, i32)>,
}
impl<'a> ProcessorBuilder<'a> {
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
    pub fn with_format(&mut self, format: Option<(Format<'a>, Format<'a>)>) -> &mut Self {
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
            processor.force_format(&format.0, &format.1);
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
        let userdata = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

        let mut processor1 = Processor::builder().build().unwrap();
        let mut processor2 = Processor::builder().build().unwrap();
        let mut processor3 = Processor::builder().build().unwrap();

        assert!(processor1.userdata().is_none());

        processor1.set_userdata(&userdata);
        processor2.set_userdata(&userdata);
        processor3.set_userdata(&userdata);

        assert!(processor1.userdata().is_some());
        assert_eq!(processor1.userdata(), processor2.userdata());
        assert_eq!(processor1.userdata(), processor3.userdata());
    }

    #[test]
    #[cfg(feature = "from_image")]
    fn test_scan_image() {
        let mut image = ZBarImage::from_path("test/qrcode.png").unwrap();

        let processor = Processor::builder()
            .threaded(true)
            .with_config(ZBarSymbolType::ZBAR_QRCODE, ZBarConfig::ZBAR_CFG_ENABLE, 1)
            .with_config(ZBarSymbolType::ZBAR_CODE128, ZBarConfig::ZBAR_CFG_ENABLE, 1)
            .build()
            .unwrap();

        processor.process_image(&mut image).unwrap();

        let symbol = image.first_symbol().unwrap();

        assert_eq!(symbol.symbol_type(), ZBarSymbolType::ZBAR_QRCODE);
        assert_eq!(symbol.data(), "https://www.ikimuni.de/");
        assert_eq!(symbol.next().is_none(), true);
    }
}
