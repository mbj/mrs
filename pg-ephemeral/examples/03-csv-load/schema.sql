CREATE TABLE users (
    id         serial      PRIMARY KEY,
    name       text        NOT NULL UNIQUE,
    email      text        NOT NULL UNIQUE,
    created_at timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE events (
    id        bigserial   PRIMARY KEY,
    user_id   int         NOT NULL REFERENCES users(id),
    kind      text        NOT NULL,
    occurred  timestamptz NOT NULL
);
