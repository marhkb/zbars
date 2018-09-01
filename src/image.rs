use {
    format::*,
    symbol::*,
    symbolset::*,
};
use std::{
    fmt,
    path::Path,
};
use super::*;

pub type ZBarImageResult<'a> = Result<ZBarImage<'a>, ZBarImageError>;

#[derive(Debug)]
pub enum ZBarImageError {
    Len(u32, u32, usize),
}
impl Error for ZBarImageError {
    fn description(&self) -> &str { "image error" }
}
impl fmt::Display for ZBarImageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::ZBarImageError::*;

        match *self {
            Len(w, h, l) => write!(
                f,
                "Width and height don't match actual data length\
                 => width: {}; height: {}; actual data length: {}",
                w, h, l
            )
        }
    }
}


pub struct ZBarImage<'a> {
    image: *mut zbar_image_s,
    data: Cow<'a, [u8]>,
    userdata: Option<Cow<'a, [u8]>>,
}
impl<'a> ZBarImage<'a> {
    pub fn new(width: u32, height: u32, format: Format, data: Cow<'a, [u8]>) -> ZBarImageResult<'a>
    {
        match width as usize * height as usize == data.len() {
            true => unsafe {
                let image = zbar_image_create();
                zbar_image_set_format(image, (*format).into());
                zbar_image_set_size(image, width, height);
                zbar_image_set_data(
                    image,
                    data.as_ptr() as *mut c_void,
                    (data.len() as u32).into(),
                    None
                );
                let mut image = Self { image, data, userdata: None };
                image.set_ref(1);
                Ok(image)
            }
            false => Err(ZBarImageError::Len(width, height, data.len()))
        }
    }

    /// Creates a `ZBarImage` from owned data.
    ///
    /// # Examples
    ///
    /// ```
    /// use zbars::prelude::*;
    /// use std::borrow::Cow;
    ///
    /// // only data of length 1 for demonstration
    /// ZBarImage::from_owned(1, 1, Format::from_label("Y8"), vec![1]).unwrap();
    /// ```
    pub fn from_owned(width: u32, height: u32, format: Format, data: Vec<u8>) -> ZBarImageResult<'a>
    {
        Self::new(width, height, format, Cow::Owned(data))
    }

    /// Creates a `ZBarImage` from borrowed data.
    ///
    /// # Examples
    ///
    /// ```
    /// use zbars::prelude::*;
    /// use std::borrow::Cow;
    ///
    /// let data = vec![1];
    /// let image = ZBarImage::from_borrowed(1, 1, Format::from_label("Y8"), &data).unwrap();
    /// ```
    pub fn from_borrowed(
        width: u32, height: u32, format: Format, data: &'a (impl ?Sized + AsRef<[u8]>)
    ) -> ZBarImageResult<'a>
    {
        Self::new(width, height, format, Cow::Borrowed(data.as_ref()))
    }

    fn set_ref(&mut self, refs: i32) {
        unsafe { zbar_image_ref(**self, refs) }
    }

    /// Returns the `Format` of the pixels.
    pub fn format(&self) -> Format {
        unsafe { Format::from_value(zbar_image_get_format(**self) as u32) }
    }
    pub fn sequence(&self) -> u32 { unsafe { zbar_image_get_sequence(**self) } }
    /// Returns the width of the image in pixels
    pub fn width(&self) -> u32 { unsafe { zbar_image_get_width(**self) } }
    /// Returns the height of the image in pixels
    pub fn height(&self) -> u32 { unsafe { zbar_image_get_height(**self) } }

    /// Retrieves the image buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// use zbars::prelude::*;
    ///
    /// let image = ZBarImage::from_owned(1, 1, Format::from_label("Y8"), vec![1]).unwrap();
    /// println!("{:?}", image.data());
    /// ```
    pub fn data(&self) -> &Cow<'a, [u8]> { &self.data }
    /// Returns an `Option` containing the `SymbolSet` or `None` if the image hasn't been scanned.
    ///
    /// # Examples
    ///
    /// ```
    /// use zbars::prelude::*;
    ///
    /// let mut image = ZBarImage::from_owned(1, 1, Format::from_label("Y8"), vec![1]).unwrap();
    /// let mut scanner = ImageScanner::builder().build().unwrap();
    /// match scanner.scan_image(&mut image) {
    ///     Ok(_) => match image.symbols() {
    ///         Some(symbols) => match symbols.first_symbol() {
    ///             Some(symbol) => println!("{}", symbol.data()),
    ///             None         => println!("no symbols in scanned image"),
    ///         }
    ///         None          => unreachable!("Not possible because image has surely been scanned"),
    ///     }
    ///     Err(e)           => println!("error scanning image {}", e),
    /// };
    /// ```
    pub fn symbols(&self) -> Option<SymbolSet> {
        SymbolSet::from_raw(unsafe { zbar_image_get_symbols(**self) })
    }
    pub fn set_symbols(&mut self, symbols: Option<&SymbolSet>) {
        unsafe { zbar_image_set_symbols(**self, symbols.map_or(ptr::null(), |s| **s)) }
    }
    pub fn first_symbol(&self) -> Option<Symbol> {
        Symbol::from_raw(unsafe { zbar_image_first_symbol(self.image) })
    }
    pub fn set_sequence(&mut self, sequence_num: u32) {
        unsafe { zbar_image_set_sequence(**self, sequence_num) }
    }

    /// Just a crop with origin
    pub fn set_size(&mut self, width: u32, height: u32) {
        unsafe { zbar_image_set_size(**self, width, height) }
    }

    /// Sets owned user data for `ZBarImage`.
    ///
    /// User data cannot be shared across different `ZbarImages`.
    ///
    /// # Examples
    ///
    /// ```
    /// use zbars::prelude::*;
    ///
    /// let userdata = "Hello World".as_bytes().to_owned();
    /// let mut image =
    ///      ZBarImage::from_owned(1, 1, Format::from_label("Y800"), vec![0]).unwrap();
    /// image.set_userdata_owned(Some(userdata));
    /// assert_eq!(image.userdata().unwrap().as_ref(), "Hello World".as_bytes());
    /// ```
    pub fn set_userdata(&mut self, userdata: Option<Cow<'a, [u8]>>) {
        unsafe {
            zbar_image_set_userdata(
                **self,
                userdata.as_ref().map_or(std::ptr::null(), |s| s.as_ptr()) as *mut c_void)
        }
        self.userdata = userdata;
    }
    pub fn set_userdata_owned(&mut self, userdata: Option<Vec<u8>>) {
        self.set_userdata(userdata.map(Cow::Owned))
    }
    pub fn set_userdata_borrowed(&mut self, userdata: Option<&'a (impl AsRef<[u8]> + Clone)>) {
        self.set_userdata(userdata.map(AsRef::as_ref).map(Cow::Borrowed))
    }
    /// Returns user data of `ZBarImage`.
    ///
    /// # Examples
    ///
    /// ```
    /// use zbars::prelude::*;
    /// use std::borrow::Cow;
    ///
    /// let userdata = "Hello World".as_bytes();
    /// let mut image1 =
    ///      ZBarImage::from_owned(1, 1, Format::from_label("Y800"), vec![0]).unwrap();
    /// let mut image2 =
    ///      ZBarImage::from_owned(1, 1, Format::from_label("Y800"), vec![0]).unwrap();
    /// image1.set_userdata_borrowed(Some(&userdata));
    /// image2.set_userdata_owned(Some("Hello World".as_bytes().to_owned()));
    /// assert_eq!(image1.userdata().unwrap(), image1.userdata().unwrap());
    /// ```
    pub fn userdata(&self) -> Option<&Cow<'a, [u8]>> { self.userdata.as_ref() }
    /// Writes image on `ZBar format` to the given path.
    pub fn write(&self, path: impl AsRef<Path>) -> ZBarResult<()> {
        match unsafe { zbar_image_write(**self, as_char_ptr(path.as_ref().to_str().unwrap())) } {
            0 => Ok(()),
            e => Err(e.into()),
        }
    }
    /// Not implemented by ZBar itself.
    pub fn read(_path: impl AsRef<Path>) -> Option<Self> {
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

impl<'a> Deref for ZBarImage<'a> {
    type Target = *mut zbar_image_s;
    fn deref(&self) -> &Self::Target { &self.image }
}
impl<'a> Drop for ZBarImage<'a> {
    fn drop(&mut self) { self.set_ref(-1); }
}

#[cfg(feature = "zbar_fork")]
pub mod zbar_fork {
    use super::*;

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

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_size() {
            assert_eq!(
                ZBarImage::new(
                    2, 3, Format::from_label("Y800"), Cow::Owned(vec![0; 2 * 3])
                ).unwrap().size(),
                (2, 3)
            );
        }

        #[test]
        fn test_crop() {
            let image = ZBarImage::new(
                20, 30, Format::from_label("Y800"), Cow::Owned(vec![0; 20 * 30])
            ).unwrap();
            assert_eq!(image.crop(), (0, 0, 20, 30));
        }

        #[test]
        fn test_set_crop_smaller() {
            let mut image = ZBarImage::new(
                20, 30, Format::from_label("Y800"), Cow::Owned(vec![0; 20 * 30])
            ).unwrap();
            image.set_crop(5, 5, 10, 10);
            assert_eq!(image.crop(), (5, 5, 10, 10));
        }

        #[test]
        fn test_set_crop_larger() {
            let mut image = ZBarImage::new(
                20, 30, Format::from_label("Y800"), Cow::Owned(vec![0; 20 * 30])
            ).unwrap();
            image.set_crop(5, 50, 100, 200);
            assert_eq!(image.crop(), (5, 30, 15, 0));
        }
    }
}

#[cfg(feature = "from_image")]
pub mod from_image {
    use image_crate::{
        self,
        GenericImage,
        DynamicImage,
        ImageResult,
        Pixel,
    };
    use super::*;

    lazy_static!(static ref FORMAT: format::Format = Format::from_label("Y800"););

    impl<'a> ZBarImage<'a> {
        /// Creates a `ZBarImage` from the given path.
        ///
        /// This method invokes `ZBarImage::from_dyn_image`. So if the image is already a Luma8
        /// no additional memory will be allocated.
        ///
        /// # Examples
        ///
        /// ```
        /// extern crate zbars;
        ///
        /// use zbars::image::ZBarImage;
        ///
        /// fn main() {
        ///     let image = ZBarImage::from_path("test/code128.gif").unwrap();
        /// }
        /// ```
        pub fn from_path(path: impl AsRef<Path>) -> ImageResult<ZBarImage<'a>> {
            image_crate::open(&path).map(Self::from_dyn_image)
        }

        /// Creates a `ZBarImage` from a `DynamicImage`.
        ///
        /// The given image will owned so zero copy takes place if the image is already a
        /// `DynamicImage::ImageLuma8`. If it is something other than Luma8 a new buffer will be
        /// allocated in order to grayscale the image.
        ///
        /// # Examples
        ///
        /// ```
        /// extern crate zbars;
        /// extern crate image;
        ///
        /// use zbars::image::ZBarImage;
        /// use image::{DynamicImage, ImageBuffer};
        ///
        /// let image = ZBarImage::from_dyn_image(
        ///     DynamicImage::ImageLuma8(
        ///         // small buffer just for demonstration
        ///         ImageBuffer::from_vec(1, 1, vec![0]).unwrap()
        ///     )
        /// );
        /// ```
        pub fn from_dyn_image(image: DynamicImage) -> Self {
            ZBarImage::from_owned(
                image.dimensions().0,
                image.dimensions().1,
                *FORMAT,
                match image {
                        DynamicImage::ImageLuma8(image) => image.into_raw(),
                        other                           => other.to_luma().into_raw(),
                }
            ).unwrap() // Safe to unwrap here
        }

        /// Creates a `ZBarImage` from a `GenericImage`.
        ///
        /// As the pixel representation is not known for a `GenericImage` it will always
        /// be grayscaled and thus a new image buffer will be allocated. If possible use
        /// `ZBarImage::from_dyn_image` instead. Use this if you want to use `GenericImage`
        /// beyond this.
        ///
        /// # Examples
        ///
        /// ```
        /// extern crate zbars;
        /// extern crate image;
        ///
        /// use zbars::image::ZBarImage;
        /// use image::{DynamicImage, ImageBuffer};
        ///
        /// let image = ZBarImage::from_generic_image(
        ///     &DynamicImage::ImageRgb8(
        ///         // small buffer just for demonstration
        ///         ImageBuffer::from_vec(1, 1, vec![0, 0, 0]).unwrap()
        ///     )
        /// );
        /// ```
        pub fn from_generic_image<I>(image: &I) -> Self
            where I: GenericImage + 'static,
                  Vec<u8>: From<Vec<<<I as GenericImage>::Pixel as Pixel>::Subpixel>>
        {
            ZBarImage::from_owned(
                image.dimensions().0,
                image.dimensions().1,
                *FORMAT,
                image_crate::imageops::grayscale(image).into_raw().into()
            ).unwrap() // Safe to unwrap here
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;
        use image_crate::ImageBuffer;

        #[test]
        fn test_from_path() { assert!(ZBarImage::from_path("test/code128.gif").is_ok()); }

        #[test]
        fn test_from_dyn_image_luma() {
            let data = vec![0, 0, 0];
            let image = ZBarImage::from_dyn_image(
                DynamicImage::ImageLuma8(ImageBuffer::from_vec(1, 3, data).unwrap())
            );
            assert_eq!(image.data().as_ref(), &[0, 0, 0]);
        }

        #[test]
        fn test_from_dyn_image_rgb() {
            let data = vec![0, 0, 0];
            let image = ZBarImage::from_dyn_image(
                DynamicImage::ImageRgb8(ImageBuffer::from_vec(1, 1, data).unwrap())
            );
            assert_eq!(image.data().as_ref(), &[0]);
        }

        #[test]
        fn test_from_generic_image_luma() {
            let data = vec![0, 0, 0];
            let image = ZBarImage::from_generic_image(
                &DynamicImage::ImageLuma8(ImageBuffer::from_vec(1, 3, data).unwrap())
            );
            assert_eq!(image.data().as_ref(), &[0, 0, 0]);
        }

        #[test]
        fn test_from_generic_image_rgb() {
            let data = vec![0, 0, 0];
            let image = ZBarImage::from_generic_image(
                &DynamicImage::ImageRgb8(ImageBuffer::from_vec(1, 1, data).unwrap())
            );
            assert_eq!(image.data().as_ref(), &[0]);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn format() {
        let format = Format::from_label("Y800");
        assert_eq!(
            ZBarImage::new(
                2, 3, format, Cow::Owned(vec![0; 2 * 3])
            ).unwrap().format(),
            format
        );
    }

    #[test]
    fn test_sequence_set_and_get() {
        let mut image = ZBarImage::new(
            2, 3, Format::from_label("Y800"), Cow::Owned(vec![0; 2 * 3])
        ).unwrap();
        assert_eq!(image.sequence(), 0);
        image.set_sequence(1);
        assert_eq!(image.sequence(), 1);
        image.set_sequence(999);
        assert_eq!(image.sequence(), 999);
    }

    #[test]
    fn test_set_size_smaller() {
        let mut image = ZBarImage::new(
            20, 30, Format::from_label("Y800"), Cow::Owned(vec![0; 20 * 30])
        ).unwrap();
        image.set_size(10, 12);
        assert_eq!(image.width(), 10);
        assert_eq!(image.height(), 12);
    }

    #[test]
    fn test_set_size_larger() {
        let mut image = ZBarImage::new(
            20, 30, Format::from_label("Y800"), Cow::Owned(vec![0; 20 * 30])
        ).unwrap();
        image.set_size(100, 120);
        assert_eq!(image.width(), 100);
        assert_eq!(image.height(), 120);
    }

    #[test]
    fn test_width() {
        assert_eq!(
            ZBarImage::new(
                2, 3, Format::from_label("Y800"), Cow::Owned(vec![0; 2 * 3])
            ).unwrap().width(),
            2
        );
    }

    #[test]
    fn test_height() {
        assert_eq!(
            ZBarImage::new(
                2, 3, Format::from_label("Y800"), Cow::Owned(vec![0; 2 * 3])
            ).unwrap().height(),
            3
        );
    }

    #[test]
    fn test_data() {
        let buf = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let image = ZBarImage::new(
            3, 4, Format::from_label("Y800"), Cow::Borrowed(&buf)
        ).unwrap();
        assert_eq!(image.data().as_ref(), buf.as_slice());
    }

    #[test]
    fn test_symbols_get_and_set() {
        let mut image = ZBarImage::new(
            20, 30, Format::from_label("Y800"), Cow::Owned(vec![0; 20 * 30])
        ).unwrap();
        assert!(image.symbols().is_none());
        image.set_symbols(None);
        assert!(image.symbols().is_none());
    }

    #[test]
    fn test_first_symbol() {
        assert!(
            ZBarImage::new(
                20, 30, Format::from_label("Y800"), Cow::Owned(vec![0; 20 * 30])
            ).unwrap().first_symbol().is_none()
        );
    }

    #[test]
    fn test_userdata_borrowed_set_and_get() {
        let userdata = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

        let data = vec![0; 20 * 30];

        let mut image1 = ZBarImage::new(
            20, 30, Format::from_label("Y800"), Cow::Owned(data.clone())
        ).unwrap();
        let mut image2 = ZBarImage::new(
            20, 30, Format::from_label("Y800"), Cow::Borrowed(&data)
        ).unwrap();
        let mut image3 = ZBarImage::new(
            20, 30, Format::from_label("Y800"), Cow::Borrowed(&data)
        ).unwrap();

        assert!(image1.userdata().is_none());

        image1.set_userdata(Some(Cow::Borrowed(&userdata)));
        image2.set_userdata(Some(Cow::Borrowed(&userdata)));
        image3.set_userdata(Some(Cow::Owned(userdata.clone())));

        assert_eq!(image1.userdata().unwrap(), image2.userdata().unwrap());
        assert_eq!(image1.userdata().unwrap(), image3.userdata().unwrap());
    }

    #[test]
    fn test_write() {
        let path = std::env::temp_dir().join("zbar_image");
        let image = ZBarImage::new(
            2, 3, Format::from_label("Y800"), Cow::Owned(vec![0; 2 * 3])
        ).unwrap();
        assert!(image.write(&path).is_ok());
    }

    #[test]
    fn test_write_fail() {
        let path = Path::new("/nowhere/nothing");
        let image = ZBarImage::new(
            2, 3, Format::from_label("Y800"), Cow::Owned(vec![0; 2 * 3])
        ).unwrap();
        assert!(image.write(&path).is_err());
    }
}
