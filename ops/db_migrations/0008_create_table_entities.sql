CREATE TABLE entities
(
    id                     CHAR(10)      PRIMARY KEY,
    def_id                 CHAR(10),
    listing_attr_def_id    CHAR(10)      NOT NULL,
    listing_attr_name      VARCHAR(64)   NOT NULL,
    listing_attr_value     VARCHAR(64),
    CONSTRAINT def_fk FOREIGN KEY(def_id) REFERENCES entity_defs(id),
    CONSTRAINT listing_attr_def_fk FOREIGN KEY(listing_attr_def_id) REFERENCES attribute_defs(id)
);

COMMENT ON COLUMN entities.def_id is 'The definition id of this entity.';
