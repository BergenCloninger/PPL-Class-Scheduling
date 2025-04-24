use std::{collections::HashMap, sync::LazyLock};
use serde_json::{Value, Map};
use worker::*;

use super::{resource_callback, Callback};
use db::{update::*, exists::*, create::*};

pub static GENERAL_UPDATES: LazyLock<HashMap<http::Method, Callback>> = LazyLock::new(|| {
    HashMap::from([(
        http::Method::PUT,
        resource_callback!(database, user, _query, body, {
            user.require_perm(&crate::auth::UserPerms::High)?;

            let parsed: Value = serde_json::from_str(&body)
                .map_err(|e| Error::RustError(format!("JSON parse error: {}", e)))?;

            let class_data_array = parsed.get("classInfo")
                .and_then(|v| v.as_array())
                .ok_or_else(|| Error::RustError("Missing or invalid 'classInfo' array".into()))?;

            let time_data_array = parsed.get("timeInfo")
                .and_then(|v| v.as_array());

            let class_data = class_data_array.get(0)
                .and_then(|v| v.as_object())
                .cloned()
                .unwrap_or_default();

            let schedule_data = time_data_array
                .and_then(|arr| arr.get(0))
                .and_then(|v| v.as_object())
                .cloned();

            // --- Class Update ---
            if !class_data.is_empty() {
                let id_str = class_data.get("id")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| Error::RustError("Missing class ID".into()))?;

                let id: i32 = id_str.parse()
                    .map_err(|_| Error::RustError("Invalid class ID".into()))?;

                for (key, value) in &class_data {
                    if key != "id" {
                        update_class(&database, key, &value.to_string().trim_matches('"'), id)
                            .await
                            .map_err(|e| Error::RustError(format!("Class update failed: {:?}", e)))?;
                    }
                }
            }

            // --- Optional Schedule Update ---
            if let Some(schedule) = schedule_data {
                let required = ["start_hour", "start_minute", "end_hour", "end_minute"];
                for key in &required {
                    if !schedule.contains_key(*key) {
                        return Response::error(format!("Missing schedule field: {}", key), 400);
                    }
                }

                let start_hour = schedule["start_hour"].as_i64().unwrap_or_default() as i32;
                let start_minute = schedule["start_minute"].as_i64().unwrap_or_default() as i32;
                let end_hour = schedule["end_hour"].as_i64().unwrap_or_default() as i32;
                let end_minute = schedule["end_minute"].as_i64().unwrap_or_default() as i32;

                // Find schedule by time only (ignoring days)
                let mut schedule_id_opt = get_record_id(
                    &database,
                    "schedules",
                    &["start_hour", "start_minute", "end_hour", "end_minute"],
                    &[
                        &start_hour.to_string(),
                        &start_minute.to_string(),
                        &end_hour.to_string(),
                        &end_minute.to_string(),
                    ],
                )
                .await
                .ok()
                .flatten();

                // If not found, create and then fetch
                if schedule_id_opt.is_none() {
                    // You may still need to pass a default days value if DB requires it
                    create_schedule(&database, start_hour, start_minute, end_hour, end_minute, "MWF")
                        .await
                        .map_err(|e| Error::RustError(format!("Schedule creation failed: {:?}", e)))?;

                    schedule_id_opt = get_record_id(
                        &database,
                        "schedules",
                        &["start_hour", "start_minute", "end_hour", "end_minute"],
                        &[
                            &start_hour.to_string(),
                            &start_minute.to_string(),
                            &end_hour.to_string(),
                            &end_minute.to_string(),
                        ],
                    )
                    .await
                    .ok()
                    .flatten();
                }

                if let Some(schedule_id) = schedule_id_opt {
                    if let Some(class_id_str) = class_data.get("id").and_then(|v| v.as_str()) {
                        let class_id: i32 = class_id_str.parse()
                            .map_err(|_| Error::RustError("Invalid class ID".into()))?;

                        update_class_schedule_room(&database, "schedule_id", &schedule_id.to_string(), class_id)
                            .await
                            .map_err(|e| Error::RustError(format!("Failed to update class_schedule_room: {:?}", e)))?;
                    }
                } else {
                    return Response::error("Failed to retrieve or create schedule ID", 500);
                }
            }

            Response::ok("Update successful")
        })
    )])
});