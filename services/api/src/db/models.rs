use diesel::prelude::*;
use uuid::Uuid;

use crate::{ApiResult, HttpStatus, Knapsack};

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

impl Task {
    pub fn from_dto(task: &Knapsack) -> Self {
        Task {
            id: task.task,
            status: format!("{}", task.status),
            ts_submitted: task.timestamps.submitted,
            ts_started: task.timestamps.started,
            ts_completed: task.timestamps.completed,
            problem_capacity: task.problem.capacity as i32,
            problem_weights: map_arr(&task.problem.weights),
            problem_values: map_arr(&task.problem.values),
        }
    }
}

#[derive(Queryable, Selectable, Insertable, Identifiable, Associations, Debug)]
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
pub fn map_arr(arr: &[u32]) -> Vec<Option<i32>> {
    arr.iter().map(|i| Some(*i as i32)).collect()
}

pub fn map_arr_inv(arr: &[Option<i32>]) -> ApiResult<Vec<u32>> {
    if !arr.iter().all(|i| i.is_some()) {
        // This is an application error, as Rocket ensures all inputted vectors
        // for weights and values contain non-null entries
        Err(HttpStatus::new(500))
    } else {
        Ok(arr.iter().map(|i| i.unwrap() as u32).collect())
    }
}
