CREATE TABLE entity_defs
(
    id                      CHAR(10)      PRIMARY KEY,
    name                    VARCHAR(64)   NOT NULL,
    description             VARCHAR(256),
    listing_attr_def_id     CHAR(10)      NOT NULL,
    CONSTRAINT ent_def_listing_attr_def_fk  FOREIGN KEY(listing_attr_def_id) REFERENCES attribute_defs(id)
);
