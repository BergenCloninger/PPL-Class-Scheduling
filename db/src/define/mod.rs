use worker::*;
use worker_sys::D1ExecResult;
use crate::SQLError;

pub async fn define_db(database: &D1Database) -> std::result::Result<(), SQLError> {
    let result: Result<D1ExecResult> = database.exec(
        "CREATE TABLE `users` (`id` INTEGER PRIMARY KEY AUTOINCREMENT, `username` VARCHAR(255) UNIQUE, `password` VARCHAR(255), `role` VARCHAR(255));
        CREATE TABLE `faculty` (`id` INTEGER PRIMARY KEY AUTOINCREMENT, `name` VARCHAR(255), `email` VARCHAR(255), `department` VARCHAR(255));
        CREATE TABLE `classes` (`id` INTEGER PRIMARY KEY AUTOINCREMENT, `name` VARCHAR(255), `description` TEXT, `capacity` INTEGER, `code` VARCHAR(255) UNIQUE, `class_type` VARCHAR(255), `section` VARCHAR(255), `term` VARCHAR(255));
        CREATE TABLE `schedules` (`id` INTEGER PRIMARY KEY AUTOINCREMENT, `start_hour` INTEGER, `start_minute` INTEGER, `end_hour` INTEGER, `end_minute` INTEGER, `days` VARCHAR(50));
        CREATE TABLE `rooms` (`id` INTEGER PRIMARY KEY AUTOINCREMENT, `room_number` VARCHAR(255), `capacity` INTEGER, `room_type` VARCHAR(255));
        CREATE TABLE `features` (`id` INTEGER PRIMARY KEY AUTOINCREMENT, `name` VARCHAR(255), `description` TEXT);
        CREATE TABLE `room_features` (`id` INTEGER PRIMARY KEY AUTOINCREMENT, `room_id` INTEGER, `feature_id` INTEGER, FOREIGN KEY (`room_id`) REFERENCES `rooms` (`id`), FOREIGN KEY (`feature_id`) REFERENCES `features` (`id`));
        CREATE TABLE `preferences` (`id` INTEGER PRIMARY KEY AUTOINCREMENT, `faculty_id` INTEGER, `preference_type` VARCHAR(255), `value` VARCHAR(255), FOREIGN KEY (`faculty_id`) REFERENCES `faculty` (`id`));
        CREATE TABLE `class_schedule_rooms` (`id` INTEGER PRIMARY KEY AUTOINCREMENT, `class_id` INTEGER, `schedule_id` INTEGER, `room_id` INTEGER, FOREIGN KEY (`class_id`) REFERENCES `classes` (`id`), FOREIGN KEY (`schedule_id`) REFERENCES `schedules` (`id`), FOREIGN KEY (`room_id`) REFERENCES `rooms` (`id`));
        CREATE TABLE `class_faculty` (`id` INTEGER PRIMARY KEY AUTOINCREMENT, `class_id` INTEGER, `faculty_id` INTEGER, FOREIGN KEY (`class_id`) REFERENCES `classes` (`id`), FOREIGN KEY (`faculty_id`) REFERENCES `faculty` (`id`));
        CREATE TABLE `reports` (`id` INTEGER PRIMARY KEY AUTOINCREMENT, `report_type` VARCHAR(255), `description` TEXT);

        -- Inserting sample data
        INSERT INTO `users` (`username`, `password`, `role`) VALUES ('admin', 'adminpass', 'admin');
        INSERT INTO `users` (`username`, `password`, `role`) VALUES ('john_doe', 'password123', 'faculty');
        INSERT INTO `users` (`username`, `password`, `role`) VALUES ('jane_doe', 'password456', 'faculty');

        INSERT INTO `faculty` (`name`, `email`, `department`) VALUES ('Dr. John Smith', 'john.smith@university.edu', 'Computer Science');
        INSERT INTO `faculty` (`name`, `email`, `department`) VALUES ('Dr. Jane Doe', 'jane.doe@university.edu', 'Mathematics');

        INSERT INTO `classes` (`name`, `description`, `capacity`, `code`, `class_type`, `section`, `term`) VALUES ('CS 101', 'Introduction to Computer Science', 30, 'CS101-2025', 'Lecture', 'A', 'Spring 2025');
        INSERT INTO `classes` (`name`, `description`, `capacity`, `code`, `class_type`, `section`, `term`) VALUES ('MATH 201', 'Calculus II', 40, 'MATH201-2025', 'Lecture', 'B', 'Spring 2025');

        INSERT INTO `schedules` (`start_hour`, `start_minute`, `end_hour`, `end_minute`, `days`) VALUES (9, 0, 10, 30, 'MWF');
        INSERT INTO `schedules` (`start_hour`, `start_minute`, `end_hour`, `end_minute`, `days`) VALUES (11, 0, 12, 30, 'TTh');

        INSERT INTO `rooms` (`room_number`, `capacity`, `room_type`) VALUES ('Room 101', 50, 'Lecture Hall');
        INSERT INTO `rooms` (`room_number`, `capacity`, `room_type`) VALUES ('Room 102', 40, 'Classroom');

        INSERT INTO `features` (`name`, `description`) VALUES ('Projector', 'Room has a projector for presentations');
        INSERT INTO `features` (`name`, `description`) VALUES ('Whiteboard', 'Room has a whiteboard for writing notes');

        INSERT INTO `room_features` (`room_id`, `feature_id`) VALUES (1, 1);
        INSERT INTO `room_features` (`room_id`, `feature_id`) VALUES (1, 2);
        INSERT INTO `room_features` (`room_id`, `feature_id`) VALUES (2, 1);

        INSERT INTO `preferences` (`faculty_id`, `preference_type`, `value`) VALUES (1, 'Preferred Class Time', 'Morning');
        INSERT INTO `preferences` (`faculty_id`, `preference_type`, `value`) VALUES (2, 'Preferred Class Time', 'Afternoon');

        INSERT INTO `class_schedule_rooms` (`class_id`, `schedule_id`, `room_id`) VALUES (1, 1, 1);
        INSERT INTO `class_schedule_rooms` (`class_id`, `schedule_id`, `room_id`) VALUES (2, 2, 2);

        INSERT INTO `class_faculty` (`class_id`, `faculty_id`) VALUES (1, 1);
        INSERT INTO `class_faculty` (`class_id`, `faculty_id`) VALUES (2, 2);

        INSERT INTO `reports` (`report_type`, `description`) VALUES ('Room Booking', 'Room 101 has been booked for CS 101');
        INSERT INTO `reports` (`report_type`, `description`) VALUES ('Class Assignment', 'Dr. Jane Doe has been assigned to MATH 201');
        "
    ).await;

    match result {
        Ok(_r) => Ok(()),
        Err(_e) => {
            return Err(SQLError::QueryFailure);
        }
    }
}
