use std::fmt::Display;

pub type Result<T> = std::result::Result<T, String>;

pub trait StrErr<T> {
    fn str_err(self) -> Result<T>;
}

impl<T, E: Display> StrErr<T> for std::result::Result<T, E> {
    fn str_err(self) -> Result<T> {
        match self {
            Ok(t) => Ok(t),
            Err(e) => Err(e.to_string()),
        }
    }
}
