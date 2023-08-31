-- Add migration script here
CREATE TABLE IF NOT EXISTS user_colors (
    id INTEGER PRIMARY KEY,
    color_name TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS phones (
    id INTEGER PRIMARY KEY,
    number TEXT,
    phone_type TEXT
);

CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY,
    name TEXT,
    phone_id INTEGER,
    color_id INTEGER,
    FOREIGN KEY (color_id) REFERENCES user_colors(id),
    FOREIGN KEY (phone_id) REFERENCES phones(id)
);

INSERT INTO phones (number, phone_type) VALUES ('123-456-7890', 'HOME');
INSERT INTO phones (number, phone_type) VALUES ('987-654-3210', 'WORK');

INSERT INTO user_colors (color_name) VALUES ('pale');
INSERT INTO user_colors (color_name) VALUES ('brown');

INSERT INTO users (name, phone_id, color_id) VALUES ('KOKO', 1, 1);
INSERT INTO users (name, phone_id, color_id) VALUES ('ROMY', 2, 2);



