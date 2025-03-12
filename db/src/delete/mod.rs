use wasm_bindgen::JsValue;
use worker::*;
use crate::{DeleteError, SQLError};

// Takes in a database connection, table name, and id (index) of the table entry. 
// Sets result equal to the result of execute_delete_query. 
// Returns Ok() if the execution was valid, otherwise it errors.
// Uses a utility function to check if the table name is valid (compared against a list of table names) to prevent against sql injection
async fn execute_delete_by_id_query(database: &D1Database, table_name: &str, id: &str) -> std::result::Result<(), DeleteError> {
    if !crate::util::validate_names(table_name) {
        return Err(DeleteError::InvalidTableName);
    }
    
    let sql = format!("DELETE FROM {} WHERE id = ?;", table_name);

    let statement = database.prepare(&sql);

    let js_values: Vec<JsValue> = vec![JsValue::from(id)];

    let query: Result<D1PreparedStatement> = statement.bind(&js_values);

    let result: Result<D1Result> = match query {
        Ok(r) => r.run().await,
        Err(_e) => return Err(DeleteError::SqlError(SQLError::BindFailure)),
    };

    match result {
        Ok(res) => {
            if res.success() {
                Ok(())
            } else {
                Err(DeleteError::SqlError(SQLError::QueryFailure))
            }
        }
        Err(_e) => {
            Err(DeleteError::SqlError(SQLError::QueryFailure))
        }
    }
}

pub async fn delete_user(database: &D1Database, id: &str) -> std::result::Result<(), DeleteError> {
    execute_delete_by_id_query(database, "users", id).await
}

pub async fn delete_faculty(database: &D1Database, id: &str) -> std::result::Result<(), DeleteError> {
    execute_delete_by_id_query(database, "faculty", id).await
}

pub async fn delete_class(database: &D1Database, id: &str) -> std::result::Result<(), DeleteError> {
    execute_delete_by_id_query(database, "classes", id).await
}

pub async fn delete_schedule(database: &D1Database, id: &str) -> std::result::Result<(), DeleteError> {
    execute_delete_by_id_query(database, "schedules", id).await
}

pub async fn delete_room(database: &D1Database, id: &str) -> std::result::Result<(), DeleteError> {
    execute_delete_by_id_query(database, "rooms", id).await
}

pub async fn delete_feature(database: &D1Database, id: &str) -> std::result::Result<(), DeleteError> {
    execute_delete_by_id_query(database, "features", id).await
}

pub async fn delete_preference(database: &D1Database, id: &str) -> std::result::Result<(), DeleteError> {
    execute_delete_by_id_query(database, "preferences", id).await
}

pub async fn delete_class_schedule_room(database: &D1Database, id: &str) -> std::result::Result<(), DeleteError> {
    execute_delete_by_id_query(database, "class_schedule_rooms", id).await
}

pub async fn delete_class_faculty(database: &D1Database, id: &str) -> std::result::Result<(), DeleteError> {
    execute_delete_by_id_query(database, "class_faculty", id).await
}

pub async fn delete_room_feature(database: &D1Database, id: &str) -> std::result::Result<(), DeleteError> {
    execute_delete_by_id_query(database, "room_features", id).await
}

pub async fn delete_report(database: &D1Database, id: &str) -> std::result::Result<(), DeleteError> {
    execute_delete_by_id_query(database, "reports", id).await
}