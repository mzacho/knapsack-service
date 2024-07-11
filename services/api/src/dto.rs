use std::fmt::Display;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::models::{Task as DbTask, Solution as DbSolution};
use crate::{ApiResult, HttpStatus};
use crate::db::models::map_arr_inv;

#[derive(Serialize, Deserialize, Debug)]
pub struct Problem {
    pub capacity: u32,
    pub weights: Vec<u32>,
    pub values: Vec<u32>,
}

/// This type is only used to properly serialize the problem object
/// when it appears inside the Knapsack struct.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ProblemBody {
    Problem(Problem),
}

impl ProblemBody {
    pub fn owned_to_problem(self) -> Problem {
        match self {
            ProblemBody::Problem(p) => p,
        }
    }

    fn to_problem(&self) -> &Problem {
        match self {
            ProblemBody::Problem(p) => p,
        }
    }

    /// Validate that none of the inputs are too large, so that primitive
    /// type casting to i32, which is what the ORM expects, is safe.
    pub fn validate(&self) -> ApiResult<()> {
        use crate::HttpStatus;
        let problem = self.to_problem();
        let assert_all_representable_by_i32 = |vec: &[u32]| -> ApiResult<()> {
            if vec.iter().any(|v| i32::try_from(*v).is_err()) {
                Err(HttpStatus::new(400))
            } else {
                Ok(())
            }
        };
        assert_all_representable_by_i32(&problem.weights)
            .and_then(|_| assert_all_representable_by_i32(&problem.values))
    }
}

#[derive(Serialize)]
pub struct Knapsack {
    pub task: Uuid,
    pub status: Status,
    pub timestamps: Timestamps,
    pub problem: Problem,
    pub solution: MyOption<Solution>,
}


impl Knapsack {
    pub fn new(problem: Problem) -> Self {
        Self {
            task: Uuid::new_v4(),
            status: Status::Submitted,
            timestamps: Timestamps {
                submitted: crate::current_time(),
                started: Option::None,
                completed: Option::None,
            },
            problem,
            solution: MyOption::<Solution>::None(Empty {}),
        }
    }

    pub fn from_task(task: &DbTask) -> ApiResult<Self> {
        Ok(Self {
            task: task.id,
            status: Status::from_str(task.status.as_str())?,
            timestamps: Timestamps {
                submitted: task.ts_submitted,
                started: task.ts_started,
                completed: task.ts_completed,
            },
            problem: Problem {
                capacity: task.problem_capacity as u32,
                weights: map_arr_inv(&task.problem_weights)?,
                values: map_arr_inv(&task.problem_values)?,
            },
            solution: MyOption::<Solution>::None(Empty {}),
        })
    }

    pub fn set(self, solution: DbSolution) -> ApiResult<Self> {
        Ok(Self {
            solution: MyOption::Some(Solution {
                packed_items: map_arr_inv(&solution.packed_items)?,
                total_value: solution.total_value as u32,
            }),
            ..self
        })
    }
}

// Redefining the Option type is a bit terrible, but it's an easy hack
// to serialize Option::None to `{}` (as in the given sample example
// inputs) instead of `null`.
#[derive(Serialize)]
#[serde(untagged)]
pub enum MyOption<T> {
    None(Empty),
    Some(T),
}

#[derive(Serialize)]
pub struct Empty {}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Submitted,
    Started,
    Completed,
}
impl Status {
    fn from_str(status: &str) -> ApiResult<Self> {
        use Status::*;
        match status {
            "submitted" => Ok(Submitted),
            "started" => Ok(Started),
            "completed" => Ok(Completed),
            _ => Err(HttpStatus::new(500))
        }
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Submitted => write!(f,"submitted"),
            Status::Started => write!(f,"started"),
            Status::Completed => write!(f,"completed"),
        }
    }
}

#[derive(Serialize)]
pub struct Timestamps {
    pub submitted: i32,
    pub started: Option<i32>,
    pub completed: Option<i32>,
}

#[derive(Serialize)]
pub struct Solution {
    // array of integers (indices to weights and values)
    pub packed_items: Vec<u32>,
    // sum of value of packed_items
    pub total_value: u32,
}
