CREATE TABLE entity_link_defs_attribute_defs_xref
(
    entity_link_def_id             CHAR(10),
    attribute_def_id               CHAR(10),
    PRIMARY KEY (entity_link_def_id, attribute_def_id),
    CONSTRAINT entity_def_fk       FOREIGN KEY(entity_link_def_id)    REFERENCES entity_link_defs(id) ON DELETE CASCADE,
    CONSTRAINT attribute_def_fk    FOREIGN KEY(attribute_def_id)      REFERENCES attribute_defs(id)
);

COMMENT ON COLUMN entity_link_defs_attribute_defs_xref.entity_link_def_id is 'The definition id of the entity link that has the referred attribute.';
COMMENT ON COLUMN entity_link_defs_attribute_defs_xref.attribute_def_id   is 'The definition id of the attribute that the referred entity link has.';
