use diesel::prelude::*;
use uuid::Uuid;

// todo: use the api as a library to pull in these types

#[derive(Queryable, Selectable, Insertable, Identifiable, Debug)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = crate::db::schema::tasks)]
pub struct Task {
    pub id: Uuid,
    pub status: String,
    pub ts_submitted: i32,
    pub ts_started: Option<i32>,
    pub ts_completed: Option<i32>,
    pub problem_capacity: i32,
    pub problem_weights: Vec<Option<i32>>,
    pub problem_values: Vec<Option<i32>>,
}

#[derive(Queryable, Selectable, Insertable, Identifiable, Associations, Debug, Clone)]
#[diesel(belongs_to(Task))]
#[diesel(table_name = crate::db::schema::solutions)]
pub struct Solution {
    pub id: Uuid,
    pub packed_items: Vec<Option<i32>>,
    pub total_value: i32,
    pub task_id: Uuid,
}

/// PostgreSQL rows of type `integer[] NOT NULL` are allowed to have null entries
/// in the array. I don't think it's possible to specify that all entries are
/// non-null...
pub fn map_arr(arr: &[i32]) -> Vec<Option<i32>> {
    arr.iter().map(|i| Some(*i)).collect()
}

pub fn map_arr_inv(arr: &[Option<i32>]) -> Result<Vec<i32>, &str> {
    if !arr.iter().all(|i| i.is_some()) {
        // This is an application error, as Rocket ensures all inputted vectors
        // for weights and values contain non-null entries
        Err("Unexpected None in map_arr_inv")
    } else {
        Ok(arr.iter().map(|i| i.unwrap()).collect())
    }
}
