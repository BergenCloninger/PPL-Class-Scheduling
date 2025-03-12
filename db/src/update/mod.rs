use wasm_bindgen::JsValue;
use worker::*;
use crate::{UpdateError, SQLError};

// Takes in a database connection, table name, column_name, new_value, and id (index) of the table entry. 
// Updates the given table, column, and index with the new value (replaces the old value).
// Returns Ok() if the execution was valid, otherwise it errors.
pub async fn update(database: &D1Database, table_name: &str, column_name: &str, new_value: &str, id: i32) -> std::result::Result<(), UpdateError> {
    if !crate::util::validate_names(table_name) {
        return Err(UpdateError::InvalidTableName);
    }
    
    let sql = format!(
        "UPDATE {} SET {} = ? WHERE id = ?;", //column_name is currently not checked, not sure exactly how to do that right now. 
        table_name, 
        column_name
    );

    let statement: D1PreparedStatement = database.prepare(&sql);

    let query: Result<D1PreparedStatement> = statement.bind(&[
        JsValue::from(new_value), 
        JsValue::from(id)
    ]);

    let result: Result<D1Result> = match query {
        Ok(r) => r.run().await,
        Err(_e) => return Err(UpdateError::SqlError(SQLError::BindFailure)),
    };

    match result {
        Ok(_r) => Ok(()),
        Err(_e) => return Err(UpdateError::SqlError(SQLError::QueryFailure)),
    }
}

pub async fn update_user(database: &D1Database, column_name: &str, new_value: &str, id: i32) -> std::result::Result<(), UpdateError> {
    update(database, "users", column_name, new_value, id).await
}

pub async fn update_faculty(database: &D1Database, column_name: &str, new_value: &str, id: i32) -> std::result::Result<(), UpdateError> {
    update(database, "faculty", column_name, new_value, id).await
}

pub async fn update_class(database: &D1Database, column_name: &str, new_value: &str, id: i32) -> std::result::Result<(), UpdateError> {
    update(database, "classes", column_name, new_value, id).await
}

pub async fn update_schedule(database: &D1Database, column_name: &str, new_value: &str, id: i32) -> std::result::Result<(), UpdateError> {
    update(database, "schedules", column_name, new_value, id).await
}

pub async fn update_room(database: &D1Database, column_name: &str, new_value: &str, id: i32) -> std::result::Result<(), UpdateError> {
    update(database, "rooms", column_name, new_value, id).await
}

pub async fn update_feature(database: &D1Database, column_name: &str, new_value: &str, id: i32) -> std::result::Result<(), UpdateError> {
    update(database, "features", column_name, new_value, id).await
}

pub async fn update_preference(database: &D1Database, column_name: &str, new_value: &str, id: i32) -> std::result::Result<(), UpdateError> {
    update(database, "preferences", column_name, new_value, id).await
}

pub async fn update_class_schedule_room(database: &D1Database, column_name: &str, new_value: &str, id: i32) -> std::result::Result<(), UpdateError> {
    update(database, "class_schedule_rooms", column_name, new_value, id).await
}

pub async fn update_class_faculty(database: &D1Database, column_name: &str, new_value: &str, id: i32) -> std::result::Result<(), UpdateError> {
    update(database, "class_faculty", column_name, new_value, id).await
}

pub async fn update_room_feature(database: &D1Database, column_name: &str, new_value: &str, id: i32) -> std::result::Result<(), UpdateError> {
    update(database, "room_features", column_name, new_value, id).await
}

pub async fn update_report(database: &D1Database, column_name: &str, new_value: &str, id: i32) -> std::result::Result<(), UpdateError> {
    update(database, "reports", column_name, new_value, id).await
}