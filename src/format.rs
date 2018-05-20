use super::*;

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
/// use std::borrow::Cow;
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
    pub fn from_value(value: u32) -> Self { Format(value) }
    /// Creates a `Format` from the given FOURCC label and lets `Format` borrow that label.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use zbars::prelude::Format;
    /// use std::borrow::Cow;
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
    pub fn from_label(label: impl AsRef<str>) -> Self {
        Format({
            let byte_slice = label.as_ref().as_bytes();
            let mut bytes = [32; 4];
            for i in 0..byte_slice.len() {
                bytes[i] = byte_slice[i];
            }
            unsafe { mem::transmute(bytes) }
        })
    }

    /// Returns the FOURCC value for this `Format`
    pub fn value(&self) -> u32 { self.0 }
    pub fn as_label(&self) -> String { self.to_string() }
}
impl Deref for Format {
    type Target = u32;
    fn deref(&self) -> &Self::Target { &self.0 }
}
impl ToString for Format {
    fn to_string(&self) -> String {
        std::str::from_utf8(&unsafe { mem::transmute::<_, [u8; 4]>(*self) })
            .unwrap().trim().to_owned()
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
