CREATE SCHEMA knapsack;
SET search_path TO public,knapsack;

CREATE TABLE tasks (
    id uuid PRIMARY KEY,
    status varchar(20) NOT NULL,
    ts_submitted integer NOT NULL,
    ts_started integer,
    ts_completed integer,
    problem_capacity integer NOT NULL,
    problem_weights integer[] NOT NULL,
    problem_values integer[] NOT NULL
);

CREATE TABLE solutions (
    id uuid PRIMARY KEY,
    packed_items integer[] NOT NULL,
    total_value integer NOT NULL,
    task_id uuid NOT NULL REFERENCES tasks (id)
);
