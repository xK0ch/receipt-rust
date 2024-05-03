CREATE TABLE receipt_item
(
    id               UUID                         NOT NULL DEFAULT uuid_generate_v4() PRIMARY KEY,
    created_at       TIMESTAMP                    NOT NULL DEFAULT current_timestamp,
    last_modified_at TIMESTAMP                    NOT NULL DEFAULT current_timestamp,

    name             VARCHAR(255)                 NOT NULL,
    amount           INTEGER                      NOT NULL,
    price            NUMERIC(10, 2)               NOT NULL,

    receipt_id       UUID REFERENCES receipt (id) NOT NULL
);