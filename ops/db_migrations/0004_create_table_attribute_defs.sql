CREATE TABLE attribute_defs
(
    id              CHAR(10)      PRIMARY KEY,
    name            VARCHAR(64)   NOT NULL,
    description     VARCHAR(256),
    value_type      VARCHAR(16)   NOT NULL,
    default_value   VARCHAR(20),
    required        BOOLEAN       DEFAULT false,
    tag_id          CHAR(10),
    CONSTRAINT tag_fk FOREIGN KEY(tag_id) REFERENCES tags(id),
    CONSTRAINT name_desc_unique UNIQUE NULLS NOT DISTINCT (name, description)
);
