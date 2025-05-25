-- migrate:up
CREATE TABLE work_entries (
    id SERIAL PRIMARY KEY,
    work_date DATE NOT NULL,
    job_a_hours REAL NOT NULL,
    job_b_hours REAL NOT NULL
);

-- migrate:down
DROP TABLE work_entries;
