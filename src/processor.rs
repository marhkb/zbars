use image::ZbarImage;
use super::*;
use symbolset::SymbolSet;
use std::ffi::OsString;


pub struct Processor {
    processor: *mut zbar_processor_s,
}
impl Processor {
    pub fn new(threaded: bool) -> Self {
        let mut processor = Processor {
            processor: unsafe { zbar_processor_create(threaded as i32) }
        };
        processor.set_config(ZBarSymbolType::ZBAR_NONE, ZBarConfig::ZBAR_CFG_ENABLE, 0);
        processor
    }
    pub fn builder() -> ProcessorBuilder { ProcessorBuilder::new() }

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
    pub fn force_format(&mut self, input_format: Format, output_format: Format) -> i32 {
        unsafe  { zbar_processor_force_format(**self, input_format as u64, output_format as u64) }
    }
    pub fn set_userdata(&mut self, userdata: &[u8]) {
        //TODO
        unimplemented!("TBD")
    }
    pub fn userdata(&self) {
        //TODO
        unimplemented!("TBD")
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
    //TODO: special type as return
    pub fn process_image(&self, image: &ZbarImage) -> i32 {
        unsafe { zbar_process_image(**self, **image) }
    }
}
impl Deref for Processor {
    type Target = *mut zbar_processor_s;
    fn deref(&self) -> &<Self as Deref>::Target { &self.processor }
}
impl Drop for Processor {
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
    pub fn build(&self) -> Processor {
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
        if let Some(format) = self.format {
            processor.force_format(format.0, format.1);
        }
        self.config.iter().for_each(|values| {
            processor.set_config(values.0, values.1, values.2);
        });
        processor
    }
}



#[cfg(test)]
mod test {
    use super::*;

//    #[test]
    fn test() {
        let mut processor = Processor::builder()
            .threaded(true)
            .with_config(ZBarSymbolType::ZBAR_QRCODE, ZBarConfig::ZBAR_CFG_ENABLE, 1)
            .with_config(ZBarSymbolType::ZBAR_CODE128, ZBarConfig::ZBAR_CFG_ENABLE, 1)
            .build();

        processor.init("/dev/video0", true).unwrap();
        processor.set_active(true).unwrap();
        processor.set_visible(true).unwrap();

        processor.process_one(-1);
        let symbol = processor.get_results().unwrap().first_symbol().unwrap();
        println!("{}", symbol.data());
    }

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
            .build();
        assert!(processor.init("nothing", true).is_err())
    }

    #[test]
    #[cfg(feature = "from_image")]
    fn test_scan_image() {
        let image = ZbarImage::from_path("test/qrcode.png").unwrap();

        let processor = Processor::builder()
            .threaded(true)
            .with_config(ZBarSymbolType::ZBAR_QRCODE, ZBarConfig::ZBAR_CFG_ENABLE, 1)
            .with_config(ZBarSymbolType::ZBAR_CODE128, ZBarConfig::ZBAR_CFG_ENABLE, 1)
            .build();

        processor.process_image(&image);

        let symbol = image.first_symbol().unwrap();

        assert_eq!(symbol.symbol_type(), ZBarSymbolType::ZBAR_QRCODE);
        assert_eq!(symbol.data(), "https://www.ikimuni.de/");
        assert_eq!(symbol.next().is_none(), true);
    }
}
