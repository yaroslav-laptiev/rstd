CREATE TABLE IF NOT EXISTS projects (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    status TEXT NOT NULL CHECK (
        status IN (
           "active",
           "archived",
        )
    ),
    title TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
);

ALTER TABLE tasks
ADD COLUMN IF NOT EXISTS project_id INTEGER NOT NULL DEFAULT 0;