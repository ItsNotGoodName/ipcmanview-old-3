CREATE TABLE IF NOT EXISTS cameras (
    id INTEGER PRIMARY KEY,
    ip TEXT NOT NULL UNIQUE,
    username TEXT NOT NULL,
    password TEXT NOT NULL,
    scan_cursor DATETIME NOT NULL
);

CREATE TABLE IF NOT EXISTS camera_details (
    id INTEGER PRIMARY KEY,
    sn TEXT,
    device_class TEXT,
    device_type TEXT,
    hardware_version TEXT,
    market_area TEXT,
    process_info TEXT,
    vendor TEXT,
    FOREIGN KEY (id) REFERENCES cameras (id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS camera_software_versions (
    id INTEGER PRIMARY KEY,
    build TEXT,
    build_date TEXT,
    security_base_line_version TEXT,
    version TEXT,
    web_version TEXT,
    FOREIGN KEY (id) REFERENCES cameras (id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS camera_files (
    camera_id INTEGER NOT NULL,
    file_path TEXT NOT NULL,
    start_time DATETIME NOT NULL,
    end_time DATETIME NOT NULL,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    UNIQUE (camera_id, file_path),
    UNIQUE (camera_id, start_time),
    FOREIGN KEY (camera_id) REFERENCES cameras (id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS active_scans (
    camera_id INTEGER PRIMARY KEY,
    kind STRING NOT NULL, -- Full, Cursor, Manual
    range_start DATETIME NOT NULL,
    range_end DATETIME NOT NULL,
    started_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    FOREIGN KEY (camera_id) REFERENCES cameras (id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS completed_scans (
    id INTEGER PRIMARY KEY,
    camera_id INTEGER NOT NULL,
    kind STRING NOT NULL, -- Full, Cursor, Manual
    range_start DATETIME NOT NULL,
    range_end DATETIME NOT NULL,
    started_at DATETIME NOT NULL,
    duration INTEGER NOT NULL,
    FOREIGN KEY (camera_id) REFERENCES cameras (id) ON DELETE CASCADE
);
