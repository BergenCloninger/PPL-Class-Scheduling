use worker::*;
use worker_sys::D1ExecResult;
use crate::SQLError;

pub async fn define_db(database: &D1Database) -> std::result::Result<(), SQLError> {
    let result: Result<D1ExecResult> = database.exec(
        "CREATE TABLE `users` (`id` INTEGER PRIMARY KEY AUTOINCREMENT, `username` VARCHAR(255) UNIQUE, `password` VARCHAR(255), `role` VARCHAR(255));
        CREATE TABLE `faculty` (`id` INTEGER PRIMARY KEY AUTOINCREMENT, `name` VARCHAR(255), `email` VARCHAR(255), `department` VARCHAR(255));
        CREATE TABLE `classes` (`id` INTEGER PRIMARY KEY AUTOINCREMENT, `name` VARCHAR(255), `description` TEXT, `capacity` INTEGER, `code` VARCHAR(255), `class_type` VARCHAR(255), `section` VARCHAR(255), `term` VARCHAR(255), `year` VARCHAR(255));
        CREATE TABLE `schedules` (`id` INTEGER PRIMARY KEY AUTOINCREMENT, `start_hour` INTEGER, `start_minute` INTEGER, `end_hour` INTEGER, `end_minute` INTEGER, `days` VARCHAR(50));
        CREATE TABLE `rooms` (`id` INTEGER PRIMARY KEY AUTOINCREMENT, `room_number` VARCHAR(255), `capacity` INTEGER, `room_type` VARCHAR(255));
        CREATE TABLE `features` (`id` INTEGER PRIMARY KEY AUTOINCREMENT, `name` VARCHAR(255), `description` TEXT);
        CREATE TABLE `room_features` (`id` INTEGER PRIMARY KEY AUTOINCREMENT, `room_id` INTEGER, `feature_id` INTEGER, FOREIGN KEY (`room_id`) REFERENCES `rooms` (`id`), FOREIGN KEY (`feature_id`) REFERENCES `features` (`id`));
        CREATE TABLE `preferences` (`id` INTEGER PRIMARY KEY AUTOINCREMENT, `faculty_id` INTEGER, `preference_type` VARCHAR(255), `value` VARCHAR(255), FOREIGN KEY (`faculty_id`) REFERENCES `faculty` (`id`));
        CREATE TABLE `class_schedule_rooms` (`id` INTEGER PRIMARY KEY AUTOINCREMENT, `class_id` INTEGER, `schedule_id` INTEGER, `room_id` INTEGER, FOREIGN KEY (`class_id`) REFERENCES `classes` (`id`), FOREIGN KEY (`schedule_id`) REFERENCES `schedules` (`id`), FOREIGN KEY (`room_id`) REFERENCES `rooms` (`id`));
        CREATE TABLE `class_faculty` (`id` INTEGER PRIMARY KEY AUTOINCREMENT, `class_id` INTEGER, `faculty_id` INTEGER, FOREIGN KEY (`class_id`) REFERENCES `classes` (`id`), FOREIGN KEY (`faculty_id`) REFERENCES `faculty` (`id`));
        CREATE TABLE `reports` (`id` INTEGER PRIMARY KEY AUTOINCREMENT, `report_type` VARCHAR(255), `description` TEXT);
        INSERT INTO rooms (room_number, capacity, room_type) VALUES ('Empty Room', 0, 'Placeholder');"
    ).await;

    match result {
        Ok(_r) => Ok(()),
        Err(_e) => {
            return Err(SQLError::QueryFailure);
        }
    }
}
