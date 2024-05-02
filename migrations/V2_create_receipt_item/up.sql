CREATE TABLE receipt_item
(
    id               UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    created_at       TIMESTAMP                    NOT NULL,
    last_modified_at TIMESTAMP                    NOT NULL,

    name             VARCHAR(255)                 NOT NULL,
    amount           INTEGER                      NOT NULL,
    price            NUMERIC(10, 2)               NOT NULL,

    receipt_id       UUID REFERENCES receipt (id) NOT NULL
);