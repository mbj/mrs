/// Represent a path that was verified to point to a file.
///
/// Allows to deduplicate some pre-checks where otherwise `PathBuf`
/// would be used, needing a per `PathBuf` input check.
///
/// Beware of time-of-check-vs-time-of-use issues.
#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct FileBuf(std::path::PathBuf);

impl std::str::FromStr for FileBuf {
    type Err = &'static str;

    /// Parse file buf from stri
    ///
    /// Examples:
    ///
    /// File exists
    ///
    /// ```
    /// # use file_buf::*;
    ///
    /// let file_buf = <FileBuf as std::str::FromStr>::from_str("./Cargo.toml").unwrap();
    ///
    /// assert_eq!(std::path::PathBuf::from("./Cargo.toml"), file_buf.as_ref());
    /// ```
    ///
    /// File does not exist
    ///
    /// ```
    /// # use file_buf::*;
    ///
    /// assert_eq!(
    ///     Err("path is not a file"),
    ///     <FileBuf as std::str::FromStr>::from_str("does_not_exist.txt"),
    /// )
    /// ```
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let path = std::path::PathBuf::from(value);

        if path.is_file() {
            Ok(FileBuf(path))
        } else {
            Err("path is not a file")
        }
    }
}

impl AsRef<std::path::Path> for FileBuf {
    /// Return path reference
    ///
    /// ```
    /// # use file_buf::*;
    ///
    /// let file_buf = <FileBuf as std::str::FromStr>::from_str("./Cargo.toml").unwrap();
    ///
    /// assert_eq!(std::path::PathBuf::from("./Cargo.toml"), file_buf.as_ref());
    /// ```
    fn as_ref(&self) -> &std::path::Path {
        &self.0
    }
}

impl FileBuf {
    /// Return file name
    ///
    /// ```
    /// # use file_buf::*;
    ///
    /// let file_buf = <FileBuf as std::str::FromStr>::from_str("./Cargo.toml").unwrap();
    ///
    /// assert_eq!("Cargo.toml", file_buf.file_name());
    /// ```
    pub fn file_name(&self) -> &std::ffi::OsStr {
        self.0.file_name().unwrap()
    }

    /// Create file buf for cases the file existance cannot be checked ahead of time
    ///
    /// Examples:
    ///
    /// Still a valid file by syntax.
    ///
    /// ```
    /// # use file_buf::*;
    ///
    /// let file_buf = FileBuf::from_path_unchecked_existance("does_not_exist.txt".into()).unwrap();
    ///
    /// assert_eq!(
    ///     std::path::PathBuf::from("does_not_exist.txt"),
    ///     file_buf.as_ref()
    /// );
    /// ```
    ///
    /// Not a faild file by syntax.
    ///
    /// ```
    /// # use file_buf::*;
    ///
    /// assert_eq!(None, FileBuf::from_path_unchecked_existance("/".into()))
    /// ```
    #[allow(clippy::manual_map)] // lifetime of yielded pointer leaks into using arg
    pub fn from_path_unchecked_existance(path: std::path::PathBuf) -> Option<Self> {
        match path.file_name() {
            Some(_) => Some(Self(path)),
            None => None,
        }
    }

    /// Read to string
    pub fn read_to_string(&self) -> String {
        std::fs::read_to_string(&self.0).unwrap()
    }

    /// Return str representation
    ///
    /// ### Panics
    ///
    /// if file-buf string is not valid UTF8
    pub fn to_str(&self) -> &str {
        self.0.to_str().unwrap()
    }
}

#[macro_export]
macro_rules! file_buf {
    ($string: literal) => {
        <file_buf::FileBuf as std::str::FromStr>::from_str($string).unwrap()
    };
}

#[macro_export]
macro_rules! file_buf_unchecked_existance {
    ($string: literal) => {
        file_buf::FileBuf::from_path_unchecked_existance(std::path::PathBuf::from($string)).unwrap()
    };
}
