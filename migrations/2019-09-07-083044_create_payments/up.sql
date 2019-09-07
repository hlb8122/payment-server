CREATE TYPE public.payment_state_enum AS ENUM
    ('confirmed', 'pending', 'received', 'rejected', 'expired');

CREATE TABLE public.payments
(
    id uuid NOT NULL,
    issue_time timestamp without time zone NOT NULL,
    amount bigint NOT NULL,
    address text NOT NULL,
    expiry_time timestamp without time zone,
    req_memo text COLLATE pg_catalog."default",
    merchant_data bytea,
    ack_memo text COLLATE pg_catalog."default",
    tokenize boolean NOT NULL,
    tx_data bytea,
    payment_state payment_state_enum,
    payment_time timestamp without time zone,
    tx_id text COLLATE pg_catalog."default",
    refund_to text COLLATE pg_catalog."default",
    callback_url text COLLATE pg_catalog."default",
    CONSTRAINT payments_pkey PRIMARY KEY (id)
)