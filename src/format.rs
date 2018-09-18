use std::{
    mem,
    str::from_utf8,
};

pub const Y800: Format = Format(0x5945_5247);
pub const Y8: Format = Format(0x2020_3859);

/// A FOURCC code (https://www.fourcc.org/fourcc.php)
///
/// The type `Format` holds the FOURCC label (e.g. Y800) and the corresponding FOURCC value.
/// It can be constructed from an label or a FOURCC value.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use zbars::prelude::Format;
///
/// // create Format that borrows the given label
/// let format = Format::from_label("Y800");
/// println!("{}", format.value());
///
/// // create Format that owns the given label
/// let format = Format::from_label("Y800");
/// println!("{}", format.value());
///
/// // create Format from FOURCC value
/// let format = Format::from_value(0x30303859);
/// println!("{}", format.as_label());
/// ```
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Format(u32);
impl Format {
    /// Creates a `Format` from the given FOURCC value.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use zbars::prelude::Format;
    ///
    /// let format = Format::from_value(0x30303859);
    /// println!("{}", format.as_label());
    /// println!("{}", format.value());
    ///
    /// ```
    pub fn from_value(value: u32) -> Self { value.into() }
    /// Creates a `Format` from the given FOURCC label and lets `Format` borrow that label.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use zbars::prelude::Format;
    ///
    /// // create Format that borrows the given label
    /// let format = Format::from_label("Y800");
    /// println!("{}", format.as_label());
    /// println!("{}", format.value());
    ///
    /// // create Format that owns the given label
    /// let format = Format::from_label("Y800");
    /// println!("{}", format.as_label());
    /// println!("{}", format.value());
    /// ```
    pub fn from_label(label: &(impl AsRef<str> + ?Sized)) -> Self { label.into() }

    /// Returns the FOURCC value for this `Format`
    pub fn value(&self) -> u32 { self.into() }
    pub fn as_label(&self) -> String { self.to_string() }
}

impl From<u32> for Format {
    fn from(value: u32) -> Self { Format(value) }
}
impl<'a> From<&'a Format> for u32 {
    fn from(format: &'a Format) -> Self { format.0 }
}

impl<'a, T> From<&'a T> for Format where T: AsRef<str> + ?Sized {
    fn from(label: &'a T) -> Self {
        Format({
            let byte_slice = label.as_ref().as_bytes();
            let mut bytes = [32; 4];
            bytes[..byte_slice.len()].clone_from_slice(byte_slice);
            unsafe { mem::transmute(bytes) }
        })
    }
}
impl ToString for Format {
    fn to_string(&self) -> String {
        from_utf8(&unsafe { mem::transmute::<_, [u8; 4]>(*self) }).unwrap().trim().to_owned()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_value() {
        assert_eq!(Format::from_value(0x564E5559).as_label(), "YUNV");
        assert_eq!(Format::from_value(0x564E5559).value(), 0x564E5559);
        assert_eq!(Format::from_value(0x20203859).as_label(), "Y8");
        assert_eq!(Format::from_value(0x20203859).value(), 0x20203859);
    }

    #[test]
    fn test_from_label() {
        assert_eq!(Format::from_label("YUNV").value(), 0x564E5559);
        assert_eq!(Format::from_label("Y8").value(), 0x20203859);

    }

    #[test]
    fn test_label() {
        assert_eq!(Format::from_label("YUNV").as_label(), "YUNV");
    }

    #[test]
    fn test_value() {
        assert_eq!(Format::from_value(0x564E5559).value(), 0x564E5559);
    }

    #[test]
    fn test_eq() {
        assert_eq!(Format::from_label("YUNV"), Format::from_label("YUNV"));
        assert_eq!(Format::from_label("YUNV"), Format::from_value(0x564E5559));
    }
}
