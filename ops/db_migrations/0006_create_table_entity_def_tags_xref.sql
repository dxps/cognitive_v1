CREATE TABLE entity_def_tags_xref
(
    entity_def_id      CHAR(10),
    tag_id             CHAR(10),
    PRIMARY KEY (entity_def_id, tag_id),
    CONSTRAINT entity_def_fk    FOREIGN KEY(entity_def_id) REFERENCES entity_defs(id),
    CONSTRAINT attribute_def_fk FOREIGN KEY(tag_id)        REFERENCES tags(id)
);
