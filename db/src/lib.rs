mod util;

pub mod create;
pub mod read;
pub mod update;
pub mod delete;
pub mod define;
pub mod exists;

#[derive(Debug)]
pub enum SQLError {
	BindFailure,
    QueryFailure,
    PrepareFailure
}

#[derive(Debug)]
pub enum CreateError {
    SqlError(SQLError),
	InvalidTableName,
    DataIntegrityError,
}

#[derive(Debug)]
pub enum ReadError {
    SqlError(SQLError),
    InvalidTableName,
    NotFoundError,
}

#[derive(Debug)]
pub enum UpdateError {
    SqlError(SQLError),
    InvalidColumnName,
	InvalidTableName,
    DataIntegrityError,
    DeserializationError,
}

#[derive(Debug)]
pub enum DeleteError {
    SqlError(SQLError),
    InvalidTableName,
    DataIntegrityError,
}