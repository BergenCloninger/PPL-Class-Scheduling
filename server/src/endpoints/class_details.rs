use serde::{Deserialize, Serialize};
use worker::*;
use std::{collections::HashMap, sync::LazyLock};
use super::{resource_callback, Callback};

#[derive(Deserialize, Serialize)]
#[derive(Debug)]
pub struct ClassDetails {
    #[serde(rename = "WCU Prefix")]
    pub wcu_prefix: String,
    
    #[serde(rename = "Course Code")]
    pub course_code: i32,
    
    #[serde(rename = "Name")]
    pub name: String,
    
    #[serde(rename = "Year")]
    pub year: i32,
    
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
    HashMap::from([
        (
            http::Method::POST,
            resource_callback!(database, user, _query, body, {
                user.require_perm(&crate::auth::UserPerms::High)?;

                let class_details: ClassDetails = serde_json::from_str(body)?;
                console_log!("Received class details: {:#?}", class_details);

                let class_result = db::create::create_class(
                    &database,
                    &class_details.name,
                    &"Description for class".to_string(), //nplaceholder
                    30, //placeholder
                    &class_details.course_code.to_string(),
                    &class_details.wcu_prefix,
                    &class_details.section,
                    &class_details.term,
                ).await;

                if class_result.is_err() {
                    let error_message = format!("Failed to insert class: {:?}", class_result);
                    console_log!("{}", error_message);  // Log the error message for debugging
                    return Response::error(error_message, 500);
                }

                let schedule_id = match parse_times(&class_details.times) {
                    Some((start_hour, start_minute, end_hour, end_minute)) => {
                        let schedule_result = db::create::create_schedule(
                            database,
                            start_hour,
                            start_minute,
                            end_hour,
                            end_minute,
                            &class_details.days,
                        ).await;

                        if schedule_result.is_err() {
                            return Response::error(format!("Failed to insert schedule: {:?}", schedule_result), 500);
                        }

                        1
                    }
                    None => {
                        return Response::error("Invalid class times", 400);
                    }
                };

                let faculty_result = db::create::create_faculty(
                    database,
                    &class_details.instructor,
                    &"instructor@example.com".to_string(),
                    &"Department Placeholder".to_string(),
                ).await;

                if faculty_result.is_err() {
                    return Response::error(format!("Failed to insert faculty: {:?}", faculty_result), 500);
                }

                let class_faculty_result = db::create::create_class_faculty (
                    database,
                    1, //placeholder
                    1, //placeholder
                ).await;

                if class_faculty_result.is_err() {
                    return Response::error(format!("Failed to link faculty to class: {:?}", class_faculty_result), 500);
                }

                let room_result = db::create::create_class_schedule_room(
                    database,
                    1, //placeholder
                    schedule_id,
                    1, //placeholder
                ).await;

                if room_result.is_err() {
                    return Response::error(format!("Failed to assign room to class: {:?}", room_result), 500);
                }

                // Return success response after all steps
                Response::ok("Class details added successfully")
            })
        )
    ])
});