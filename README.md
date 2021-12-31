# cause

[Cause] is a tiny generic implementation of the [std::error::Error] trait.

It takes 1 type parameter(`T: Debug`) who describes what error type happened.

It is dereference-able as `&T`.

And if you use macro [cause], it automatically stores some extra information,
the filename and line number, only when it was compiled with `debug_assertions`.

## Examples

```rust
#[derive(Debug, PartialEq, Eq)]
pub enum ErrorType {
    InvalidArgumentsError,
    InternalError,
    NotFoundError,
}

use ErrorType::*;
use cause::Cause;

// It creates an instance of `Cause<ErrorType>`
let cause = Cause::new(InternalError);
assert_eq!(format!("{}", cause), "InternalError".to_string());

// It is dereference-able.
assert_eq!(*cause, InternalError);

let http_status_code = match *cause {
    InternalError => 500,
    InvalidArgumentsError => 400,
    NotFoundError => 404,
};
assert_eq!(http_status_code, 500);

// with a message:
let cause = Cause::new(InvalidArgumentsError).msg("oops!");
assert_eq!(
    format!("{}", cause),
    "InvalidArgumentsError: oops!".to_string()
);

// with source of error (any error type can be set with `src()`):
let cause = Cause::new(InternalError).src(Cause::new(NotFoundError));
assert_eq!(
    format!("{}", cause),
    "InternalError\n\nCaused by:\n    NotFoundError\n".to_string()
);

// an example of Cause who have a standard io error.
use std::io::{Error, ErrorKind};
let io_err = Error::new(ErrorKind::Other, "oh no!");
println!("{}", Cause::new(InternalError).src(io_err).msg("internal error caused by io error"));

// a couple of macro examples
use cause::cause;

let cause = cause!(InternalError);
println!("{}", cause);
  // => "InternalError" on release build
  // => "InternalError: [lib.rs:59]" on debug build

let cause = cause!(NotFoundError, "There is no such contents.");
println!("{}", cause);
  // => "InternalError: There is no such contents." on release build
  // => "InternalError: There is no such contents. [lib.rs:59]" on debug build

```

License: MIT
