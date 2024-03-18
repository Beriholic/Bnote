DROP TABLE IF EXISTS note;
CREATE TABLE note(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    created_at DATETIME DEFAULT (datetime(CURRENT_TIMESTAMP,'localtime'))
    is_delete INTEGER DEFAULT 0,
    category_id INTEGER DEFAULT NULL
);
DROP TABLE IF EXISTS category;
CREATE TABLE category(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    created_at DATETIME DEFAULT (datetime('now')),
    is_delete INTEGER DEFAULT 0
);

DROP TABLE IF EXISTS total_info;
CREATE TABLE total_info(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    total INTEGER DEFAULT 0
);

INSERT INTO total_info (total) VALUES (0);