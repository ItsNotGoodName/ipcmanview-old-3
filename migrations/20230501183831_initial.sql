CREATE TABLE IF NOT EXISTS cameras (
    id INTEGER PRIMARY KEY,
    ip TEXT NOT NULL UNIQUE,
    username TEXT NOT NULL,
    password TEXT NOT NULL,
    scan_cursor DATETIME NOT NULL,
    refreshed_at DATETIME NOT NULL DEFAULT (DATETIME(0, 'unixepoch')),
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS camera_details (
    id INTEGER PRIMARY KEY,
    sn TEXT DEFAULT '' NOT NULL,
    device_class TEXT DEFAULT '' NOT NULL,
    device_type TEXT DEFAULT '' NOT NULL,
    hardware_version TEXT DEFAULT '' NOT NULL,
    market_area TEXT DEFAULT '' NOT NULL,
    process_info TEXT DEFAULT '' NOT NULL,
    vendor TEXT DEFAULT '' NOT NULL,
    FOREIGN KEY (id) REFERENCES cameras (id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS camera_softwares (
    id INTEGER PRIMARY KEY,
    build TEXT DEFAULT '' NOT NULL,
    build_date TEXT DEFAULT '' NOT NULL,
    security_base_line_version TEXT DEFAULT '' NOT NULL,
    version TEXT DEFAULT '' NOT NULL,
    web_version TEXT DEFAULT '' NOT NULL,
    FOREIGN KEY (id) REFERENCES cameras (id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS camera_licenses (
    camera_id INTEGER NOT NULL,
    abroad_info TEXT DEFAULT '' NOT NULL,
    all_type BOOLEAN DEFAULT false NOT NULL,
    digit_channel NUMBER DEFAULT 0 NOT NULL,
    effective_days NUMBER DEFAULT 0 NOT NULL,
    effective_time NUMBER DEFAULT 0 NOT NULL,
    license_id NUMBER DEFAULT 0 NOT NULL,
    product_type TEXT DEFAULT '' NOT NULL,
    status NUMBER DEFAULT 0 NOT NULL,
    username TEXT DEFAULT '' NOT NULL,
    FOREIGN KEY (camera_id) REFERENCES cameras (id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS camera_files (
    id INTEGER PRIMARY KEY,
    camera_id INTEGER NOT NULL,
    file_path TEXT NOT NULL,
    kind TEXT NOT NULL,
    size INTEGER NOT NULL,
    start_time DATETIME NOT NULL,
    end_time DATETIME NOT NULL,
    updated_at DATETIME NOT NULL,
    events JSON NOT NULL,
    UNIQUE (camera_id, file_path),
    UNIQUE (camera_id, start_time),
    FOREIGN KEY (camera_id) REFERENCES cameras (id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS ipc_events (
    name STIRNG NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS pending_scans (
    id INTEGER PRIMARY KEY,
    camera_id INTEGER NOT NULL,
    kind STRING NOT NULL, -- full, cursor, manual
    range_start DATETIME NOT NULL,
    range_end DATETIME NOT NULL,
    UNIQUE (camera_id, kind),
    FOREIGN KEY (camera_id) REFERENCES cameras (id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS active_scans (
    camera_id INTEGER PRIMARY KEY,
    kind STRING NOT NULL, -- full, cursor, manual
    range_start DATETIME NOT NULL,
    range_end DATETIME NOT NULL,
    started_at DATETIME NOT NULL,

    -- Mutable
    range_cursor DATETIME NOT NULL,
    deleted INTEGER NOT NULL DEFAULT 0,
    upserted INTEGER NOT NULL DEFAULT 0,
    percent REAL NOT NULL DEFAULT 0.0,

    FOREIGN KEY (camera_id) REFERENCES cameras (id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS completed_scans (
    id INTEGER PRIMARY KEY,
    --
    camera_id INTEGER NOT NULL,
    kind STRING NOT NULL, -- full, cursor, manual
    range_start DATETIME NOT NULL,
    range_end DATETIME NOT NULL,
    started_at DATETIME NOT NULL,
    --
    range_cursor DATETIME NOT NULL,
    deleted INTEGER NOT NULL,
    upserted INTEGER NOT NULL,
    percent REAL NOT NULL,
    --
    duration INTEGER NOT NULL,
    success BOOLEAN NOT NULL,
    error STRING NOT NULL,

    -- Mutable
    retry_queued BOOLEAN NOT NULL DEFAULT false,
    can_retry BOOLEAN NOT NULL,

    FOREIGN KEY (camera_id) REFERENCES cameras (id) ON DELETE CASCADE
);
