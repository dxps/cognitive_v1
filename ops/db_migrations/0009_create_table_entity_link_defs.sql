CREATE TABLE entity_link_defs
(
    id                      CHAR(10)         PRIMARY KEY,
    name                    VARCHAR(32),
    description             VARCHAR(256),
    cardinality             CHAR(3)          NOT NULL   DEFAULT '1:1',
    source_entity_def_id    CHAR(10)         NOT NULL,
    target_entity_def_id    CHAR(10)         NOT NULL,
    CONSTRAINT cardinality_check       CHECK(cardinality in ('1:1', '1:M', 'M:M')),
    CONSTRAINT source_entity_def_fk    FOREIGN KEY(source_entity_def_id)    REFERENCES entity_defs(id),
    CONSTRAINT target_entity_def_fk    FOREIGN KEY(target_entity_def_id)    REFERENCES entity_defs(id)
);
