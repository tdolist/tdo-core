#[derive(Debug)]
#[allow(dead_code)]
pub enum StorageError {
    FileCorrupted,
    SaveFailure,
    FileNotFound,
}


#[derive(Debug)]
#[allow(dead_code)]
pub enum TodoError {
    NotInList,
    NoSuchList,
}
