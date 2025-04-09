#[allow(dead_code)]
pub fn validate_names(table_name: &str) -> bool {
    static VALID_TABLES: &[&str] = &[
        "users", "faculty", "classes", "schedules", "rooms", "features", "room_features", "preferences", "class_schedule_rooms", "faculty_rooms", "reports", "class_faculty"
    ];

    VALID_TABLES.contains(&table_name)
}