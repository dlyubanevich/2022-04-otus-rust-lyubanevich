-- Add migration script here
DROP TABLE IF EXISTS rooms;
DROP TABLE IF EXISTS devices_rooms;

CREATE TABLE rooms
(
    id          INTEGER PRIMARY KEY AUTOINCREMENT   NOT NULL,
    name        TEXT                                NOT NULL
);

CREATE TABLE devices_rooms
(
    room_id     INTEGER                             NOT NULL,
    device_name TEXT                                NOT NULL,
    FOREIGN KEY (room_id) REFERENCES rooms(id) on delete cascade
);
