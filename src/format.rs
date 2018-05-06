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
/// use zbar_rs::prelude::Format;
///
/// // create Format that borrows the given label
/// let format = Format::from_label_slice("Y800");
/// println!("{}", format.fourcc());
///
/// // create Format that owns the given label
/// let format = Format::from_label_owned("Y800");
/// println!("{}", format.fourcc());
///
/// // create Format from FOURCC value
/// let format = Format::from_fourcc(0x30303859_u32);
/// println!("{}", format.label());
/// ```
#[derive(Debug)]
pub struct Format<'a> {
    /// FOURCC label (e.g. Y800)
    label: Cow<'a, str>,
    /// FOURCC value
    fourcc: u64,
}
impl<'a> Format<'a> {
    /// Creates a `Format` from the given FOURCC label and lets `Format` own that label.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use zbar_rs::prelude::Format;
    ///
    /// let format = Format::from_label_owned("Y800");
    /// println!("{}", format.label());
    /// println!("{}", format.fourcc());
    /// ```
    pub fn from_label_owned<T>(label: T) -> Self where T: ToString + AsRef<str> {
        Format { label: Cow::Owned(label.to_string()), fourcc: fourcc(label) }
    }
    /// Creates a `Format` from the given FOURCC label and lets `Format` borrow that label.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use zbar_rs::prelude::Format;
    ///
    /// let format = Format::from_label_slice("Y800");
    /// println!("{}", format.label());
    /// println!("{}", format.fourcc());
    /// ```
    pub fn from_label_slice<T>(label: &'a T) -> Self where T: ?Sized + AsRef<str> {
        Format { label: Cow::Borrowed(label.as_ref()), fourcc: fourcc(label) }
    }
    /// Creates a `Format` from the given FOURCC value.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use zbar_rs::prelude::Format;
    ///
    /// let format = Format::from_fourcc(0x30303859_u32);
    /// println!("{}", format.label());
    /// println!("{}", format.fourcc());
    ///
    /// ```
    pub fn from_fourcc<T>(fourcc: T) -> Self where T: Into<u64> {
        use std::str::from_utf8;

        let fourcc = fourcc.into();
        let bytes = unsafe { transmute::<u32, [u8; 4]>(fourcc as u32) };
        Format { label: Cow::Owned(from_utf8(&bytes).unwrap().to_owned()), fourcc }
    }
    /// Returns the FOURCC label for this `Format`
    pub fn label(&'a self) -> &'a str {
        use self::Cow::*;

        match self.label {
            Owned(ref label) => label,
            Borrowed(label)  => label,
        }
    }
    /// Returns the FOURCC value for this `Format`
    pub fn fourcc(&self) -> u64 { self.fourcc }
}
impl<'a> PartialEq for Format<'a> {
    fn eq(&self, other: &Self) -> bool { self.fourcc == other.fourcc }
}

fn fourcc<T>(label: T) -> u64 where T: AsRef<str> {
    // pad 4
    let format = format!("{:4}", label.as_ref());
    let bytes = format.as_bytes();
    unsafe { transmute::<[u8; 4], u32>([bytes[0], bytes[1], bytes[2], bytes[3]]) as u64 }
}
