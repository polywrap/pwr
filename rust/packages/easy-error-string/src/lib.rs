#[macro_export]
macro_rules! use_easy_error_string {
    ( $( $x:expr ),* ) => {
        #[derive(Debug, PartialEq)]
        pub struct StringError(String);

        impl StringError {
            pub fn new(message: impl Into<String>) -> Self {
                Self(message.into())
            }
        }

        impl std::fmt::Display for StringError {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", self.to_string())
            }
        }

        pub type EasyResult<T> = std::result::Result<T, StringError>;

        impl From<StringError> for String {
            fn from(s: StringError) -> Self {
                s.to_string()
            }
        }

        impl<T> From<T> for StringError where T: std::error::Error {
            fn from(s: T) -> Self {
                StringError(s.to_string())
            }
        }

        trait MapToErrorString<T> {
            fn map_err_str(self) -> std::result::Result<T, StringError>;
            fn easy_err(self) -> std::result::Result<T, StringError>;
        }

        trait OkOrErrorString<T> {
          fn ok_or_str(self, message: impl Into<String>) -> std::result::Result<T, StringError>;
          fn ok_or_obtuse_str(self) -> std::result::Result<T, StringError>;
          fn easy_err(self) -> std::result::Result<T, StringError>;
        }

        impl<T, E> MapToErrorString<T> for std::result::Result<T, E> where E: std::fmt::Display {
            fn map_err_str(self) -> std::result::Result<T, StringError> {
                match self {
                    Ok(value) => Ok(value),
                    Err(e) => Err(StringError(e.to_string())),
                }
            }

            fn easy_err(self) -> std::result::Result<T, StringError> {
                self.map_err_str()
            }
        }

        impl<T> OkOrErrorString<T> for Option<T> {
            fn ok_or_str(self, message: impl Into<String>) -> std::result::Result<T, StringError> {
                Ok(match self {
                    Some(value) => Ok(value),
                    None => Err(StringError(message.into())),
                }?)
            }

            fn ok_or_obtuse_str(self) -> std::result::Result<T, StringError> {
                Ok(match self {
                    Some(value) => Ok(value),
                    None => Err(StringError("Expected Option to be Some".to_string())),
                }?)
            }

            fn easy_err(self) -> std::result::Result<T, StringError> {
                Ok(self.ok_or_obtuse_str()?)
            }
        }
    };
}
