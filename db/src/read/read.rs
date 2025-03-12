use wasm_bindgen::JsValue;
use worker::*;
use crate::{ReadError, SQLError};
use serde::de::DeserializeOwned;
use super::{Class, ClassScheduleRoom, Faculty, ClassFaculty, Feature, Preference, Report, Room, RoomFeature, Schedule, User};

/// Takes in a database connection, table name, and id (index) of the table entry. 
/// Calls execute select_query if the name is valid, with the same parameters it was called with. 
/// Returns Ok() if the execution was valid, otherwise it errors.
/// Uses a utility function to check if the table name is valid (compared against a list of table names) to prevent against SQL injection
async fn read_by_id<T: DeserializeOwned>(database: &D1Database, table_name: &str, id: i32) -> std::result::Result<T, ReadError> {
    if !crate::util::validate_names(table_name) {
        return Err(ReadError::InvalidTableName);
    }

    // TODO: Add 'validate_columns' here. 

    let row: Option<T> = execute_select_query(database, table_name, id).await?;

    match row {
        Some(record) => Ok(record),
        None => Err(ReadError::NotFoundError),
    }
}

/// Executes a select statement with a given table name and integer id and returns the entire resulting record
/// Requires a database connection object. 
/// Returns the status of the query (Ok() if successful, or a specific error about what went wrong if not)
async fn execute_select_query<T: DeserializeOwned>(database: &D1Database, table_name: &str, id: i32) -> std::result::Result<Option<T>, ReadError> {
    let sql = format!(
        "SELECT * FROM {} WHERE id = ?;",
        table_name
    );

    let statement = database.prepare(&sql);
    
    let js_values = vec![JsValue::from(id)];

    let query: Result<D1PreparedStatement> = statement.bind(&js_values);

    let result: Result<Option<T>> = match query {
        Ok(r) => r.first::<T>(None).await,
        Err(_e) => return Err(ReadError::SqlError(SQLError::BindFailure)),
    };

    match result {
        Ok(res) => Ok(res),
        Err(_e) => Err(ReadError::SqlError(SQLError::QueryFailure)),
    }
}

/// Executes a select statement that retrieves all records from a given table.
/// Requires a database connection object. 
/// Returns a vector of all the records as type T or an error if something goes wrong.
async fn execute_select_all_query<T: DeserializeOwned>(database: &D1Database, table_name: &str) -> std::result::Result<Vec<T>, ReadError> {
    let sql = format!(
        "SELECT * FROM {};",
        table_name
    );

    let statement = database.prepare(&sql);
    
    let query: Result<D1PreparedStatement> = statement.bind(&[]);

    let result: Result<D1Result> = match query {
        Ok(r) => r.all().await,
        Err(_e) => return Err(ReadError::SqlError(SQLError::BindFailure)),
    };

    match result {
        Ok(res) => res.results::<T>().map_err(|_e| ReadError::SqlError(SQLError::QueryFailure)),
        Err(_e) => Err(ReadError::SqlError(SQLError::QueryFailure)),
    }
}

pub async fn read_from_user_with_username(database: &D1Database, username: &str) -> std::result::Result<User, ReadError> {   
    let sql = "SELECT * FROM users WHERE username = ?;".to_string();

    let statement = database.prepare(&sql);
    
    let js_values = vec![JsValue::from(username)];

    let query: Result<D1PreparedStatement> = statement.bind(&js_values);

    let result: Result<Option<User>> = match query {
        Ok(r) => r.first::<User>(None).await,
        Err(_e) => return Err(ReadError::SqlError(SQLError::BindFailure)),
    };

    let row: Option<User> = match result {
        Ok(res) => Ok(res),
        Err(_e) => Err(ReadError::SqlError(SQLError::QueryFailure)),
    }?;

    match row {
        Some(record) => Ok(record),
        None => Err(ReadError::NotFoundError),
    }
}

pub async fn read_from_user(database: &D1Database, user_id: i32) -> std::result::Result<User, ReadError> {
    read_by_id::<User>(database, "users", user_id).await
}

pub async fn read_from_faculty(database: &D1Database, faculty_id: i32) -> std::result::Result<Faculty, ReadError> {
    read_by_id::<Faculty>(database, "faculty", faculty_id).await
}

pub async fn read_from_class(database: &D1Database, class_id: i32) -> std::result::Result<Class, ReadError> {
    read_by_id::<Class>(database, "classes", class_id).await
}

pub async fn read_from_schedule(database: &D1Database, schedule_id: i32) -> std::result::Result<Schedule, ReadError> {
    read_by_id::<Schedule>(database, "schedules", schedule_id).await
}

pub async fn read_from_room(database: &D1Database, room_id: i32) -> std::result::Result<Room, ReadError> {
    read_by_id::<Room>(database, "rooms", room_id).await
}

pub async fn read_from_feature(database: &D1Database, feature_id: i32) -> std::result::Result<Feature, ReadError> {
    read_by_id::<Feature>(database, "features", feature_id).await
}

pub async fn read_from_preference(database: &D1Database, preference_id: i32) -> std::result::Result<Preference, ReadError> {
    read_by_id::<Preference>(database, "preferences", preference_id).await
}

pub async fn read_from_class_schedule_room(database: &D1Database, class_schedule_room_id: i32) -> std::result::Result<ClassScheduleRoom, ReadError> {
    read_by_id::<ClassScheduleRoom>(database, "class_schedule_rooms", class_schedule_room_id).await
}

pub async fn read_from_class_faculty(database: &D1Database, class_faculty_id: i32) -> std::result::Result<ClassFaculty, ReadError> {
    read_by_id::<ClassFaculty>(database, "class_faculty", class_faculty_id).await
}

pub async fn read_from_report(database: &D1Database, report_id: i32) -> std::result::Result<Report, ReadError> {
    read_by_id::<Report>(database, "reports", report_id).await
}

pub async fn read_from_room_feature(database: &D1Database, room_feature_id: i32) -> std::result::Result<RoomFeature, ReadError> {
    read_by_id::<RoomFeature>(database, "room_features", room_feature_id).await
}

pub async fn read_all_from_classes(database: &D1Database) -> std::result::Result<Vec<Class>, ReadError> {
    execute_select_all_query::<Class>(database, "classes").await
}

pub async fn read_all_from_schedule(database: &D1Database) -> std::result::Result<Vec<Schedule>, ReadError> {
    execute_select_all_query::<Schedule>(database, "schedules").await
}

pub async fn read_all_from_faculty(database: &D1Database) -> std::result::Result<Vec<Faculty>, ReadError> {
    execute_select_all_query::<Faculty>(database, "faculty").await
}

pub async fn read_all_from_preferences(database: &D1Database) -> std::result::Result<Vec<Preference>, ReadError> {
    execute_select_all_query::<Preference>(database, "preferences").await
}

pub async fn read_all_from_rooms(database: &D1Database) -> std::result::Result<Vec<Room>, ReadError> {
    execute_select_all_query::<Room>(database, "rooms").await
}

pub async fn read_all_from_features(database: &D1Database) -> std::result::Result<Vec<Feature>, ReadError> {
    execute_select_all_query::<Feature>(database, "features").await
}