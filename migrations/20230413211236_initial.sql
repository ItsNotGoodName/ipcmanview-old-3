CREATE TABLE IF NOT EXISTS cameras (
    id INTEGER PRIMARY KEY,
    ip TEXT NOT NULL UNIQUE,
    username TEXT NOT NULL,
    password TEXT NOT NULL
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

CREATE TABLE IF NOT EXISTS camera_software_version (
    id INTEGER PRIMARY KEY,
    build TEXT,
    build_date TEXT,
    security_base_line_version TEXT,
    version TEXT,
    web_version TEXT,
    FOREIGN KEY (id) REFERENCES cameras (id) ON DELETE CASCADE
);
