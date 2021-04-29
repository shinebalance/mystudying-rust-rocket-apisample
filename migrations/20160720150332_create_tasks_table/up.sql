CREATE TABLE tasks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    description VARCHAR NOT NULL,
    completed BOOLEAN NOT NULL DEFAULT 0
);

INSERT INTO tasks (description) VALUES ("demo task");
INSERT INTO tasks (description) VALUES ("demo task2");


CREATE TABLE records (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    wakeupdatetime VARCHAR NOT NULL,
    condition INTEGER NOT NULL,
    description VARCHAR NOT NULL,
    isperiod BOOLEAN NOT NULL DEFAULT 0
);

INSERT INTO records (
    wakeupdatetime, condition, description, isperiod
) VALUES ("2021-04-20 07:00:00", "5", "demo record", 0);
INSERT INTO records (
    wakeupdatetime, condition, description, isperiod
) VALUES ("2021-04-21 07:00:00", "5", "demo record2", 0);
