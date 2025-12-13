CREATE TABLE bigint_attributes
(
    id                   CHAR(10),
    owner_id             CHAR(10),
    def_id               CHAR(10),
    value                BIGINT,
    CONSTRAINT bigint_attributes___pk       PRIMARY KEY (id),
    CONSTRAINT bigint_attributes___def_fk   FOREIGN KEY (def_id)   REFERENCES attribute_defs(id)
);

COMMENT ON COLUMN bigint_attributes.def_id is 'The definition id of this attribute.';
