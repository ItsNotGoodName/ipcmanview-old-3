CREATE TABLE IF NOT EXISTS camera_files (
    camera_id INTEGER NOT NULL,
    file_path TEXT NOT NULL,
    start_time DATETIME NOT NULL,
    end_time DATETIME NOT NULL,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    UNIQUE (camera_id, file_path),
    FOREIGN KEY (camera_id) REFERENCES cameras (id) ON DELETE CASCADE
);
