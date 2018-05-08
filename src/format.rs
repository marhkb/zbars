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
            label: {
                let bytes = unsafe { transmute::<u32, [u8; 4]>(fourcc as u32) };
                Cow::Owned(from_utf8(&bytes).unwrap().to_owned())
            },
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
        Format {
            fourcc: {
                // pad 4
                let label_format = format!("{:4}", label.as_ref());
                let bytes = label_format.as_bytes();
                unsafe { transmute::<[u8; 4], u32>([bytes[0], bytes[1], bytes[2], bytes[3]]) }
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
