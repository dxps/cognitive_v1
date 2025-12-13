CREATE TABLE entity_defs_attribute_defs_xref
(
    entity_def_id      CHAR(10),
    attribute_def_id   CHAR(10),
    show_index         INT2       NOT NULL   CHECK(show_index > 0),
    PRIMARY KEY (entity_def_id, attribute_def_id),
    CONSTRAINT entity_def_fk      FOREIGN KEY(entity_def_id)      REFERENCES entity_defs(id),
    CONSTRAINT attribute_def_fk   FOREIGN KEY(attribute_def_id)   REFERENCES attribute_defs(id)
);

COMMENT ON COLUMN entity_defs_attribute_defs_xref.entity_def_id    is 'The definition id of the entity that has the attribute.';
COMMENT ON COLUMN entity_defs_attribute_defs_xref.attribute_def_id is 'The definition id of the attribute that the referred entity definition has.';
