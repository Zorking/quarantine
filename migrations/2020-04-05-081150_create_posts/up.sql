-- Your SQL goes here

CREATE TABLE tasks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    description VARCHAR NOT NULL
);

INSERT INTO tasks (description) VALUES ("demo task");
INSERT INTO tasks (description) VALUES ("demo task2")
