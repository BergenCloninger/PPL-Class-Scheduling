use worker::*;
use crate::{CreateError, SQLError};

pub async fn record_exists(
    database: &D1Database,
    table_name: &str,
    columns: &[&str],
    values: &[&str],
) -> std::result::Result<bool, CreateError> {
    if columns.len() != values.len() {
        return Err(CreateError::DataIntegrityError);
    }

    if !crate::util::validate_names(table_name) {
        return Err(CreateError::InvalidTableName);
    }

    let conditions: Vec<String> = columns.iter().map(|c| format!("{} = ?", c)).collect();
    let where_clause: String = conditions.join(" AND ");

    let sql: String = format!("SELECT id FROM {} WHERE {} LIMIT 1;", table_name, where_clause);
    let statement: D1PreparedStatement = database.prepare(&sql);

    let js_values: Vec<wasm_bindgen::JsValue> = values.iter().map(|&v| wasm_bindgen::JsValue::from(v)).collect();
    let query: D1PreparedStatement = statement.bind(&js_values).map_err(|_e| CreateError::SqlError(SQLError::BindFailure))?;

    let result: std::result::Result<Option<serde_json::Value>, Error> = query.first::<serde_json::Value>(None).await;

    match result {
        Ok(Some(_)) => Ok(true),
        Ok(None) => Ok(false),
        Err(_) => Err(CreateError::SqlError(SQLError::QueryFailure)),
    }
}

pub async fn schedule_exists(
    database: &D1Database,
    start_hour: i32,
    start_minute: i32,
    end_hour: i32,
    end_minute: i32,
    days: &str,
) -> std::result::Result<bool, CreateError> {
    record_exists(
        database,
        "schedules",
        &["start_hour", "start_minute", "end_hour", "end_minute", "days"],
        &[
            &start_hour.to_string(),
            &start_minute.to_string(),
            &end_hour.to_string(),
            &end_minute.to_string(),
            days,
        ],
    )
    .await
}

pub async fn faculty_exists(database: &D1Database, name: &str) -> std::result::Result<bool, CreateError> {
    record_exists(database, "faculty", &["name"], &[name]).await
}

pub async fn get_record_id(
    database: &D1Database,
    table_name: &str,
    columns: &[&str],
    values: &[&str],
) -> std::result::Result<Option<i64>, CreateError> {
    if columns.len() != values.len() {
        return Err(CreateError::DataIntegrityError);
    }

    if !crate::util::validate_names(table_name) {
        return Err(CreateError::InvalidTableName);
    }

    let conditions: Vec<String> = columns.iter().map(|c| format!("{} = ?", c)).collect();
    let where_clause: String = conditions.join(" AND ");
    let sql = format!("SELECT id FROM {} WHERE {} LIMIT 1;", table_name, where_clause);

    let statement = database.prepare(&sql);
    let js_values: Vec<wasm_bindgen::JsValue> = values.iter().map(|&v| wasm_bindgen::JsValue::from(v)).collect();
    let query = statement.bind(&js_values).map_err(|_e| CreateError::SqlError(SQLError::BindFailure))?;

    match query.first::<serde_json::Value>(None).await {
        Ok(Some(val)) => {
            let id = val.get("id").and_then(|v| v.as_i64());
            Ok(id)
        },
        Ok(None) => Ok(None),
        Err(_) => Err(CreateError::SqlError(SQLError::QueryFailure)),
    }
}