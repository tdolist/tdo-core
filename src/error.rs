//! Custom Error Types for errors that may occur when handling todos (or lists).

use std::error::Error;
use std::fmt;
use std::convert::From;

/// Custom Result Type for tdo.
///
/// This abbreviation is introduced since many functions throughout the crate return this type of result, which bundles all possible errors of the `tdo_core` crate.
pub type TdoResult<T> = Result<T, ErrorKind>;

/// Enum to collect all types of tdo errors.
///
/// This is simply a wrapper for all custom error classes the `tdo_crate` has.
#[derive(Clone, Copy, Debug)]
pub enum ErrorKind {
    /// A storage-related error occured while interacting with the file system.
    StorageError(StorageError),
    /// An error within the tdo data structures occured.
    TodoError(TodoError),
}

/// The Errors that may occur while interacting with the file system.
#[derive(Clone, Copy, Debug)]
pub enum StorageError {
    /// The accessed file is corrupted. This is most likely because someone edited the JSON file manually.
    FileCorrupted,
    // TODO: Actually use the FileCorrupted error type instead of throwing the Serde-Error. ^^
    //  -- Feliix42 (2017-03-24; 17:39)
    /// The data could not be written to the file.
    SaveFailure,
    /// The requested file could not be found.
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

/// Errors that can arise when working with todo lists.
#[derive(Clone, Copy, Debug)]
pub enum TodoError {
    /// The requested item is not in the list.
    NotInList,
    /// The requested todo list does not exist.
    NoSuchList,
    /// The default list is tried to be removed.
    CanNotRemoveDefault,
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
            TodoError::CanNotRemoveDefault => "The default list can no be removed"
        }
    }
}

impl From<TodoError> for ErrorKind {
    fn from(e: TodoError) -> Self {
        ErrorKind::TodoError(e)
    }
}
