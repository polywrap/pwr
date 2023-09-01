easy_error_string::use_easy_error_string!();

#[test]
fn easy_err() {
    let result = normal_fn().easy_err();
    assert_eq!(result, Ok(()));

    let result = native_string_err_fn().easy_err();
    assert_eq!(result, Err(StringError("Test error".to_string())));

    let result = string_error_fn().easy_err();
    assert_eq!(result, Err(StringError("Test error".to_string())));
}

#[test]
fn casting() {
    let result = native_nested_string_err_fn();
    assert_eq!(result, Err("Test error".to_string()));

    let result = nested_string_err_fn();
    assert_eq!(result, Err(StringError("Test error".to_string())));

    let result = nested_string_err_fn2();
    assert_eq!(result, Err(StringError("Test error".to_string())));

    let result = nested_error_fn();
    assert_eq!(result, Err(StringError("Test error".to_string())));
}

fn normal_fn() -> Result<(), String> {
    Ok(())
}

fn native_string_err_fn() -> Result<(), String> {
    Err("Test error".to_string())
}

fn string_error_fn() -> Result<(), StringError> {
    Err(StringError("Test error".to_string()))
}

fn error_fn() -> Result<(), TestError> {
    Err(TestError::SomeError("Test error".to_string()))
}

fn native_nested_string_err_fn() -> Result<(), String> {
    Ok(native_string_err_fn().easy_err()?)
}

fn nested_string_err_fn() -> Result<(), StringError> {
    Ok(native_string_err_fn().easy_err()?)
}

fn nested_string_err_fn2() -> Result<(), StringError> {
    Ok(native_string_err_fn().easy_err()?)
}

fn nested_error_fn() -> Result<(), StringError> {
    Ok(error_fn()?)
}

#[derive(Debug, PartialEq)]
enum TestError {
    SomeError(String),
}

impl std::fmt::Display for TestError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TestError::SomeError(s) => write!(f, "{}", s),
        }
    }
}

impl std::error::Error for TestError {}