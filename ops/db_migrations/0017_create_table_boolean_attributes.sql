CREATE TABLE boolean_attributes
(
    id                   CHAR(10),
    owner_id             CHAR(10),
    def_id               CHAR(10),
    value                BOOLEAN,
    CONSTRAINT boolean_attributes___pk       PRIMARY KEY (id),
    CONSTRAINT boolean_attributes___def_fk   FOREIGN KEY (def_id)   REFERENCES attribute_defs(id)
);

COMMENT ON COLUMN boolean_attributes.def_id is 'The definition id of this attribute.';
