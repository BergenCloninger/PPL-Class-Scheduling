use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub role: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Faculty {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub department: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Class {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub capacity: i32,
    pub code: String,
    pub class_type: String,
    pub section: String,
    pub term: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Schedule {
    pub id: i32,
    pub start_hour: i32,
    pub start_minute: i32,
    pub end_hour: i32,
    pub end_minute: i32,
    pub days: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Room {
    pub id: i32,
    pub room_number: String,
    pub capacity: i32,
    pub room_type: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Feature {
    pub id: i32,
    pub name: String,
    pub description: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct RoomFeature {
    pub id: i32,
    pub room_id: i32,
    pub feature_id: i32,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Preference {
    pub id: i32,
    pub faculty_id: i32,
    pub preference_type: String,
    pub value: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ClassScheduleRoom {
    pub id: i32,
    pub class_id: i32,
    pub schedule_id: i32,
    pub room_id: i32,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ClassFaculty {
    pub id: i32,
    pub class_id: i32,
    pub faculty_id: i32,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Report {
    pub id: i32,
    pub report_type: String,
    pub description: String,
}
