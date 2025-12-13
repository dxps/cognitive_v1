CREATE TABLE real_attributes
(
    id                   CHAR(10),
    owner_id             CHAR(10),
    def_id               CHAR(10),
    value                REAL,
    CONSTRAINT real_attributes___pk       PRIMARY KEY (id),
    CONSTRAINT real_attributes___def_fk   FOREIGN KEY (def_id)   REFERENCES attribute_defs(id)
);

COMMENT ON COLUMN real_attributes.def_id is 'The definition id of this attribute.';
