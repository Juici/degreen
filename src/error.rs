use std::error::Error as StdError;
use std::fmt;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(::std::io::Error),
    Clap(::clap::Error),
    Msg(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Error::*;
        match *self {
            Io(ref error) => write!(f, "{}", error),
            Clap(ref error) => write!(f, "{}", error),
            Msg(ref s) => f.write_str(s),
        }
    }
}

impl StdError for Error {
    fn cause(&self) -> Option<&StdError> {
        use self::Error::*;
        match *self {
            Io(ref error) => Some(error),
            Clap(ref error) => Some(error),
            _ => None,
        }
    }
}

#[macro_export]
macro_rules! error {
    ($fmt:expr) => (Err($crate::error::Error::Msg(format!($fmt))));
    ($fmt:expr, $($arg:tt)*) => (Err($crate::error::Error::Msg(format!($fmt, $($arg)*))));
}

macro_rules! impl_from {
    ($($name:ident($err:ty))*) => ($(
        impl From<$err> for Error {
            fn from(error: $err) -> Self {
                Error::$name(error)
            }
        }
    )*)
}

impl_from! {
    Io(::std::io::Error)
    Clap(::clap::Error)
}
