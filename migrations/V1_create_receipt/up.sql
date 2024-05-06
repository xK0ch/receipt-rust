CREATE
    EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE receipt
(
    id               UUID                     NOT NULL DEFAULT uuid_generate_v4() PRIMARY KEY,
    created_at       TIMESTAMP WITH TIME ZONE NOT NULL,
    last_modified_at TIMESTAMP WITH TIME ZONE NOT NULL,

    sum              NUMERIC(10, 2)           NOT NULL
);
