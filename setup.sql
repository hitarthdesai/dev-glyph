CREATE TABLE marks (
    id INTEGER NOT NULL,
    note TEXT DEFAULT '' NOT NULL,
    created_at TIMESTAMP DEFAULT (strftime('%Y-%m-%d %H-%M-%S','now')) NOT NULL
);