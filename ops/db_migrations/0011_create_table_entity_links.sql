CREATE TABLE entity_links
(
    id                    CHAR(10)   PRIMARY KEY,
    def_id                CHAR(10)   NOT NULL,
    source_entity_id      CHAR(10)   NOT NULL,
    target_entity_id      CHAR(10)   NOT NULL,
    CONSTRAINT entity_link_def_fk   FOREIGN KEY(def_id)             REFERENCES entity_link_defs(id),
    CONSTRAINT source_entity_fk     FOREIGN KEY(source_entity_id)   REFERENCES entities(id),
    CONSTRAINT target_entity_fk     FOREIGN KEY(target_entity_id)   REFERENCES entities(id)
);

COMMENT ON COLUMN entity_links.def_id is 'The definition id of this entity link.';
