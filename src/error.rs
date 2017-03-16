//! Errors for Tdo
use std::error::Error;
use std::fmt;
use std::convert::From;

/// Custom Result for tdo.
pub type TdoResult<T> = Result<T, ErrorKind>;

/// Enum to collect all tdo errors.
#[derive(Clone, Copy, Debug)]
#[allow(missing_docs)]
pub enum ErrorKind {
    StorageError(StorageError),
    TodoError(TodoError),
}

/// Error for handling any error while working with the file system.
#[derive(Clone, Copy, Debug)]
#[allow(missing_docs)]
pub enum StorageError {
    FileCorrupted,
    SaveFailure,
    FileNotFound,
}

impl fmt::Display for StorageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl Error for StorageError {
    fn description(&self) -> &str {
        match *self {
            StorageError::FileCorrupted => "File is corrupted",
            StorageError::SaveFailure => "File could not be saved",
            StorageError::FileNotFound => "File was not found",
        }
    }
}

impl From<StorageError> for ErrorKind {
    fn from(e: StorageError) -> Self {
        ErrorKind::StorageError(e)
    }
}

/// Error for handling list errors for todos.
#[derive(Clone, Copy, Debug)]
#[allow(missing_docs)]
pub enum TodoError {
    NotInList,
    NoSuchList,
}

impl fmt::Display for TodoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl Error for TodoError {
    fn description(&self) -> &str {
        match *self {
            TodoError::NotInList => "Todo is not in this list",
            TodoError::NoSuchList => "No such list",
        }
    }
}

impl From<TodoError> for ErrorKind {
    fn from(e: TodoError) -> Self {
        ErrorKind::TodoError(e)
    }
}
