CREATE
EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE receipt
(
    id               UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    sum              NUMERIC(10, 2) NOT NULL,
    created_at       TIMESTAMP      NOT NULL,
    last_modified_at TIMESTAMP      NOT NULL
);
