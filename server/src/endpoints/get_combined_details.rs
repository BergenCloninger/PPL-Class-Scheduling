use std::{collections::HashMap, sync::LazyLock};
use serde::Serialize;
use super::{resource_callback, Callback};
use worker::*;

use db::read::{Schedule, Faculty, Preference, Room, Feature};

#[derive(Serialize)]
pub struct CombinedClassDetails {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub capacity: i32,
    pub code: String,
    pub class_type: String,
    pub section: String,
    pub term: String,
    pub schedule: Vec<Schedule>,
    pub faculty: Vec<Faculty>,
    pub room: Vec<Room>,
    pub features: Vec<Feature>,
}

#[derive(Serialize)]
pub struct CombinedDetails {
    pub classes: Vec<Class>,
    pub schedules: Vec<Schedule>,
    pub faculty: Vec<Faculty>,
    pub preferences: Vec<Preference>,
    pub rooms: Vec<Room>,
    pub features: Vec<Feature>,
}

#[derive(Serialize)]
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

pub static GET_COMBINED_DETAILS: LazyLock<HashMap<http::Method, Callback>> = LazyLock::new(|| {
    HashMap::from([
        (
            http::Method::GET,
            resource_callback!(database, user, query, _body, {
                user.require_perm(&crate::auth::UserPerms::General)?;

                let classes_res = db::read::read_all_from_classes(&database).await;
                let class_schedule_rooms_res = db::read::read_all_from_class_schedule_room(&database).await;
                let class_faculty_res = db::read::read_all_from_class_faculty(&database).await;
                let room_features_res = db::read::read_all_from_room_feature(&database).await;
                let rooms_res = db::read::read_all_from_rooms(&database).await;
                let schedules_res = db::read::read_all_from_schedule(&database).await;
                let faculty_res = db::read::read_all_from_faculty(&database).await;
                let features_res = db::read::read_all_from_features(&database).await;

                let classes = match classes_res {
                    Ok(val) => val,
                    Err(e) => return Response::error(format!("{:?}", e), 500),
                };

                let schedules = match schedules_res {
                    Ok(val) => val,
                    Err(e) => return Response::error(format!("{:?}", e), 500),
                };

                let faculty = match faculty_res {
                    Ok(val) => val,
                    Err(e) => return Response::error(format!("{:?}", e), 500),
                };

                let rooms = match rooms_res {
                    Ok(val) => val,
                    Err(e) => return Response::error(format!("{:?}", e), 500),
                };

                let features = match features_res {
                    Ok(val) => val,
                    Err(e) => return Response::error(format!("{:?}", e), 500),
                };

                let class_schedule_rooms = match class_schedule_rooms_res {
                    Ok(val) => val,
                    Err(e) => return Response::error(format!("{:?}", e), 500),
                };

                let class_faculty = match class_faculty_res {
                    Ok(val) => val,
                    Err(e) => return Response::error(format!("{:?}", e), 500),
                };

                let room_features = match room_features_res {
                    Ok(val) => val,
                    Err(e) => return Response::error(format!("{:?}", e), 500),
                };

                let combined_classes: Vec<CombinedClassDetails> = classes.into_iter().map(|class| {
                    let class_schedules: Vec<&Schedule> = class_schedule_rooms.iter()
                        .filter(|csr| csr.class_id == class.id)
                        .filter_map(|csr| schedules.iter().find(|schedule| schedule.id == csr.schedule_id))
                        .collect();

                    let class_faculties: Vec<&Faculty> = class_faculty.iter()
                        .filter(|cf| cf.class_id == class.id)
                        .filter_map(|cf| faculty.iter().find(|fac| fac.id == cf.faculty_id))
                        .collect();

                    let class_rooms: Vec<&Room> = class_schedule_rooms.iter()
                        .filter(|csr| csr.class_id == class.id)
                        .filter_map(|csr| rooms.iter().find(|room| room.id == csr.room_id))
                        .collect();

                    let room_id = class_rooms.get(0).map(|room| room.id);
                    let room_features_list: Vec<Feature> = room_features.iter()
                        .filter(|rf| rf.room_id == room_id.unwrap_or_default())
                        .filter_map(|rf| features.iter().find(|f| f.id == rf.feature_id))
                        .cloned()
                        .collect();

                    CombinedClassDetails {
                        id: class.id,
                        name: class.name,
                        description: class.description,
                        capacity: class.capacity,
                        code: class.code,
                        class_type: class.class_type,
                        section: class.section,
                        term: class.term,
                        schedule: class_schedules.into_iter().cloned().collect(),  // This is now Vec<Schedule>
                        faculty: class_faculties.into_iter().cloned().collect(),
                        room: class_rooms.into_iter().cloned().collect(),
                        features: room_features_list,
                    }
                }).collect();

                Response::ok(serde_json::to_string(&combined_classes)?)
            })
        ),
    ])
});