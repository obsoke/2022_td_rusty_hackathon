CREATE TABLE IF NOT EXISTS categories
(
    id          INTEGER PRIMARY KEY NOT NULL,
    name        TEXT                NOT NULL
);

CREATE TABLE IF NOT EXISTS cards
(
    id          INTEGER PRIMARY KEY NOT NULL,
    question    TEXT                NOT NULL,
    answer      TEXT                NOT NULL,
    category    INTEGER             NOT NULL,
    FOREIGN KEY(category) REFERENCES categories(id)
);
