CREATE TABLE receipt_item
(
    id               UUID                         NOT NULL DEFAULT uuid_generate_v4() PRIMARY KEY,
    created_at       TIMESTAMP WITH TIME ZONE     NOT NULL,
    last_modified_at TIMESTAMP WITH TIME ZONE     NOT NULL,

    name             VARCHAR(255)                 NOT NULL,
    amount           INTEGER                      NOT NULL,
    price            NUMERIC(10, 2)               NOT NULL,

    receipt_id       UUID REFERENCES receipt (id) NOT NULL
);