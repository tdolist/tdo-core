#[derive(Debug)]
#[allow(dead_code)]
pub enum StorageError {
    FileCorrupted,
    SaveFailure,
}


#[derive(Debug)]
#[allow(dead_code)]
pub enum TodoError {
    NotInList,
    NoSuchList,
}
