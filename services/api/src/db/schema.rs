// @generated automatically by Diesel CLI.

diesel::table! {
    solutions (id) {
        id -> Uuid,
        packed_items -> Array<Nullable<Int4>>,
        total_value -> Int4,
        task_id -> Uuid,
    }
}

diesel::table! {
    tasks (id) {
        id -> Uuid,
        #[max_length = 20]
        status -> Varchar,
        ts_submitted -> Int4,
        ts_started -> Nullable<Int4>,
        ts_completed -> Nullable<Int4>,
        problem_capacity -> Int4,
        problem_weights -> Array<Nullable<Int4>>,
        problem_values -> Array<Nullable<Int4>>,
    }
}

diesel::joinable!(solutions -> tasks (task_id));

diesel::allow_tables_to_appear_in_same_query!(
    solutions,
    tasks,
);
