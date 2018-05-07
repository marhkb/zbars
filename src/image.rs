use std::{
    path::Path,
    borrow::Cow,
    slice::from_raw_parts,
};
use super::*;
use format::*;
use symbol::*;
use symbolset::*;

pub struct ZBarImage<'a> {
    image: *mut zbar_image_s,
    buf: Option<&'a [u8]>,
}
impl<'a> ZBarImage<'a> {
    unsafe fn from_raw(image: *mut zbar_image_s, buf: Option<&'a [u8]>) -> Self { Self { image, buf } }
    pub fn from_buf(width: u32, height: u32, format: &Format, buf: Vec<u8>) -> Self {
        assert_eq!((width * height) as usize, buf.len());

        unsafe {
            let image = zbar_image_create();
            zbar_image_set_format(image, format.fourcc());
            zbar_image_set_size(image, width, height);
            zbar_image_set_data(
                image,
                buf.as_ptr() as *mut c_void,
                buf.len() as u64,
                Some(zbar_image_free_data)
            );

            let image = Self::from_raw(image, None);

            ::std::mem::forget(buf);

            image
        }
    }
    pub fn from_slice(width: u32, height: u32, format: &Format, slice: &'a [u8]) -> Self {
        unsafe {
            let image = zbar_image_create();
            zbar_image_set_format(image, format.fourcc());
            zbar_image_set_size(image, width, height);
            zbar_image_set_data(
                image,
                slice.as_ptr() as *mut c_void,
                slice.len() as u64,
                None
            );

            Self::from_raw(image, Some(slice))
        }
    }
    pub fn image_ref(&mut self) {
        //TODO: Needed?
        unimplemented!("TBD")
    }
    pub fn convert(&self, format: &Format) -> Self {
        unsafe { Self::from_raw(zbar_image_convert(**self, format.fourcc()), None) }
    }
    pub fn convert_resize(&self, _format: Format, _width: u32, _height: u32) -> Self {
        //TODO: exits with SIGSEGV
        unimplemented!("TBD: exits with SIGSEGV")
//        unsafe {
//            Self::from_raw(zbar_image_convert_resize(**self, format as u64, width, height), None)
//        }
    }
    pub fn format(&self) -> Format {
        unsafe { Format::from_fourcc(zbar_image_get_format(**self)) }
    }
    pub fn sequence(&self) -> u32 { unsafe { zbar_image_get_sequence(**self) } }
    pub fn width(&self) -> u32 { unsafe { zbar_image_get_width(**self) } }
    pub fn height(&self) -> u32 { unsafe { zbar_image_get_height(**self) } }
    pub fn data(&self) -> &[u8] {
        unsafe {
            from_raw_parts(
                zbar_image_get_data(**self) as *const u8,
                zbar_image_get_data_length(**self) as usize
            )
        }
    }
    pub fn symbols(&self) -> Option<SymbolSet> {
        SymbolSet::from_raw(unsafe { zbar_image_get_symbols(**self) })
    }
    pub fn set_symbols(&mut self, symbols: Option<&SymbolSet>) {
        unsafe { zbar_image_set_symbols(**self, symbols.map_or(::std::ptr::null(), |s| **s)) }
    }
    pub fn first_symbol(&self) -> Option<Symbol> {
        Symbol::from_raw(unsafe { zbar_image_first_symbol(self.image) })
    }
    //TODO: Not Needed
    //    pub fn set_format(&mut self) {
//        unimplemented!("TBD")
//    }
    pub fn set_sequence(&mut self, sequence_num: u32) {
        unsafe { zbar_image_set_sequence(**self, sequence_num) }
    }
    //TODO: Not needed
//    pub fn set_size(&mut self, width: u32, height: u32) {
//        unsafe { zbar_image_set_size(**self, width, height) }
//    }
    pub fn set_userdata(&mut self, userdata: Vec<u8>) {
        //TODO
        unimplemented!("TBD")
    }
    pub fn userdata(&self) -> &[u8] {
        //TODO
        unimplemented!("TBD")
    }
    pub fn write<P>(&self, path: P) -> ZBarResult<()> where P: AsRef<Path> {
        let result = unsafe {
            zbar_image_write(
                **self,
                path.as_ref().as_os_str().to_str().unwrap().as_bytes().as_ptr() as *mut i8,
            )
        };
        match result {
            0 => Ok(()),
            e => Err(e.into()),
        }
    }
    pub fn read<P>(_path: P) -> Option<Self> where P: AsRef<Path> {
        //TODO: zbar.h days: TBD
//        ZbarImage {
//            image: unsafe {
//                zbar_image_read(
//                    path.as_ref().as_os_str().to_str().unwrap().as_bytes().as_ptr() as *mut i8
//                )
//            }
//        }
        unimplemented!("zbar.h days: TBD")
    }
}

#[cfg(feature = "zbar_fork")]
impl<'a> ZBarImage<'a> {
    pub fn size(&self) -> (u32, u32) {
        unsafe {
            let mut dim = (0, 0);
            zbar_image_get_size(**self, &mut dim.0 as *mut u32, &mut dim.1 as *mut u32);
            dim
        }
    }
    pub fn crop(&self) -> (u32, u32, u32, u32) {
        unsafe {
            let mut crop = (0, 0, 0, 0);
            zbar_image_get_crop(
                **self,
                &mut crop.0 as *mut u32, &mut crop.1 as *mut u32,
                &mut crop.2 as *mut u32, &mut crop.3 as *mut u32
            );
            crop
        }
    }
    pub fn set_crop(&mut self, x: u32, y: u32, width: u32, height: u32) {
        unsafe { zbar_image_set_crop(**self, x, y, width, height) }
    }
}
impl<'a> Deref for ZBarImage<'a> {
    type Target = *mut zbar_image_s;
    fn deref(&self) -> &Self::Target { &self.image }
}
impl<'a> Drop for ZBarImage<'a> {
    fn drop(&mut self) { unsafe { zbar_image_destroy(**self) } }
}

#[cfg(feature = "from_image")]
pub mod from_image {
    extern crate image;

    use self::image::{
        GenericImage,
        ImageResult,
        Pixel,
    };
    use super::*;

    impl<'a> ZBarImage<'a> {
        pub fn from_path<P>(path: P) -> ImageResult<Self> where P: AsRef<Path> {
            image::open(&path).map(Self::from_image)
        }

        pub fn from_image<I>(image: I) -> Self
            where I: GenericImage + 'static,
                  u8: From<<<I as GenericImage>::Pixel as Pixel>::Subpixel>,
        {
            let image = image::imageops::grayscale(&image);

            ZBarImage::from_buf(
                image.dimensions().0,
                image.dimensions().1,
                &Format::from_label(Cow::Borrowed("Y800")),
                image.pixels().map(|p| u8::from(p.data[0])).collect::<Vec<u8>>()
            )
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test() {
            assert!(ZBarImage::from_path("test/code128.gif").is_ok());
        }
    }
}

#[cfg(test)]
#[cfg(feature = "zbar_fork")]
mod test_zbar_fork {
    use super::*;

    #[test]
    fn test_size() {
        assert_eq!(
            ZBarImage::from_buf(2, 3, &Format::from_label(Cow::Borrowed("Y800")),
                                [0; 2 * 3].to_vec()).size(), (2, 3)
        );
    }

    #[test]
    fn test_crop_get_and_set() {
        let mut image = ZBarImage::from_buf(20, 30, &Format::from_label(Cow::Borrowed("Y800")), [0; 20 * 30].to_vec());
        assert_eq!(image.crop(), (0, 0, 20, 30));
        image.set_crop(5, 5, 10, 10);
        assert_eq!(image.crop(), (5, 5, 10, 10));
    }
}

#[cfg(test)]
mod test {
    use super::*;

//    #[test]
    fn test_mem_from_buf() {
        for _ in 0..1000000 {
            let buf = [0; 500 * 500];
            ZBarImage::from_buf(500, 500, &Format::from_label(Cow::Borrowed("Y800")), buf.to_vec());
        }
    }

//    #[test]
    fn test_mem_from_slice() {
        for _ in 0..1000000 {
            let buf = [0; 500 * 500];
            ZBarImage::from_slice(500, 500, &Format::from_label(Cow::Borrowed("Y800")), buf.as_ref());
        }
    }

    // If this does not compile everything is fine
//    fn from_slice() -> ZBarImage<'static> {
//        let buf = [0; 500 * 500];
//        ZBarImage::from_slice(500, 500, buf.as_ref())
//    }

    #[test]
    fn test_convert() {
        let image = ZBarImage::from_buf(2, 3, &Format::from_label(Cow::Borrowed("Y800")), [0; 2 * 3].to_vec());
        let format = Format::from_label(Cow::Borrowed("GREY"));
        assert_eq!(image.convert(&format).format(), format)
    }

    //TODO
//    #[test]
//    fn test_convert_resize() {
//        let image = ZbarImage::new(10, 20, [0; 10 * 20].to_vec());
//        let (format, width, height) = (Format::Y800, 20, 40);
//        let converted = image.convert_resize(format, width, height);
//        assert_eq!(converted.format(), format);
//        assert_eq!(converted.width(), width);
//        assert_eq!(converted.height(), height);
//    }
    #[test]
    fn format() {
        let format = Format::from_label(Cow::Borrowed("Y800"));
        assert_eq!(ZBarImage::from_buf(2, 3, &format, [0; 2 * 3].to_vec()).format(), format);
    }

    #[test]
    fn test_sequence_set_and_get() {
        let mut image = ZBarImage::from_buf(2, 3, &Format::from_label(Cow::Borrowed("Y800")), [0; 2 * 3].to_vec());
        assert_eq!(image.sequence(), 0);
        image.set_sequence(1);
        assert_eq!(image.sequence(), 1);
        image.set_sequence(999);
        assert_eq!(image.sequence(), 999);
    }

    #[test]
    fn test_width() {
        assert_eq!(
            ZBarImage::from_buf(2, 3, &Format::from_label(Cow::Borrowed("Y800")), [0; 2 * 3].to_vec()).width(),
            2
        );
    }

    #[test]
    fn test_height() {
        assert_eq!(
            ZBarImage::from_buf(2, 3, &Format::from_label(Cow::Borrowed("Y800")), [0; 2 * 3].to_vec()).height(),
            3
        );
    }

    #[test]
    fn test_data() {
        let buf = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11].to_vec();
        let image = ZBarImage::from_buf(3, 4, &Format::from_label(Cow::Borrowed("Y800")), buf.clone());
        assert_eq!(buf.as_slice(), image.data());
    }

    #[test]
    fn test_symbols_get_and_set() {
        let mut image = ZBarImage::from_buf(20, 30, &Format::from_label(Cow::Borrowed("Y800")), [0; 20 * 30].to_vec());
        assert!(image.symbols().is_none());
        image.set_symbols(None);
        assert!(image.symbols().is_none());
    }

    #[test]
    fn test_first_symbol() {
        assert!(
            ZBarImage::from_buf(20, 30, &Format::from_label(Cow::Borrowed("Y800")), [0; 20 * 30].to_vec()).first_symbol().is_none()
        );
    }

    #[test]
    fn test_write() {
        let path = std::env::temp_dir().join("zbar_image");
        let image = ZBarImage::from_buf(2, 3, &Format::from_label(Cow::Borrowed("Y800")), [0; 2 * 3].to_vec());
        assert!(image.write(&path).is_ok());
    }

    #[test]
    fn test_write_fail() {
        let path = Path::new("/nowhere/nothing");
        let image = ZBarImage::from_buf(2, 3, &Format::from_label(Cow::Borrowed("Y800")), [0; 2 * 3].to_vec());
        assert!(image.write(&path).is_err());
    }
}
