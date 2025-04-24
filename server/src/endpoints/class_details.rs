use serde::{Deserialize, Serialize};
use worker::*;
use std::{collections::HashMap, sync::LazyLock};
use std::convert::TryInto;

use super::{resource_callback, Callback};

#[derive(Deserialize, Serialize, Debug)]
pub struct ClassDetails {
    #[serde(rename = "WCU Prefix")]
    pub wcu_prefix: String,

    #[serde(rename = "Course Code")]
    pub course_code: String,

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Year")]
    pub year: String,

    #[serde(rename = "Section")]
    pub section: String,

    #[serde(rename = "Term")]
    pub term: String,

    #[serde(rename = "Days")]
    pub days: String,

    #[serde(rename = "Times")]
    pub times: String,

    #[serde(rename = "Credit Hrs")]
    pub credit_hrs: i32,

    #[serde(rename = "Capacity")]
    pub capacity: i32,

    #[serde(rename = "Instructor")]
    pub instructor: String,

    #[serde(rename = "Scheduler")]
    pub scheduler: String,
}

fn parse_times(times: &str) -> Option<(i32, i32, i32, i32)> {
    let time_parts: Vec<&str> = times.split('-').collect();
    if time_parts.len() == 2 {
        let start = time_parts[0].trim();
        let end = time_parts[1].trim();
        let start_parts: Vec<&str> = start.split(':').collect();
        let end_parts: Vec<&str> = end.split(':').collect();

        if start_parts.len() == 2 && end_parts.len() == 2 {
            let start_hour = start_parts[0].parse::<i32>().ok()?;
            let start_minute = start_parts[1].parse::<i32>().ok()?;
            let end_hour = end_parts[0].parse::<i32>().ok()?;
            let end_minute = end_parts[1].parse::<i32>().ok()?;
            return Some((start_hour, start_minute, end_hour, end_minute));
        }
    }
    None
}

pub static CLASS_DETAILS: LazyLock<HashMap<http::Method, Callback>> = LazyLock::new(|| {
    HashMap::from([(
        http::Method::POST,
        resource_callback!(database, user, _query, body, {
            user.require_perm(&crate::auth::UserPerms::High)?;

            let class_details: ClassDetails = match serde_json::from_str(body) {
                Ok(details) => details,
                Err(e) => {
                    let msg = format!("[PARSE_ERROR] Failed to parse body: {:?}", e);
                    console_log!("{}", msg);
                    return Response::error(msg, 400);
                }
            };

            console_log!("[START] Received ClassDetails: {:?}", class_details);

            let class_result = db::create::create_class(
                &database,
                &class_details.name,
                &"Description for class".to_string(),
                class_details.capacity,
                &class_details.course_code,
                &class_details.wcu_prefix,
                &class_details.section,
                &class_details.term,
                &class_details.year,
            ).await;

            if let Err(e) = class_result {
                let msg = format!("[CLASS_CREATE_FAIL] Failed to insert class '{}': {:?}", class_details.name, e);
                console_log!("{}", msg);
                return Response::error(msg, 500);
            }

            let class_id: i32 = match db::exists::get_record_id(
                &database,
                "classes",
                &["name", "section", "term"],
                &[&class_details.name, &class_details.section, &class_details.term],
            ).await {
                Ok(Some(id)) => match id.try_into() {
                    Ok(val) => val,
                    Err(_) => return Response::error("[CLASS_ID_ERROR] Class ID too large", 500),
                },
                _ => {
                    let msg = format!("[CLASS_ID_ERROR] Could not retrieve class ID for '{}', section '{}', term '{}'",
                        class_details.name, class_details.section, class_details.term);
                    console_log!("{}", msg);
                    return Response::error(msg, 500);
                },
            };

            let schedule_id: i32 = match parse_times(&class_details.times) {
                Some((start_hour, start_minute, end_hour, end_minute)) => {
                    console_log!("[SCHEDULE] Parsed time: {}:{} to {}:{}", start_hour, start_minute, end_hour, end_minute);

                    let schedule_exists = match db::exists::schedule_exists(
                        &database,
                        start_hour,
                        start_minute,
                        end_hour,
                        end_minute,
                        &class_details.days,
                    ).await {
                        Ok(exists) => exists,
                        Err(e) => {
                            let msg = format!("[SCHEDULE_CHECK_ERROR] DB error: {:?}", e);
                            console_log!("{}", msg);
                            return Response::error(msg, 500);
                        }
                    };

                    if schedule_exists {
                        return Response::error("[SCHEDULE_EXISTS] Schedule already exists", 400);
                    }

                    let schedule_result = db::create::create_schedule(
                        database,
                        start_hour,
                        start_minute,
                        end_hour,
                        end_minute,
                        &class_details.days,
                    ).await;

                    if let Err(e) = schedule_result {
                        let msg = format!("[SCHEDULE_CREATE_FAIL] Failed to insert schedule: {:?}", e);
                        console_log!("{}", msg);
                        return Response::error(msg, 500);
                    }

                    match db::exists::get_record_id(
                        database,
                        "schedules",
                        &["start_hour", "start_minute", "end_hour", "end_minute", "days"],
                        &[
                            &start_hour.to_string(),
                            &start_minute.to_string(),
                            &end_hour.to_string(),
                            &end_minute.to_string(),
                            &class_details.days,
                        ],
                    ).await {
                        Ok(Some(id)) => match id.try_into() {
                            Ok(val) => val,
                            Err(_) => return Response::error("[SCHEDULE_ID_ERROR] Schedule ID too large", 500),
                        },
                        _ => {
                            let msg = "[SCHEDULE_ID_ERROR] Could not retrieve schedule ID".to_string();
                            console_log!("{}", msg);
                            return Response::error(msg, 500);
                        }
                    }
                }
                None => {
                    let msg = format!("[TIME_PARSE_ERROR] Invalid class times: '{}'", class_details.times);
                    console_log!("{}", msg);
                    return Response::error(msg, 400);
                }
            };

            let faculty_exists = match db::exists::faculty_exists(&database, &class_details.instructor).await {
                Ok(exists) => exists,
                Err(e) => {
                    let msg = format!("[FACULTY_CHECK_ERROR] Error checking faculty '{}': {:?}", class_details.instructor, e);
                    console_log!("{}", msg);
                    return Response::error(msg, 500);
                }
            };

            if faculty_exists {
                return Response::error(format!("[FACULTY_EXISTS] Faculty '{}' already exists", class_details.instructor), 400);
            }

            let faculty_result = db::create::create_faculty(
                database,
                &class_details.instructor,
                &"instructor@example.com".to_string(),
                &"Department Placeholder".to_string(),
            ).await;

            if let Err(e) = faculty_result {
                let msg = format!("[FACULTY_CREATE_FAIL] Failed to insert faculty '{}': {:?}", class_details.instructor, e);
                console_log!("{}", msg);
                return Response::error(msg, 500);
            }

            let faculty_id: i32 = match db::exists::get_record_id(
                database,
                "faculty",
                &["name"],
                &[&class_details.instructor],
            ).await {
                Ok(Some(id)) => match id.try_into() {
                    Ok(val) => val,
                    Err(_) => return Response::error("[FACULTY_ID_ERROR] Faculty ID too large", 500),
                },
                _ => {
                    let msg = format!("[FACULTY_ID_ERROR] Could not retrieve ID for '{}'", class_details.instructor);
                    console_log!("{}", msg);
                    return Response::error(msg, 500);
                }
            };

            let class_faculty_result = db::create::create_class_faculty(
                database,
                class_id,
                faculty_id,
            ).await;

            if let Err(e) = class_faculty_result {
                let msg = format!("[CLASS_FACULTY_LINK_FAIL] Failed to link faculty to class: {:?}", e);
                console_log!("{}", msg);
                return Response::error(msg, 500);
            }

            let room_result = db::create::create_class_schedule_room(
                database,
                class_id,
                schedule_id,
                1, // Placeholder room ID
            ).await;

            if let Err(e) = room_result {
                let msg = format!("[ROOM_ASSIGN_FAIL] Failed to assign room: {:?}", e);
                console_log!("{}", msg);
                return Response::error(msg, 500);
            }

            console_log!("[SUCCESS] Class created successfully with class_id: {}, schedule_id: {}, faculty_id: {}", class_id, schedule_id, faculty_id);
            Response::ok(format!("Class added. class_id: {}, schedule_id: {}, faculty_id: {}", class_id, schedule_id, faculty_id))
        })
    )])
});