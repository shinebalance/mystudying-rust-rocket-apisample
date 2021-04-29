CREATE TABLE tasks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    description VARCHAR NOT NULL,
    completed BOOLEAN NOT NULL DEFAULT 0
);

INSERT INTO tasks (description) VALUES ("demo task");
INSERT INTO tasks (description) VALUES ("demo task2");


CREATE TABLE records (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    description VARCHAR NOT NULL,
    completed BOOLEAN NOT NULL DEFAULT 0
);

INSERT INTO records (description) VALUES ("demo task");
INSERT INTO records (description) VALUES ("demo task2");
