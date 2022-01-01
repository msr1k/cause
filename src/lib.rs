//! [Cause] is a tiny generic implementation of the [std::error::Error] trait.
//!
//! It takes 1 type parameter(`T: Debug`) who describes what error type happened.
//!
//! It is dereferencable as `&T`.
//!
//! And if you use macro [cause], it automatically stores some extra information,
//! the filename and line number, only when it was compiled with `debug_assertions`.
//!
//! # Examples
//!
//! ```
//! #[derive(Debug, PartialEq, Eq)]
//! pub enum ErrorType {
//!     InvalidArgumentsError,
//!     InternalError,
//!     NotFoundError,
//! }
//!
//! use ErrorType::*;
//! use cause::Cause;
//! use std::error::Error;
//!
//! // It creates an instance of `Cause<ErrorType>`
//! let cause = Cause::new(InternalError);
//! assert_eq!(cause.to_string(), "InternalError".to_string());
//! assert!(cause.message().is_none());
//! assert!(cause.source().is_none());
//!
//! // It is dereferencable.
//! assert_eq!(*cause, InternalError);
//!
//! let http_status_code = match *cause {
//!     InternalError => 500,
//!     InvalidArgumentsError => 400,
//!     NotFoundError => 404,
//! };
//! assert_eq!(http_status_code, 500);
//!
//! // set the message:
//! let cause = Cause::new(InvalidArgumentsError).msg("oops!");
//! assert_eq!(cause.to_string(), "InvalidArgumentsError: oops!".to_string());
//! assert_eq!(cause.message(), Some(&"oops!".to_string()));
//! assert!(cause.source().is_none());
//!
//! // set the source of this error (any error type can be set with `src()`):
//! let cause = Cause::new(InternalError).src(Cause::new(NotFoundError));
//! assert_eq!(
//!     cause.to_string(),
//!     "InternalError\n\nCaused by:\n    NotFoundError\n".to_string()
//! );
//! assert!(cause.message().is_none());
//! assert!(cause.source().is_some());
//!
//! // an example of Cause who have a standard io error.
//! use std::io::Error as IoErr;
//! use std::io::ErrorKind;
//! let io_err = IoErr::new(ErrorKind::Other, "oh no!");
//! println!("{}", Cause::new(InternalError).src(io_err).msg("internal error caused by io error"));
//! 
//! // a couple of macro examples
//! use cause::cause;
//!
//! let cause = cause!(InternalError);
//! println!("{}", cause);
//!   // => "InternalError" on release build
//!   // => "InternalError: [lib.rs:59]" on debug build
//!
//! let cause = cause!(NotFoundError, "There is no such contents.");
//! println!("{}", cause);
//!   // => "InternalError: There is no such contents." on release build
//!   // => "InternalError: There is no such contents. [lib.rs:59]" on debug build
//!
//! ```

/// A macro to create a [Cause] which situationally appends filename and line number information at the end of message.
#[macro_export]
macro_rules! cause {
    ($type:expr) => {
        if cfg!(debug_assertions) {
            Cause::new($type).msg(format!("[{}:{}]", file!(), line!()))
        } else {
            Cause::new($type)
        }
    };
    ($type:expr, $msg:expr) => {
        if cfg!(debug_assertions) {
            Cause::new($type).msg(format!("{} [{}:{}]", $msg, file!(), line!()))
        } else {
            Cause::new($type).msg($msg)
        }
    };
}

use std::error::Error;

/// A tiny generic implementation of the [std::error::Error] trait.
#[derive(Debug)]
pub struct Cause<T> {
    cause: T,
    msg: Option<String>,
    src: Option<Box<dyn Error + Send + 'static>>,
}

impl<T> Cause<T> {

    /// Create a [Cause] instance with its `cause`.
    pub fn new(cause: T) -> Self {
        Self {
            cause,
            msg: None,
            src: None,
        }
    }

    /// Set the message for this error.
    pub fn msg(mut self, msg: impl Into<String>) -> Self {
        self.msg = Some(msg.into());
        self
    }

    /// Set the lower-level source of this error, if any.
    pub fn src(mut self, src: impl Error + Send + 'static) -> Self {
        self.src = Some(Box::new(src));
        self
    }

    /// Get a reference to the `cause`
    pub fn cause(&self) -> &T {
        &self.cause
    }

    /// Get a reference to the message
    pub fn message(&self) -> Option<&String> {
        match self.msg.as_ref() {
            Some(msg) => Some(msg),
            None => None,
        }
    }
}

use std::fmt::Display;
use std::fmt::Debug;

impl<T: Debug> Display for Cause<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let mut message: String = match self.msg.as_ref() {
            Some(m) => format!("{:?}: {}", self.cause, m),
            None => format!("{:?}", self.cause),
        };
        if let Some(ref s) = self.src {
            message.push_str(&format!("\n\nCaused by:\n    {}\n", s));
        }
        write!(f, "{}", message)
    }
}

impl<T: Debug> Error for Cause<T> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self.src {
            Some(e) => Some(e.as_ref()),
            None => None,
        }
    }
}

use std::ops::Deref;

impl<T: Debug> Deref for Cause<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.cause
    }
}

#[cfg(test)]
mod tests {

    #[derive(Debug, PartialEq, Eq)]
    pub enum ErrorType {
        InvalidArgumentsError,
        InternalError,
        UnknownError,
    }

    #[derive(Debug)]
    enum AlphabetError {
        AError,
        BError,
        CError,
    }

    #[test]
    fn it_works() {
        use ErrorType::*;
        use AlphabetError::*;
        use super::Cause;

        let cause = Cause::new(InternalError);
        let http_status_code = match *cause {
            InternalError => 500,
            InvalidArgumentsError => 400,
            _ => 418
        };
        assert_eq!(*cause, InternalError);
        assert_eq!(http_status_code, 500);

        println!("{}", Cause::new(InternalError).msg("oh no!"));
        println!("{}", Cause::new(InvalidArgumentsError).msg("oops"));

        println!("{}", Cause::new(AError));
        println!("{}", Cause::new(BError));

        println!("{}", Cause::new(InternalError).src(Cause::new(UnknownError).msg("nested")).msg("something went wrong"));
        println!("{}", Cause::new(InternalError).src(Cause::new(CError)).msg( "another nested"));

        use std::io::{Error, ErrorKind};
        let io_err = Error::new(ErrorKind::Other, "oh no!");
        println!("{}", Cause::new(InternalError).src(io_err).msg("internal error caused by io error"));
    }
}
