use {
    as_char_ptr,
    ffi,
    format::{
        Format,
        Y800
    },
    symbol::ZBarSymbol,
    symbol_set::ZBarSymbolSet,
    ZBarResult,
};
#[cfg(feature = "from_image")]
use image_crate::{
    self,
    DynamicImage,
    GenericImage,
    imageops,
    ImageResult,
    Pixel
};
use std::{
    error::Error,
    rc::Rc,
    fmt,
    os::raw::c_void,
    path::Path,
    ptr,
    slice::from_raw_parts
};

pub type Result<T> = ::std::result::Result<ZBarImage<T>, ZBarImageError>;

unsafe extern fn image_destroyed_handler(_: *mut ffi::zbar_image_s) { trace!("free image"); }

#[derive(Debug)]
pub enum ZBarImageError {
    Len(u32, u32, usize),
}
impl Error for ZBarImageError {}
impl fmt::Display for ZBarImageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ZBarImageError::Len(w, h, l) => write!(
                f,
                "width and height don't match actual data length\
                 => width: {}; height: {}; actual data length: {}",
                w, h, l
            )
        }
    }
}

pub struct ZBarImage<T> {
    image: *mut ffi::zbar_image_s,
    data: Rc<T>,
}
impl<T> ZBarImage<T> {
    fn set_ref(&self, refs: i32) { unsafe { ffi::zbar_image_ref(self.image, refs) } }
    pub(crate) fn image(&self) -> *mut ffi::zbar_image_s { self.image }
    /// Returns the `Format` of the pixels.
    pub fn format(&self) -> Format {
        unsafe { (ffi::zbar_image_get_format(self.image) as u32).into() }
    }
    pub fn sequence(&self) -> u32 { unsafe { ffi::zbar_image_get_sequence(self.image) } }
    /// Returns the width of the image in pixels
    pub fn width(&self) -> u32 { unsafe { ffi::zbar_image_get_width(self.image) } }
    /// Returns the height of the image in pixels
    pub fn height(&self) -> u32 { unsafe { ffi::zbar_image_get_height(self.image) } }

    /// Retrieves the image buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// use zbars::prelude::*;
    ///
    /// let image = ZBarImage::new(1, 1, Format::from_label("Y8"), vec![1]).unwrap();
    /// println!("{:?}", image.data());
    /// ```
    pub fn data(&self) -> &[u8] {
        unsafe {
            from_raw_parts(
                ffi::zbar_image_get_data(self.image) as *const u8,
                ffi::zbar_image_get_data_length(self.image) as usize
            )
        }
    }
    /// Returns an `Option` containing the `SymbolSet` or `None` if the image hasn't been scanned.
    ///
    /// # Examples
    ///
    /// ```
    /// use zbars::prelude::*;
    ///
    /// let image = ZBarImage::new(1, 1, Format::from_label("Y8"), vec![1]).unwrap();
    /// let mut scanner = ZBarImageScanner::builder().build().unwrap();
    /// match scanner.scan_image(&image) {
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
    pub fn symbols(&self) -> Option<ZBarSymbolSet> {
        ZBarSymbolSet::from_raw(unsafe { ffi::zbar_image_get_symbols(self.image) }, self.image)
    }
    pub fn set_symbols(&self, symbols: Option<&ZBarSymbolSet>) {
        unsafe {
            ffi::zbar_image_set_symbols(
                self.image,
                symbols.map_or(ptr::null(), ZBarSymbolSet::symbol_set)
            )
        }
    }
    pub fn first_symbol(&self) -> Option<ZBarSymbol> {
        ZBarSymbol::from_raw(unsafe { ffi::zbar_image_first_symbol(self.image) }, self.image)
    }
    pub fn set_sequence(&self, sequence_num: u32) {
        unsafe { ffi::zbar_image_set_sequence(self.image, sequence_num) }
    }

    /// Just a crop with origin
    pub fn set_size(&self, width: u32, height: u32) {
        unsafe { ffi::zbar_image_set_size(self.image, width, height) }
    }

    /// Writes image on `ZBar format` to the given path.
    pub fn write(&self, path: impl AsRef<Path>) -> ZBarResult<()> {
        match unsafe { ffi::zbar_image_write(self.image, as_char_ptr(path.as_ref().to_str().unwrap())) } {
            0 => Ok(()),
            e => Err(e.into()),
        }
    }
    /// Not implemented by ZBar itself.
    pub fn read(_path: impl AsRef<Path>) -> Option<Self> {
        //TODO: zbar.h days: TBD
//        ZBarImage {
//            image: unsafe {
//                zbar_image_read(
//                    path.as_ref().as_os_str().to_str().unwrap().as_bytes().as_ptr() as *mut i8
//                )
//            }
//        }
        unimplemented!("zbar.h days: zbar_image_read TBD")
    }
}
#[cfg(feature = "zbar_fork")]
impl<T> ZBarImage<T> {
    pub fn size(&self) -> (u32, u32) {
        unsafe {
            let mut dim = (0, 0);
            ffi::zbar_image_get_size(self.image, &mut dim.0 as *mut u32, &mut dim.1 as *mut u32);
            dim
        }
    }

    pub fn crop(&self) -> (u32, u32, u32, u32) {
        unsafe {
            let mut crop = (0, 0, 0, 0);
            ffi::zbar_image_get_crop(
                self.image,
                &mut crop.0 as *mut u32, &mut crop.1 as *mut u32,
                &mut crop.2 as *mut u32, &mut crop.3 as *mut u32
            );
            crop
        }
    }

    pub fn set_crop(&self, x: u32, y: u32, width: u32, height: u32) {
        unsafe { ffi::zbar_image_set_crop(self.image, x, y, width, height) }
    }
}

impl<T> ZBarImage<T> where T: AsRef<[u8]> {
    /// ```compile_fail
    /// use zbars::prelude::*;
    ///
    /// let image = {
    ///     let data = vec![];
    ///     ZBarImage::new(0, 0, Y800, &data)
    /// };
    /// ```
    pub fn new(width: u32, height: u32, format: Format, data: T) -> Result<T> {
        if width as usize * height as usize == data.as_ref().len() {
            unsafe {
                let image = ffi::zbar_image_create();
                ffi::zbar_image_set_format(image, format.value().into());
                ffi::zbar_image_set_size(image, width, height);
                ffi::zbar_image_set_data(
                    image,
                    data.as_ref().as_ptr() as *mut c_void,
                    (data.as_ref().len() as u32).into(),
                    Some(image_destroyed_handler)
                );
                Ok(Self { image, data: data.into() })
            }
        } else {
            Err(ZBarImageError::Len(width, height, data.as_ref().len()))
        }
    }
}

#[cfg(feature = "from_image")]
impl ZBarImage<Vec<u8>> {
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
    pub fn from_path(path: impl AsRef<Path>) -> ImageResult<Self> {
        image_crate::open(&path).map(Self::from)
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
    pub fn from_dyn_image(image: DynamicImage) -> Self { image.into() }

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
        Self::create_image(image.dimensions(), imageops::grayscale(image).into_raw().into())
    }

    fn create_image(dimensions: (u32, u32), data: Vec<u8>) -> Self {
        ZBarImage::new(dimensions.0, dimensions.1, Y800, data).unwrap() // Safe to unwrap here
    }
}
#[cfg(feature = "from_image")]
impl From<DynamicImage> for ZBarImage<Vec<u8>> {
    fn from(image: DynamicImage) -> Self {
        Self::create_image(image.dimensions(), match image {
            DynamicImage::ImageLuma8(image) => image,
            other                           => other.to_luma()
        }.into_raw())
    }
}

impl<T> Clone for ZBarImage<T> {
    fn clone(&self) -> Self {
        let image = Self { image: self.image, data: self.data.clone() };
        image.set_ref(1);
        image
    }
}

impl<T> Drop for ZBarImage<T> {
    fn drop(&mut self) { self.set_ref(-1) }
}

#[cfg(test)]
mod test {
    #[cfg(feature = "from_image")]
    use image_crate::ImageBuffer;
    use super::*;

    #[test]
    fn test_clone_owned() {
        let image =  ZBarImage::new(2, 3, Y800, vec![0; 2 * 3]).unwrap();
        {
            let _clone = image.clone();
        }
        assert_eq!(image.data(), &[0; 2 * 3])
    }

    #[test]
    fn test_clone_ref() {
        let data = vec![0; 2 * 3];
        let image =  ZBarImage::new(2, 3, Y800, &data).unwrap();
        {
            let _clone = image.clone();
        }
        assert_eq!(image.data(), &[0; 2 * 3])
    }

    #[test]
    fn format() {
        let format = Format::from_label("Y800");
        let data = vec![0; 2 * 3];
        assert_eq!(
            ZBarImage::new(
                2, 3, format, &data
            ).unwrap().format(),
            format
        );
    }

    #[test]
    fn test_sequence_set_and_get() {
        let image = ZBarImage::new(2, 3, Format::from_label("Y800"), vec![0; 2 * 3])
            .unwrap();
        assert_eq!(image.sequence(), 0);
        image.set_sequence(1);
        assert_eq!(image.sequence(), 1);
        image.set_sequence(999);
        assert_eq!(image.sequence(), 999);
    }

    #[test]
    fn test_set_size_smaller() {
        let image = ZBarImage::new(20, 30, Format::from_label("Y800"), vec![0; 20 * 30])
            .unwrap();
        image.set_size(10, 12);
        assert_eq!(image.width(), 10);
        assert_eq!(image.height(), 12);
    }

    #[test]
    fn test_set_size_larger() {
        let image = ZBarImage::new(20, 30, Format::from_label("Y800"), vec![0; 20 * 30])
            .unwrap();
        image.set_size(100, 120);
        assert_eq!(image.width(), 100);
        assert_eq!(image.height(), 120);
    }

    #[test]
    fn test_width() {
        assert_eq!(
            ZBarImage::new(2, 3, Format::from_label("Y800"), vec![0; 2 * 3])
                .unwrap()
                .width(),
            2
        );
    }

    #[test]
    fn test_height() {
        assert_eq!(
            ZBarImage::new(2, 3, Format::from_label("Y800"), vec![0; 2 * 3])
                .unwrap()
                .height(),
            3
        );
    }

    #[test]
    fn test_data() {
        let buf: Vec<u8> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let image = ZBarImage::new(3, 4, Format::from_label("Y800"), &buf[..]).unwrap();
        assert_eq!(image.data().as_ref(), buf.as_slice());
    }

    #[test]
    fn test_symbols_get_and_set() {
        let image = ZBarImage::new(20, 30, Format::from_label("Y800"), vec![0; 20 * 30])
            .unwrap();
        assert!(image.symbols().is_none());
        image.set_symbols(None);
        assert!(image.symbols().is_none());
    }

    #[test]
    fn test_first_symbol() {
        assert!(
            ZBarImage::new(20, 30, Format::from_label("Y800"), vec![0; 20 * 30])
                .unwrap()
                .first_symbol()
                .is_none()
        );
    }

    #[test]
    fn test_write() {
        let path = ::std::env::temp_dir().join("zbar_image");
        let image = ZBarImage::new(2, 3, Format::from_label("Y800"), vec![0; 2 * 3])
            .unwrap();
        assert!(image.write(&path).is_ok());
    }

    #[test]
    fn test_write_fail() {
        let path = Path::new("/nowhere/nothing");
        let image = ZBarImage::new(2, 3, Format::from_label("Y800"), vec![0; 2 * 3])
            .unwrap();
        assert!(image.write(&path).is_err());
    }

    #[test]
    #[cfg(feature = "zbar_fork")]
    fn test_size() {
        assert_eq!(
            ZBarImage::new(2, 3, Format::from_label("Y800"), vec![0; 2 * 3])
                .unwrap()
                .size(),
            (2, 3)
        );
    }

    #[test]
    #[cfg(feature = "zbar_fork")]
    fn test_crop() {
        let image = ZBarImage::new(20, 30, Format::from_label("Y800"), vec![0; 20 * 30])
            .unwrap();
        assert_eq!(image.crop(), (0, 0, 20, 30));
    }

    #[test]
    #[cfg(feature = "zbar_fork")]
    fn test_set_crop_smaller() {
        let image = ZBarImage::new(20, 30, Format::from_label("Y800"), vec![0; 20 * 30])
            .unwrap();
        image.set_crop(5, 5, 10, 10);
        assert_eq!(image.crop(), (5, 5, 10, 10));
    }

    #[test]
    #[cfg(feature = "zbar_fork")]
    fn test_set_crop_larger() {
        let image = ZBarImage::new(20, 30, Format::from_label("Y800"), vec![0; 20 * 30])
            .unwrap();
        image.set_crop(5, 50, 100, 200);
        assert_eq!(image.crop(), (5, 30, 15, 0));
    }

    #[test]
    #[cfg(feature = "from_image")]
    fn test_from_path() { assert!(ZBarImage::from_path("test/code128.gif").is_ok()); }

    #[test]
    #[cfg(feature = "from_image")]
    fn test_from_dyn_image_luma() {
        let data = vec![0, 0, 0];
        let image = ZBarImage::from_dyn_image(
            DynamicImage::ImageLuma8(ImageBuffer::from_vec(1, 3, data).unwrap())
        );
        assert_eq!(image.data(), &[0, 0, 0]);
    }

    #[test]
    #[cfg(feature = "from_image")]
    fn test_from_dyn_image_rgb() {
        let data = vec![0, 0, 0];
        let image = ZBarImage::from_dyn_image(
            DynamicImage::ImageRgb8(ImageBuffer::from_vec(1, 1, data).unwrap())
        );
        assert_eq!(image.data(), &[0]);
    }

    #[test]
    #[cfg(feature = "from_image")]
    fn test_from_generic_image_luma() {
        let data = vec![0, 0, 0];
        let image = ZBarImage::from_generic_image(
            &DynamicImage::ImageLuma8(ImageBuffer::from_vec(1, 3, data).unwrap())
        );
        assert_eq!(image.data(), &[0, 0, 0]);
    }

    #[test]
    #[cfg(feature = "from_image")]
    fn test_from_generic_image_rgb() {
        let data = vec![0, 0, 0];
        let image = ZBarImage::from_generic_image(
            &DynamicImage::ImageRgb8(ImageBuffer::from_vec(1, 1, data).unwrap())
        );
        assert_eq!(image.data(), &[0]);
    }
}
