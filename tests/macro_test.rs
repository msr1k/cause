use cause::cause;

#[derive(Debug)]
enum ErrorType {
    SomeError,
    AnotherError,
}

#[test]
fn one_argument_macro_test() {
    let cause = cause!(ErrorType::SomeError);
    if cfg!(debug_assertions) { // $ cargo test
        assert_eq!(
            format!("{}", cause),
            "SomeError: [tests/macro_test.rs:11]".to_string()
        );
    } else {                    // $ cargo test --release
        assert_eq!(
            format!("{}", cause),
            "SomeError".to_string()
        );
    }
}

#[test]
fn two_argument_macro_test() {
    let cause = cause!(ErrorType::AnotherError, "Something went wrong!");
    if cfg!(debug_assertions) { // $ cargo test
        assert_eq!(
            format!("{}", cause),
            "AnotherError: Something went wrong! [tests/macro_test.rs:27]".to_string()
        );
    } else {                    // $ cargo test --release
        assert_eq!(
            format!("{}", cause),
            "AnotherError: Something went wrong!".to_string()
        );
    }
}
