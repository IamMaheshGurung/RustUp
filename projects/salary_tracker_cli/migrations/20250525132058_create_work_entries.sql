CREATE TABLE work_entries (
    id SERIAL PRIMARY KEY,
    work_date DATE NOT NULL,
    job_a_hours FLOAT NOT NULL,
    job_b_hours FLOAT NOT NULL
);
