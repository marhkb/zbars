use std::{
    borrow::Cow,
    mem::transmute,
};

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
/// let format = Format::from_label(Cow::Borrowed("Y800"));
/// println!("{}", format.fourcc());
///
/// // create Format that owns the given label
/// let format = Format::from_label(Cow::Owned(String::from("Y800")));
/// println!("{}", format.fourcc());
///
/// // create Format from FOURCC value
/// let format = Format::from_fourcc(0x30303859);
/// println!("{}", format.label());
/// ```
#[derive(Debug)]
pub struct Format<'a> {
    /// FOURCC value
    fourcc: u32,
    /// FOURCC label (e.g. Y800)
    label: Cow<'a, str>,
}
impl<'a> Format<'a> {
    /// Creates a `Format` from the given FOURCC value.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use zbars::prelude::Format;
    ///
    /// let format = Format::from_fourcc(0x30303859);
    /// println!("{}", format.label());
    /// println!("{}", format.fourcc());
    ///
    /// ```
    pub fn from_fourcc(fourcc: u32) -> Self  {
        use std::str::from_utf8;

        Format {
            fourcc,
            label: Cow::Owned(
                from_utf8(&unsafe { transmute::<_, [u8; 4]>(fourcc) }).unwrap().trim().to_owned()
            ),
        }
    }
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
    /// let format = Format::from_label(Cow::Borrowed("Y800"));
    /// println!("{}", format.label());
    /// println!("{}", format.fourcc());
    ///
    /// // create Format that owns the given label
    /// let format = Format::from_label(Cow::Owned(String::from("Y800")));
    /// println!("{}", format.label());
    /// println!("{}", format.fourcc());
    /// ```
    pub fn from_label(label: Cow<'a, str>) -> Self {
        use std::borrow::Borrow;

        Format {
            fourcc: {
                let byte_slice = label.as_ref().as_bytes();
                let mut bytes = [32; 4];
                for i in 0..byte_slice.len() {
                    bytes[i] = byte_slice[i];
                }
                unsafe { transmute(bytes) }
            },
            label,
        }
    }
    /// Returns the FOURCC value for this `Format`
    pub fn fourcc(&self) -> u32 { self.fourcc }
    /// Returns the FOURCC label for this `Format`
    pub fn label(&'a self) -> &'a str {
        match self.label {
            Cow::Owned(ref label) => label,
            Cow::Borrowed(label)  => label,
        }
    }

}
impl<'a> PartialEq for Format<'a> {
    fn eq(&self, other: &Self) -> bool { self.fourcc == other.fourcc }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_fourcc() {
        assert_eq!(Format::from_fourcc(0x564E5559).label(), "YUNV");
        assert_eq!(Format::from_fourcc(0x564E5559).fourcc(), 0x564E5559);
        assert_eq!(Format::from_fourcc(0x20203859).label(), "Y8");
        assert_eq!(Format::from_fourcc(0x20203859).fourcc(), 0x20203859);
    }

    #[test]
    fn test_from_label() {
        assert_eq!(Format::from_label(Cow::Borrowed("YUNV")).fourcc(), 0x564E5559);
        assert_eq!(Format::from_label(Cow::Owned("Y8".into())).fourcc(), 0x20203859);

    }

    #[test]
    fn test_label() {
        assert_eq!(Format::from_label(Cow::Borrowed("YUNV")).label(), "YUNV");
    }

    #[test]
    fn test_fourcc() {
        assert_eq!(Format::from_fourcc(0x564E5559).fourcc(), 0x564E5559);
    }

    #[test]
    fn test_eq() {
        assert_eq!(Format::from_label(Cow::Borrowed("YUNV")),
                   Format::from_label(Cow::Borrowed("YUNV")));
        assert_eq!(Format::from_label(Cow::Borrowed("YUNV")),
                   Format::from_label(Cow::Owned("YUNV".into())));
        assert_eq!(Format::from_label(Cow::Borrowed("YUNV")),
                   Format::from_fourcc(0x564E5559));
        assert_eq!(Format::from_label(Cow::Owned("YUNV".into())),
                   Format::from_fourcc(0x564E5559));
    }
}
