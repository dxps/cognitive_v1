use crate::{
    domain::model::{AttributeValueType, BooleanAttribute, Entity, Id, IntegerAttribute, SmallintAttribute, TextAttribute},
    server::{AppResult, Pagination},
    ui::pages::Name,
};
use sqlx::{postgres::PgRow, FromRow, PgPool, Row};
use std::sync::Arc;

pub struct EntityRepo {
    pub dbcp: Arc<PgPool>,
}

impl EntityRepo {
    //
    pub fn new(dbcp: Arc<PgPool>) -> Self {
        Self { dbcp }
    }

    /// List all the entities.<br/>
    /// Note that the attributes of the entities are not loaded.
    pub async fn list(&self, pagination_opts: Option<&Pagination>) -> AppResult<Vec<Entity>> {
        //
        let (offset, limit) = Pagination::from(pagination_opts).get_offset_limit();
        let query = format!(
            "SELECT e.id, e.def_id, e.listing_attr_def_id, e.listing_attr_name, e.listing_attr_value, ed.name as kind 
             FROM entities e 
             JOIN entity_defs ed ON e.def_id = ed.id 
             ORDER BY name LIMIT {limit} OFFSET {offset}"
        );

        sqlx::query_as::<_, Entity>(query.as_str())
            .fetch_all(self.dbcp.as_ref())
            .await
            .map(|res| AppResult::Ok(res))?
    }

    /// List all the entities by `def_id`.<br/>
    /// Note that the attributes of the entities are not loaded.
    pub async fn list_by_def_id(&self, def_id: &Id) -> AppResult<Vec<Entity>> {
        //
        let query = "SELECT e.id, e.def_id, e.listing_attr_def_id, e.listing_attr_name, e.listing_attr_value, ed.name as kind 
                     FROM entities e 
                     JOIN entity_defs ed ON e.def_id = ed.id 
                     WHERE e.def_id = $1";
        sqlx::query_as::<_, Entity>(query)
            .bind(&def_id.as_str())
            .fetch_all(self.dbcp.as_ref())
            .await
            .map(|res| AppResult::Ok(res))?
    }

    pub async fn list_refs_by_def_id(&self, def_id: &Id) -> AppResult<Vec<(Id, Name)>> {
        //
        let res = sqlx::query_as::<_, (String, Name)>(
            "SELECT e.id, ed.name as kind FROM entities e
             JOIN entity_defs ed ON e.def_id = ed.id 
             WHERE e.def_id = $1",
        )
        .bind(def_id.as_str())
        .fetch_all(self.dbcp.as_ref())
        .await?
        .into_iter()
        .map(|(id, name)| (Id::from(id), name))
        .collect();

        Ok(res)
    }

    pub async fn get(&self, id: &Id) -> AppResult<Option<Entity>> {
        //
        let mut res = None;
        // TODO: use match (to capture the error as well and log it, at least)
        if let Ok(ent_opt) = sqlx::query_as::<_, Entity>(
            "SELECT e.id, e.def_id, e.listing_attr_def_id, e.listing_attr_name, e.listing_attr_value, ed.name as kind 
             FROM entities e 
             JOIN entity_defs ed ON e.def_id = ed.id 
             WHERE e.id = $1",
        )
        .bind(id.as_str())
        .fetch_optional(self.dbcp.as_ref())
        .await
        {
            if let Some(mut ent) = ent_opt {
                // Get the attributes of an entity, all in one shot.
                let query = "
                    SELECT * FROM (
                        SELECT a.id, ad.name, ad.value_type, a.def_id, edad.show_index, a.value as text_value, 
                            CAST (0 as int2) as smallint_value, 0 as integer_value, 0 as bigint_value, 0 as real_value,
                            false as bool_value, CURRENT_DATE as date_value, CURRENT_TIMESTAMP as timestamp_value
                            FROM attribute_defs ad 
                            JOIN text_attributes a ON a.def_id = ad.id
                            JOIN entity_defs_attribute_defs_xref edad
                                ON ad.id = edad.attribute_def_id AND edad.entity_def_id = $2
                            WHERE a.owner_id = $1
                        UNION ALL 
                        SELECT a.id, ad.name, ad.value_type, a.def_id, edad.show_index, '' as text_value, 
                            a.value as smallint_value, 0 as integer_value, 0 as bigint_value, 0 as real_value,
                            false as bool_value, CURRENT_DATE as date_value, CURRENT_TIMESTAMP as timestamp_value
                            FROM attribute_defs ad
                            JOIN smallint_attributes a ON a.def_id = ad.id
                            JOIN entity_defs_attribute_defs_xref edad
                                ON ad.id = edad.attribute_def_id AND edad.entity_def_id = $2
                            WHERE a.owner_id = $1
                        UNION ALL 
                        SELECT a.id, ad.name, ad.value_type, a.def_id, edad.show_index, '' as text_value, 
                            CAST (0 as int2) as smallint_value, a.value as integer_value, 0 as bigint_value, 0 as real_value, 
                            false as bool_value, CURRENT_DATE as date_value, CURRENT_TIMESTAMP as timestamp_value 
                            FROM attribute_defs ad
                            JOIN integer_attributes a ON a.def_id = ad.id
                            JOIN entity_defs_attribute_defs_xref edad
                                ON ad.id = edad.attribute_def_id AND edad.entity_def_id = $2
                            WHERE a.owner_id = $1
                        UNION ALL 
                        SELECT a.id, ad.name, ad.value_type, a.def_id, edad.show_index, '' as text_value, 
                            CAST (0 as int2) as smallint_value, 0 as integer_value, a.value as bigint_value, 0 as real_value,
                            false as bool_value, CURRENT_DATE as date_value, CURRENT_TIMESTAMP as timestamp_value 
                            FROM attribute_defs ad
                            JOIN bigint_attributes a ON a.def_id = ad.id
                            JOIN entity_defs_attribute_defs_xref edad
                                ON ad.id = edad.attribute_def_id AND edad.entity_def_id = $2
                            WHERE a.owner_id = $1
                        UNION ALL 
                        SELECT a.id, ad.name, ad.value_type, a.def_id, edad.show_index, '' as text_value, 
                            CAST (0 as int2) as smallint_value, 0 integer_value, 0 as bigint_value, a.value as real_value,
                            false as bool_value, CURRENT_DATE as date_value, CURRENT_TIMESTAMP as timestamp_value
                            FROM attribute_defs ad
                            JOIN real_attributes a ON a.def_id = ad.id
                            JOIN entity_defs_attribute_defs_xref edad
                                ON ad.id = edad.attribute_def_id AND edad.entity_def_id = $2
                            WHERE a.owner_id = $1
                        UNION ALL 
                        SELECT a.id, ad.name, ad.value_type, a.def_id, edad.show_index, '' as text_value, 
                            CAST (0 as int2) as smallint_value, 0 integer_value, 0 as bigint_value, 0 as real_value,
                            a.value as bool_value, CURRENT_DATE as date_value, CURRENT_TIMESTAMP as timestamp_value
                            FROM attribute_defs ad
                            JOIN boolean_attributes a ON a.def_id = ad.id
                            JOIN entity_defs_attribute_defs_xref edad
                                ON ad.id = edad.attribute_def_id AND edad.entity_def_id = $2
                            WHERE a.owner_id = $1
                        UNION ALL 
                        SELECT a.id, ad.name, ad.value_type, a.def_id, edad.show_index, '' as text_value, 
                            CAST (0 as int2) as smallint_value, 0 integer_value, 0 as bigint_value, 0 as real_value,
                            false as bool_value, a.value as date_value, CURRENT_TIMESTAMP as timestamp_value 
                            FROM attribute_defs ad
                            JOIN date_attributes a ON a.def_id = ad.id
                            JOIN entity_defs_attribute_defs_xref edad
                                ON ad.id = edad.attribute_def_id AND edad.entity_def_id = $2
                            WHERE a.owner_id = $1
                        UNION ALL 
                        SELECT a.id, ad.name, ad.value_type, a.def_id, edad.show_index, '' as text_value, 
                            CAST (0 as int2) as smallint_value, 0 integer_value, 0 as bigint_value, 0 as real_value,
                            false as bool_value, CURRENT_DATE as date_value, a.value as timestamp_value 
                            FROM attribute_defs ad
                            JOIN timestamp_attributes a ON a.def_id = ad.id
                            JOIN entity_defs_attribute_defs_xref edad
                                ON ad.id = edad.attribute_def_id AND edad.entity_def_id = $2
                            WHERE a.owner_id = $1
                    ) ORDER by show_index;
                ";
                let rows = sqlx::query(query)
                    .bind(id.as_str())
                    .bind(ent.def_id.as_str())
                    .fetch_all(self.dbcp.as_ref())
                    .await?;
                fill_in_entity_attributes(&mut ent, rows);
                res = Some(ent);
            }
        };
        Ok(res)
    }

    pub async fn add(&self, ent: &Entity) -> AppResult<()> {
        //
        log::debug!("Adding entity: '{:?}'.", ent);

        let mut txn = self.dbcp.begin().await?;

        if let Err(e) = sqlx::query(
            "INSERT INTO entities (id, def_id, listing_attr_def_id, listing_attr_name, listing_attr_value) 
             VALUES ($1, $2, $3, $4, $5)",
        )
        .bind(&ent.id.as_str())
        .bind(&ent.def_id.as_str())
        .bind(&ent.listing_attr_def_id.as_str())
        .bind(&ent.listing_attr_name)
        .bind(&ent.listing_attr_value)
        .execute(&mut *txn)
        .await
        {
            txn.rollback().await?;
            log::error!("Failed to add entity. Cause: '{}'.", e);
            return AppResult::Err(e.into());
        }

        for attr in ent.text_attributes.iter() {
            if let Err(e) = sqlx::query("INSERT INTO text_attributes (id, owner_id, def_id, value) VALUES ($1, $2, $3, $4)")
                .bind(Id::new().to_string())
                .bind(&ent.id.as_str())
                .bind(attr.def_id.as_str())
                .bind(&attr.value)
                .execute(&mut *txn)
                .await
            {
                txn.rollback().await?;
                log::error!("Failed to add entity's text attribute: {}", e);
                return AppResult::Err(e.into());
            }
        }

        for attr in ent.smallint_attributes.iter() {
            if let Err(e) = sqlx::query("INSERT INTO smallint_attributes (id, owner_id, def_id, value) VALUES ($1, $2, $3, $4)")
                .bind(Id::new().to_string())
                .bind(&ent.id.as_str())
                .bind(attr.def_id.as_str())
                .bind(attr.value)
                .execute(&mut *txn)
                .await
            {
                txn.rollback().await?;
                log::error!("Failed to add an entity smallint attribute. Cause: {}", e);
                return AppResult::Err(e.into());
            }
        }

        for attr in ent.int_attributes.iter() {
            if let Err(e) = sqlx::query("INSERT INTO integer_attributes (id, owner_id, def_id, value) VALUES ($1, $2, $3, $4)")
                .bind(Id::new().to_string())
                .bind(&ent.id.as_str())
                .bind(attr.def_id.as_str())
                .bind(attr.value)
                .execute(&mut *txn)
                .await
            {
                txn.rollback().await?;
                log::error!("Failed to add an entity integer attribute. Cause: {}", e);
                return AppResult::Err(e.into());
            }
        }

        for attr in ent.boolean_attributes.iter() {
            if let Err(e) = sqlx::query("INSERT INTO boolean_attributes (id, owner_id, def_id, value) VALUES ($1, $2, $3, $4)")
                .bind(Id::new().to_string())
                .bind(&ent.id.as_str())
                .bind(attr.def_id.as_str())
                .bind(attr.value)
                .execute(&mut *txn)
                .await
            {
                txn.rollback().await?;
                log::error!("Failed to add an entity boolean attribute. Cause: {}", e);
                return AppResult::Err(e.into());
            }
        }

        txn.commit().await?;
        Ok(())
    }

    pub async fn update(&self, ent: &Entity) -> AppResult<()> {
        //
        let mut txn = self.dbcp.begin().await?;

        for attr in ent.text_attributes.iter() {
            log::debug!("Updating entity id:'{}' w/ text attribute def_id:'{}'", &ent.id, &attr.def_id);
            if let Err(e) = sqlx::query("UPDATE text_attributes SET value = $2 WHERE id = $1")
                .bind(&attr.id.as_str())
                .bind(&attr.value)
                .execute(&mut *txn)
                .await
            {
                txn.rollback().await?;
                log::error!(
                    "Failed to update entity id:'{}' text attribute def_id:'{}'. Cause: '{}'.",
                    &ent.id,
                    &attr.def_id,
                    e
                );
                return AppResult::Err(e.into());
            }
            if ent.listing_attr_def_id == attr.def_id {
                if let Err(e) = sqlx::query("UPDATE entities SET listing_attr_value = $2 WHERE id = $1")
                    .bind(&ent.id.as_str())
                    .bind(&attr.value)
                    .execute(&mut *txn)
                    .await
                {
                    txn.rollback().await?;
                    log::error!(
                        "Failed to update entity id:'{}' listing attribute def_id:'{}'. Cause: '{}'.",
                        &ent.id,
                        &attr.def_id,
                        e
                    );
                    return AppResult::Err(e.into());
                }
            }
        }

        for attr in ent.smallint_attributes.iter() {
            log::debug!("Updating entity id:'{}' w/ smallint attribute def_id:'{}'", &ent.id, &attr.def_id);
            if let Err(e) = sqlx::query("UPDATE smallint_attributes SET value = $2 WHERE id = $1")
                .bind(&attr.id.as_str())
                .bind(&attr.value)
                .execute(&mut *txn)
                .await
            {
                txn.rollback().await?;
                log::error!(
                    "Failed to update entity id:'{}' smallint attribute def_id:'{}'. Cause: '{}'.",
                    &ent.id,
                    &attr.def_id,
                    e
                );
                return AppResult::Err(e.into());
            }
            if ent.listing_attr_def_id == attr.def_id {
                if let Err(e) = sqlx::query("UPDATE entities SET listing_attr_value = $2 WHERE id = $1")
                    .bind(&ent.id.as_str())
                    .bind(&attr.value)
                    .execute(&mut *txn)
                    .await
                {
                    txn.rollback().await?;
                    log::error!(
                        "Failed to update entity id:'{}' listing attribute def_id:'{}'. Cause: '{}'.",
                        &ent.id,
                        &attr.def_id,
                        e
                    );
                    return AppResult::Err(e.into());
                }
            }
        }

        for attr in ent.int_attributes.iter() {
            log::debug!("Updating entity id:'{}' w/ integer attribute def_id:'{}'", &ent.id, &attr.def_id);
            if let Err(e) = sqlx::query("UPDATE integer_attributes SET value = $2 WHERE id = $1")
                .bind(&attr.id.as_str())
                .bind(&attr.value)
                .execute(&mut *txn)
                .await
            {
                txn.rollback().await?;
                log::error!(
                    "Failed to update entity id:'{}' integer attribute def_id:'{}'. Cause: '{}'.",
                    &ent.id,
                    &attr.def_id,
                    e
                );
                return AppResult::Err(e.into());
            }
            if ent.listing_attr_def_id == attr.def_id {
                if let Err(e) = sqlx::query("UPDATE entities SET listing_attr_value = $2 WHERE id = $1")
                    .bind(&ent.id.as_str())
                    .bind(&attr.value)
                    .execute(&mut *txn)
                    .await
                {
                    txn.rollback().await?;
                    log::error!(
                        "Failed to update entity id:'{}' listing attribute def_id:'{}'. Cause: '{}'.",
                        &ent.id,
                        &attr.def_id,
                        e
                    );
                    return AppResult::Err(e.into());
                }
            }
        }

        for attr in ent.boolean_attributes.iter() {
            log::debug!("Updating entity id:'{}' w/ boolean attribute def_id:'{}'", &ent.id, &attr.def_id);
            if let Err(e) = sqlx::query("UPDATE boolean_attributes SET value = $2 WHERE id = $1")
                .bind(&attr.id.as_str())
                .bind(&attr.value)
                .execute(&mut *txn)
                .await
            {
                txn.rollback().await?;
                log::error!(
                    "Failed to update entity id:'{}' boolean attribute def_id:'{}'. Cause: '{}'.",
                    &ent.id,
                    &attr.def_id,
                    e
                );
                return AppResult::Err(e.into());
            }
            if ent.listing_attr_def_id == attr.def_id {
                if let Err(e) = sqlx::query("UPDATE entities SET listing_attr_value = $2 WHERE id = $1")
                    .bind(&ent.id.as_str())
                    .bind(&attr.value)
                    .execute(&mut *txn)
                    .await
                {
                    txn.rollback().await?;
                    log::error!(
                        "Failed to update entity id:'{}' listing attribute def_id:'{}'. Cause: '{}'.",
                        &ent.id,
                        &attr.def_id,
                        e
                    );
                    return AppResult::Err(e.into());
                }
            }
        }

        txn.commit().await?;
        Ok(())
    }

    pub async fn update_listing_attr_name_value_by_ent_def_id(&self, ent_def_id: &Id, attr_id: &Id) -> AppResult<()> {
        //
        let ents = self.list_by_def_id(&ent_def_id).await?;
        log::debug!("[update_listing_attr_name_value] Found ents: {:?}", ents);
        if ents.is_empty() {
            return AppResult::Ok(());
        }
        let mut txn = self.dbcp.begin().await?;
        for mut ent in ents {
            ent = self.get(&ent.id).await?.unwrap();
            for attr in ent.text_attributes.clone() {
                if attr.def_id == *attr_id {
                    ent.listing_attr_name = attr.name;
                    ent.listing_attr_value = attr.value;
                }
            }
            for attr in ent.smallint_attributes.clone() {
                if attr.def_id == *attr_id {
                    ent.listing_attr_name = attr.name;
                    ent.listing_attr_value = format!("{:?}", attr.value);
                }
            }
            for attr in ent.int_attributes.clone() {
                if attr.def_id == *attr_id {
                    ent.listing_attr_name = attr.name;
                    ent.listing_attr_value = format!("{:?}", attr.value);
                }
            }
            for attr in ent.boolean_attributes.clone() {
                if attr.def_id == *attr_id {
                    ent.listing_attr_name = attr.name;
                    ent.listing_attr_value = format!("{:?}", attr.value);
                }
            }
            if let Err(e) = sqlx::query(
                "UPDATE entities 
                    SET listing_attr_name = $1, listing_attr_value = $2 
                    WHERE entities.id = $3",
            )
            .bind(&ent.listing_attr_name)
            .bind(&ent.listing_attr_value)
            .bind(&ent.id.as_str())
            .execute(&mut *txn)
            .await
            {
                log::error!("Failed to update listing attr name and value, based on attr_id: '{attr_id}' of all entities with def_id: '{ent_def_id}'. Cause: {e}");
                return AppResult::Err(e.into());
            }
            log::debug!(
                "Updated listing attr name:'{}' and value:'{}' of entity w/ id: '{}'.",
                &ent.listing_attr_name,
                &ent.listing_attr_value,
                &ent.id
            );
        }
        txn.commit().await?;

        Ok(())
    }

    pub async fn update_listing_attr_name_by_attr_def_id(&self, attr_def_id: &Id, attr_name: &String) -> AppResult<()> {
        //
        let query = "UPDATE entities SET listing_attr_name = $2 WHERE listing_attr_def_id = $1";
        sqlx::query(query)
            .bind(attr_def_id.as_str())
            .bind(attr_name)
            .execute(self.dbcp.as_ref())
            .await
            .map(|_| AppResult::Ok(()))?
    }

    pub async fn remove(&self, id: &Id) -> AppResult<()> {
        //
        let mut txn = self.dbcp.begin().await?;

        if let Err(e) = sqlx::query(
            "WITH del_text_attrs AS (DELETE FROM text_attributes WHERE owner_id = $1 RETURNING *),
              del_smallint_attrs AS (DELETE FROM smallint_attributes WHERE owner_id = $1 RETURNING *),
                   del_int_attrs AS (DELETE FROM integer_attributes WHERE owner_id = $1 RETURNING *),
               del_boolean_attrs AS (DELETE FROM boolean_attributes WHERE owner_id = $1 RETURNING *)
            SELECT * FROM del_text_attrs, del_smallint_attrs, del_int_attrs, del_boolean_attrs",
        )
        .bind(id.as_str())
        .execute(&mut *txn)
        .await
        {
            log::error!("Failed to delete the attributes of entity id:'{}': '{}'.", id, e);
            return AppResult::Err(e.into());
        }

        if let Err(e) = sqlx::query("DELETE FROM entities WHERE id = $1")
            .bind(id.as_str())
            .execute(&mut *txn)
            .await
        {
            log::error!("Failed to delete entity by id:'{}': '{}'.", id, e);
            return AppResult::Err(e.into());
        }

        txn.commit().await?;
        Ok(())
    }
}

impl FromRow<'_, PgRow> for Entity {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: Id::new_from(row.get("id")),
            kind: row.get("kind"),
            def_id: Id::new_from(row.get("def_id")),
            attributes_order: vec![],
            text_attributes: Vec::new(),
            smallint_attributes: Vec::new(),
            int_attributes: Vec::new(),
            boolean_attributes: Vec::new(),
            listing_attr_def_id: Id::new_from(row.get("listing_attr_def_id")),
            listing_attr_name: row.get("listing_attr_name"),
            listing_attr_value: row.get("listing_attr_value"),
        })
    }
}

fn fill_in_entity_attributes(item: &mut Entity, rows: Vec<PgRow>) {
    //
    item.attributes_order = Vec::with_capacity(rows.len());
    for row in rows {
        let id = Id::new_from(row.get("id"));
        let name: String = row.get("name");
        let value_type: &str = row.get("value_type");
        let def_id = Id::new_from(row.get("def_id"));
        match value_type {
            "text" => {
                log::debug!("Found text attribute '{}'.", name);
                item.text_attributes
                    .push(TextAttribute::new(id.clone(), name, row.get("text_value"), def_id, item.id.clone()));
                item.attributes_order.push((AttributeValueType::Text, id));
            }
            "smallint" => {
                log::debug!("Found smallint attribute '{}'.", name);
                item.smallint_attributes.push(SmallintAttribute::new(
                    id.clone(),
                    name,
                    row.get("smallint_value"),
                    def_id,
                    item.id.clone(),
                ));
                item.attributes_order.push((AttributeValueType::SmallInteger, id));
            }
            "integer" => {
                log::debug!("Found integer attribute '{}'.", name);
                item.int_attributes.push(IntegerAttribute::new(
                    id.clone(),
                    name,
                    row.get("integer_value"),
                    def_id,
                    item.id.clone(),
                ));
                item.attributes_order.push((AttributeValueType::Integer, id));
            }
            "boolean" => {
                log::debug!("Found boolean attribute '{}'.", name);
                item.boolean_attributes.push(BooleanAttribute::new(
                    id.clone(),
                    name,
                    row.get("bool_value"),
                    def_id,
                    item.id.clone(),
                ));
                item.attributes_order.push((AttributeValueType::Boolean, id));
            }
            _ => {
                log::warn!(
                    "[fill_in_entity_attributes] Unhandled attribute w/ value_type: '{}' name:'{}'.",
                    value_type,
                    name
                );
            }
        }
    }
}
