-- Add migration script here
DROP TABLE IF EXISTS rooms;
DROP TABLE IF EXISTS devices;
DROP TABLE IF EXISTS devices_rooms;

CREATE TABLE rooms
(
    id          INTEGER PRIMARY KEY AUTOINCREMENT   NOT NULL,
    name        TEXT                                NOT NULL
);

CREATE TABLE devices
(
    id          INTEGER PRIMARY KEY AUTOINCREMENT   NOT NULL,
    name        TEXT                                NOT NULL
);

CREATE TABLE devices_rooms
(
    room_id     INTEGER                             NOT NULL,
    device_id   INTEGER                             NOT NULL,
    FOREIGN KEY (room_id)       REFERENCES rooms(id),
    FOREIGN KEY (device_id)     REFERENCES devices(id)
);

INSERT INTO rooms ( id, name ) VALUES ( 1, 'Bedroom' );
INSERT INTO rooms ( id, name ) VALUES ( 2, 'Kitchen' );

INSERT INTO devices ( id, name ) VALUES ( 1, 'Socket' );
INSERT INTO devices ( id, name ) VALUES ( 2, 'Thermometer' );
INSERT INTO devices ( id, name ) VALUES ( 3, 'Socket' );

INSERT INTO devices_rooms ( room_id, device_id ) VALUES ( 1, 1 );
INSERT INTO devices_rooms ( room_id, device_id ) VALUES ( 2, 2 );
INSERT INTO devices_rooms ( room_id, device_id ) VALUES ( 2, 3 );
