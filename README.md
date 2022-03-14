# cause

[Cause] is a tiny generic implementation of the [std::error::Error] trait.

It takes 1 type parameter(`T: Debug`) who describes what error type happened.

It is dereferencable as `&T`.

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
use std::error::Error;

// It creates an instance of `Cause<ErrorType>`
let cause = Cause::new(InternalError);
assert_eq!(cause.to_string(), "InternalError".to_string());
assert!(cause.message().is_none());
assert!(cause.source().is_none());

// It is dereferencable.
assert_eq!(*cause, InternalError);

let http_status_code = match *cause {
    InternalError => 500,
    InvalidArgumentsError => 400,
    NotFoundError => 404,
};
assert_eq!(http_status_code, 500);

// set the message:
let cause = Cause::new(InvalidArgumentsError).msg("oops!");
assert_eq!(cause.to_string(), "InvalidArgumentsError: oops!".to_string());
assert_eq!(cause.message(), Some(&"oops!".to_string()));
assert!(cause.source().is_none());

// set the source of this error (any error type can be set with `src()`):
let cause = Cause::new(InternalError).src(Cause::new(NotFoundError));
assert_eq!(
    cause.to_string(),
    "InternalError\n\nCaused by:\n    NotFoundError\n".to_string()
);
assert!(cause.message().is_none());
assert!(cause.source().is_some());

// an example of Cause who have a standard io error.
use std::io::Error as IoErr;
use std::io::ErrorKind;
let io_err = IoErr::new(ErrorKind::Other, "oh no!");
println!("{}", Cause::new(InternalError).src(io_err).msg("internal error caused by io error"));

// a couple of macro examples
use cause::cause;

let cause = cause!(InternalError);
println!("{}", cause);
  // => "InternalError" on release build
  // => "InternalError: [lib.rs:64]" on debug build

let cause = cause!(NotFoundError, "There is no such contents.");
println!("{}", cause);
  // => "InternalError: There is no such contents." on release build
  // => "InternalError: There is no such contents. [lib.rs:69]" on debug build

```

## Changelog

### 0.1.2

- Fixed `cause` macro issue that `use cause::Cause` was implicitly required,

### 0.1.1

Following getter methods are added to [Cuase].

- `Cause::message()`
- `Cuase::cause()`

### 0.1.0

Initial Version

## License

The MIT License (MIT)

Copyright (c) 2021 msr1k
