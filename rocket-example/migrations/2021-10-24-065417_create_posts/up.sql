CREATE TABLE posts (
    id          INT             PRIMARY KEY,
    title       VARCHAR(255)    NOT NULL,
    body        TEXT            NOT NULL,
    published   TINYINT         NOT NULL DEFAULT 0
)