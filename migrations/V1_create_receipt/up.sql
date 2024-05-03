CREATE
    EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE receipt
(
    id               UUID           NOT NULL DEFAULT uuid_generate_v4() PRIMARY KEY,
    created_at       TIMESTAMP      NOT NULL DEFAULT current_timestamp,
    last_modified_at TIMESTAMP      NOT NULL DEFAULT current_timestamp,

    sum              NUMERIC(10, 2) NOT NULL
);
