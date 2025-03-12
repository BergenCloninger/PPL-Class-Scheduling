use wasm_bindgen::JsValue;
use worker::*;
use crate::{CreateError, SQLError};

// Takes in a database connection, table name, variable number of columns, and the values to add to said columns.
// Contains a helper function to insert a given query, and functions to query every database with the column names assigned. That will prevent injection, but requires hard coding every table.
// Returns Ok() if the execution was valid, otherwise it errors.
async fn execute_insert_query(database: &D1Database, table_name: &str, columns: &[&str], values: &[&str]) -> std::result::Result<(), CreateError> {
    if columns.len() != values.len() {
        return Err(CreateError::DataIntegrityError);
    }

    if !crate::util::validate_names(table_name) {
        return Err(CreateError::InvalidTableName);
    }

    let columns_str = crate::util::to_str_list(columns);
    let placeholders = vec!["?".to_string(); values.len()].join(", ");
    
    let sql = format!("INSERT INTO {} ({}) VALUES ({});", table_name, columns_str, placeholders);

    let statement = database.prepare(&sql);

    let js_values: Vec<JsValue> = values.iter().map(|&s| JsValue::from(s)).collect();

    let query: Result<D1PreparedStatement> = statement.bind(&js_values);

    let result: Result<D1Result> = match query {
        Ok(r) => r.run().await,
        Err(_e) => return Err(CreateError::SqlError(SQLError::BindFailure)),
    };

    match result {
        Ok(res) => {
            if res.success() {
                Ok(())
            } else {
                Err(CreateError::SqlError(SQLError::BindFailure))
            }
        }
        Err(_e) => {
            Err(CreateError::SqlError(SQLError::QueryFailure))
        }
    }
}

//TODO: could and should add more error checking and base case checking to these
// It's likely that some of these will end up private, but idk exactly which ones yet. 
pub async fn create_user(database: &D1Database, username: &str, password: &str, role: &str) -> std::result::Result<(), CreateError> { 
    execute_insert_query(database, "users", &["username", "password", "role"], &[username, password, role]).await
}

pub async fn create_faculty(database: &D1Database, name: &str, email: &str, department: &str) -> std::result::Result<(), CreateError> {
    execute_insert_query(database, "faculty", &["name", "email", "department"], &[name, email, department]).await
}

pub async fn create_class(database: &D1Database, name: &str, description: &str, capacity: i32, code: &str, type_: &str, section: &str, term: &str) -> std::result::Result<(), CreateError> {
    execute_insert_query(database, "classes", &["name", "description", "capacity", "code", "class_type", "section", "term"], &[name, description, &capacity.to_string(), code, type_, section, term]).await
}

pub async fn create_schedule(database: &D1Database, start_hour: i32, start_minute: i32, end_hour: i32, end_minute: i32, days: &str) -> std::result::Result<(), CreateError> {
    execute_insert_query(database, "schedules", &["start_hour", "start_minute", "end_hour", "end_minute", "days"],
        &[&start_hour.to_string(), &start_minute.to_string(), &end_hour.to_string(), &end_minute.to_string(), days]).await
}

pub async fn create_room(database: &D1Database, room_number: &str, capacity: i32, room_type: &str) -> std::result::Result<(), CreateError> {
    execute_insert_query(database, "rooms", &["room_number", "capacity", "room_type"], &[room_number, &capacity.to_string(), room_type]).await
}

pub async fn create_feature(database: &D1Database, name: &str, description: &str) -> std::result::Result<(), CreateError> {
    execute_insert_query(database, "features", &["name", "description"], &[name, description]).await
}

pub async fn create_room_feature(database: &D1Database, room_id: i32, feature_id: i32) -> std::result::Result<(), CreateError> {
    execute_insert_query(database, "room_features", &["room_id", "feature_id"], &[&room_id.to_string(), &feature_id.to_string()]).await
}

pub async fn create_preference(database: &D1Database, faculty_id: i32, preference_type: &str, value: &str) -> std::result::Result<(), CreateError> {
    execute_insert_query(database, "preferences", &["faculty_id", "preference_type", "value"], &[&faculty_id.to_string(), preference_type, value]).await
}

pub async fn create_class_schedule_room(database: &D1Database, class_id: i32, schedule_id: i32, room_id: i32) -> std::result::Result<(), CreateError> {
    execute_insert_query(database, "class_schedule_rooms", &["class_id", "schedule_id", "room_id"], 
        &[&class_id.to_string(), &schedule_id.to_string(), &room_id.to_string()]).await
}

pub async fn create_class_faculty(database: &D1Database, class_id: i32, faculty_id: i32) -> std::result::Result<(), CreateError> {
    execute_insert_query(database, "class_faculty", &["class_id", "faculty_id"], 
        &[&class_id.to_string(), &faculty_id.to_string()]).await
}

pub async fn create_report(database: &D1Database, report_type: &str, description: &str) -> std::result::Result<(), CreateError> {
    execute_insert_query(database, "reports", &["report_type", "description"], &[report_type, description]).await
}