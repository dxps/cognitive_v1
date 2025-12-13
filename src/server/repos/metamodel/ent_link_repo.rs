use crate::{
    domain::model::{BooleanAttribute, EntityLink, Id, IntegerAttribute, ItemType, SmallintAttribute, TextAttribute},
    server::{AppResult, Pagination},
};
use sqlx::{postgres::PgRow, FromRow, PgPool, Row};
use std::sync::Arc;

pub struct EntityLinkRepo {
    pub dbcp: Arc<PgPool>,
}

impl EntityLinkRepo {
    //
    pub fn new(dbcp: Arc<PgPool>) -> Self {
        Self { dbcp }
    }

    /// List all the entities.<br/>
    /// Note that the attributes of the entities are not loaded.
    pub async fn list(&self, pagination_opts: Option<&Pagination>) -> AppResult<Vec<EntityLink>> {
        //
        let (offset, limit) = Pagination::from(pagination_opts).get_offset_limit();
        let query = format!(
            "SELECT el.id, el.def_id, el.source_entity_id, el.target_entity_id, eld.name as kind 
             FROM entity_links el 
             JOIN entity_link_defs eld ON el.def_id = eld.id 
             ORDER BY name LIMIT {limit} OFFSET {offset}"
        );

        sqlx::query_as::<_, EntityLink>(query.as_str())
            .fetch_all(self.dbcp.as_ref())
            .await
            .map(|res| AppResult::Ok(res))?
    }

    /// List all the entity links by `def_id`.<br/>
    /// Note that their attributes are not loaded.
    pub async fn list_by_def_id(&self, def_id: &Id) -> AppResult<Vec<EntityLink>> {
        //
        let query = "SELECT el.id, el.def_id, el.source_entity_id, el.target_entity_id, eld.name as kind 
                     FROM entity_links el 
                     JOIN entity_link_defs eld ON el.def_id = eld.id  
                     WHERE el.def_id = $1";
        sqlx::query_as::<_, EntityLink>(query)
            .bind(&def_id.as_str())
            .fetch_all(self.dbcp.as_ref())
            .await
            .map(|res| AppResult::Ok(res))?
    }

    pub async fn add(&self, ent_link: &EntityLink) -> AppResult<()> {
        //
        log::debug!("Adding entity link: '{:?}'.", ent_link);

        let mut txn = self.dbcp.begin().await?;

        if let Err(e) = sqlx::query("INSERT INTO entity_links (id, def_id, source_entity_id, target_entity_id) VALUES ($1, $2, $3, $4)")
            .bind(&ent_link.id.as_str())
            .bind(&ent_link.def_id.as_str())
            .bind(&ent_link.source_entity_id.as_str())
            .bind(&ent_link.target_entity_id.as_str())
            .execute(&mut *txn)
            .await
        {
            txn.rollback().await?;
            log::error!("Failed to add entity link. Cause: '{}'.", e);
            return AppResult::Err(e.into());
        }

        for attr in ent_link.text_attributes.iter() {
            if let Err(e) = sqlx::query("INSERT INTO text_attributes (id, owner_id, def_id, value) VALUES ($1, $2, $3, $4)")
                .bind(Id::new().to_string())
                .bind(&ent_link.id.as_str())
                .bind(&attr.def_id.as_str())
                .bind(&attr.value)
                .execute(&mut *txn)
                .await
            {
                txn.rollback().await?;
                log::error!(
                    "Failed to add entity link text attribute w/ owner_id: '{}' def_id:'{}' value:'{}'. Reason: '{}'.",
                    &ent_link.id.as_str(),
                    &attr.def_id.as_str(),
                    &attr.value,
                    e
                );
                return AppResult::Err(e.into());
            }
        }

        for attr in ent_link.smallint_attributes.iter() {
            if let Err(e) = sqlx::query("INSERT INTO smallint_attributes (id, owner_id, def_id, value) VALUES ($1, $2, $3, $4)")
                .bind(Id::new().to_string())
                .bind(&ent_link.id.as_str())
                .bind(ItemType::EntityLink.value())
                .bind(&attr.def_id.as_str())
                .bind(&attr.value)
                .execute(&mut *txn)
                .await
            {
                txn.rollback().await?;
                log::error!("Failed to add entity link smallint attribute. Cause: '{}'.", e);
                return AppResult::Err(e.into());
            }
        }

        for attr in ent_link.int_attributes.iter() {
            if let Err(e) = sqlx::query("INSERT INTO int_attributes (id, owner_id, def_id, value) VALUES ($1, $2, $3, $4)")
                .bind(Id::new().to_string())
                .bind(&ent_link.id.as_str())
                .bind(ItemType::EntityLink.value())
                .bind(&attr.def_id.as_str())
                .bind(&attr.value)
                .execute(&mut *txn)
                .await
            {
                txn.rollback().await?;
                log::error!("Failed to add entity link int attribute. Cause: '{}'.", e);
                return AppResult::Err(e.into());
            }
        }

        for attr in ent_link.boolean_attributes.iter() {
            if let Err(e) = sqlx::query("INSERT INTO boolean_attributes (id, owner_id, def_id, value) VALUES ($1, $2, $3, $4)")
                .bind(Id::new().to_string())
                .bind(&ent_link.id.as_str())
                .bind(&attr.def_id.as_str())
                .bind(&attr.value)
                .execute(&mut *txn)
                .await
            {
                txn.rollback().await?;
                log::error!("Failed to add entity link boolean attribute. Cause: '{}'.", e);
                return AppResult::Err(e.into());
            }
        }

        txn.commit().await?;

        Ok(())
    }

    pub async fn get(&self, id: &Id) -> AppResult<Option<EntityLink>> {
        //
        log::debug!("Getting by id:'{}' ...", id);

        let mut res = None;
        match sqlx::query_as::<_, EntityLink>(
            "SELECT el.id, el.def_id, el.source_entity_id, el.target_entity_id, eld.name as kind
             FROM entity_links el 
             JOIN entity_link_defs eld ON el.def_id = eld.id
             WHERE el.id = $1",
        )
        .bind(id.as_str())
        .fetch_optional(self.dbcp.as_ref())
        .await
        {
            Ok(ent_link_opt) => {
                if let Some(mut ent_link) = ent_link_opt {
                    // Get the attributes, all in one shot.
                    let query = "
                    SELECT a.id, ad.name, ad.value_type, a.def_id, a.value as text_value, 0 as smallint_value, 0 as integer_value, 0 as bigint_value, 0 as real_value,
                        false as bool_value, CURRENT_DATE as date_value, CURRENT_TIMESTAMP as timestamp_value
                        FROM attribute_defs ad 
                        JOIN text_attributes a ON a.def_id = ad.id  
                        WHERE a.owner_id = $1
                    UNION ALL 
                    SELECT a.id, ad.name, ad.value_type, a.def_id, '' as text_value, a.value as smallint_value, 0 as integer_value, 0 as bigint_value, 0 as real_value,
                        false as bool_value, CURRENT_DATE as date_value, CURRENT_TIMESTAMP as timestamp_value
                        FROM attribute_defs ad
                        JOIN smallint_attributes a ON a.def_id = ad.id
                        WHERE a.owner_id = $1
                    UNION ALL 
                    SELECT a.id, ad.name, ad.value_type, a.def_id, '' as text_value, 0 as smallint_value, a.value as integer_value, 0 as bigint_value, 0 as real_value, 
                        false as bool_value, CURRENT_DATE as date_value, CURRENT_TIMESTAMP as timestamp_value 
                        FROM attribute_defs ad
                        JOIN integer_attributes a ON a.def_id = ad.id
                        WHERE a.owner_id = $1
                    UNION ALL 
                    SELECT a.id, ad.name, ad.value_type, a.def_id, '' as text_value, 0 as smallint_value, 0 as integer_value, a.value as bigint_value, 0 as real_value,
                        false as bool_value, CURRENT_DATE as date_value, CURRENT_TIMESTAMP as timestamp_value 
                        FROM attribute_defs ad
                        JOIN bigint_attributes a ON a.def_id = ad.id
                        WHERE a.owner_id = $1
                    UNION ALL 
                    SELECT a.id, ad.name, ad.value_type, a.def_id, '' as text_value, 0 as smallint_value, 0 integer_value, 0 as bigint_value, a.value as real_value,
                        false as bool_value, CURRENT_DATE as date_value, CURRENT_TIMESTAMP as timestamp_value
                        FROM attribute_defs ad
                        JOIN real_attributes a ON a.def_id = ad.id
                        WHERE a.owner_id = $1
                    UNION ALL 
                    SELECT a.id, ad.name, ad.value_type, a.def_id, '' as text_value, 0 as smallint_value, 0 integer_value, 0 as bigint_value, 0 as real_value,
                        a.value as bool_value, CURRENT_DATE as date_value, CURRENT_TIMESTAMP as timestamp_value
                        FROM attribute_defs ad
                        JOIN boolean_attributes a ON a.def_id = ad.id
                        WHERE a.owner_id = $1
                    UNION ALL 
                    SELECT a.id, ad.name, ad.value_type, a.def_id, '' as text_value, 0 as smallint_value, 0 integer_value, 0 as bigint_value, 0 as real_value,
                        false as bool_value, a.value as date_value, CURRENT_TIMESTAMP as timestamp_value 
                        FROM attribute_defs ad
                        JOIN date_attributes a ON a.def_id = ad.id
                        WHERE a.owner_id = $1
                    UNION ALL 
                    SELECT a.id, ad.name, ad.value_type, a.def_id, '' as text_value, 0 as smallint_value, 0 integer_value, 0 as bigint_value, 0 as real_value,
                        false as bool_value, CURRENT_DATE as date_value, a.value as timestamp_value 
                        FROM attribute_defs ad
                        JOIN timestamp_attributes a ON a.def_id = ad.id
                        WHERE a.owner_id = $1;
                ";
                    let rows = sqlx::query(query).bind(id.as_str()).fetch_all(self.dbcp.as_ref()).await?;
                    fill_in_entity_link_attributes(&mut ent_link, rows);
                    res = Some(ent_link);
                }
            }
            Err(e) => {
                log::error!("Failed to query an entry w/ id: '{}'. Reason: '{}'.", id, e);
            }
        }
        Ok(res)
    }

    pub async fn update(&self, item: &EntityLink) -> AppResult<()> {
        //
        log::debug!("Updating entity link: '{:?}'.", item);

        let mut txn = self.dbcp.begin().await?;

        if let Err(e) = sqlx::query("UPDATE entity_links SET source_entity_id = $2, target_entity_id = $3 WHERE id = $1")
            .bind(&item.id.as_str())
            .bind(&item.source_entity_id.as_str())
            .bind(&item.target_entity_id.as_str())
            .execute(&mut *txn)
            .await
        {
            txn.rollback().await?;
            log::error!("Failed to update entity link w/ id:'{}'. Reason: '{}'.", item.id, e);
            return AppResult::Err(e.into());
        }

        for attr in item.text_attributes.iter() {
            if let Err(e) = sqlx::query("UPDATE text_attributes SET value = $2 WHERE id = $1")
                .bind(&attr.id.as_str())
                .bind(&attr.value)
                .execute(&mut *txn)
                .await
            {
                txn.rollback().await?;
                log::error!(
                    "Failed to update entity link w/ id:'{}' on text attribute '{}' as '{}'. Reason: '{}'.",
                    item.id,
                    attr.name,
                    attr.value,
                    e,
                );
                return AppResult::Err(e.into());
            }
        }

        for attr in item.smallint_attributes.iter() {
            if let Err(e) = sqlx::query("UPDATE smallint_attributes SET value = $2 WHERE id = $1")
                .bind(&attr.id.as_str())
                .bind(attr.value)
                .execute(&mut *txn)
                .await
            {
                txn.rollback().await?;
                log::error!(
                    "Failed to update entity link w/ id:'{}' on smallint attribute '{}' as '{}'. Reason: '{}'.",
                    item.id,
                    attr.name,
                    attr.value,
                    e,
                );
                return AppResult::Err(e.into());
            }
        }

        for attr in item.int_attributes.iter() {
            if let Err(e) = sqlx::query("UPDATE integer_attributes SET value = $2 WHERE id = $1")
                .bind(&attr.id.as_str())
                .bind(attr.value)
                .execute(&mut *txn)
                .await
            {
                txn.rollback().await?;
                log::error!(
                    "Failed to update entity link w/ id:'{}' on integer attribute '{}' as '{}'. Reason: '{}'.",
                    item.id,
                    attr.name,
                    attr.value,
                    e,
                );
                return AppResult::Err(e.into());
            }
        }

        for attr in item.boolean_attributes.iter() {
            if let Err(e) = sqlx::query("UPDATE boolean_attributes SET value = $2 WHERE id= $1")
                .bind(&attr.id.as_str())
                .bind(attr.value)
                .execute(&mut *txn)
                .await
            {
                txn.rollback().await?;
                log::error!(
                    "Failed to update entity link w/ id:'{}' on boolean attribute '{}' as '{}'. Reason: '{}'.",
                    item.id,
                    attr.name,
                    attr.value,
                    e,
                );
                return AppResult::Err(e.into());
            }
        }

        txn.commit().await?;

        Ok(())
    }

    pub async fn remove(&self, id: &Id) -> AppResult<()> {
        //
        log::debug!("Deleting entity link w/ id:'{}' ...", id);

        let mut txn = self.dbcp.begin().await?;

        if let Err(e) = sqlx::query("DELETE FROM entity_links WHERE id = $1")
            .bind(id.as_str())
            .execute(&mut *txn)
            .await
        {
            log::error!("Failed to delete entity link w/ id:'{}'. Cause: '{}'.", id, e);
            return AppResult::Err(e.into());
        }

        // TODO Cleanup from all _attributes tables.
        if let Err(e) = sqlx::query("DELETE FROM text_attributes WHERE owner_id = $1")
            .bind(id.as_str())
            .execute(&mut *txn)
            .await
        {
            log::error!("Failed to delete entity link w/ id:'{}'. Cause: '{}'.", id, e);
            return AppResult::Err(e.into());
        }

        txn.commit().await?;

        Ok(())
    }
}

impl FromRow<'_, PgRow> for EntityLink {
    //
    fn from_row(row: &PgRow) -> sqlx::Result<Self> {
        Ok(EntityLink {
            id: Id::new_from(row.get("id")),
            kind: row.try_get("kind")?,
            def_id: Id::new_from(row.get("def_id")),
            source_entity_id: Id::new_from(row.get("source_entity_id")),
            target_entity_id: Id::new_from(row.get("target_entity_id")),
            text_attributes: vec![],
            smallint_attributes: vec![],
            int_attributes: vec![],
            boolean_attributes: vec![],
        })
    }
}

fn fill_in_entity_link_attributes(item: &mut EntityLink, rows: Vec<PgRow>) {
    //
    for row in rows {
        let id = Id::new_from(row.get("id"));
        let name: String = row.get("name");
        let value_type: &str = row.get("value_type");
        let def_id = Id::new_from(row.get("def_id"));
        match value_type {
            "text" => {
                log::debug!("Found text attribute '{}'.", name);
                item.text_attributes
                    .push(TextAttribute::new(id, name, row.get("text_value"), def_id, item.id.clone()));
            }
            "smallint" => {
                log::debug!("Found smallint attribute '{}'.", name);
                item.smallint_attributes
                    .push(SmallintAttribute::new(id, name, row.get("smallint_value"), def_id, item.id.clone()));
            }
            "integer" => {
                log::debug!("Found integer attribute '{}'.", name);
                item.int_attributes
                    .push(IntegerAttribute::new(id, name, row.get("integer_value"), def_id, item.id.clone()));
            }
            "boolean" => {
                log::debug!("Found boolean attribute '{}'.", name);
                item.boolean_attributes
                    .push(BooleanAttribute::new(id, name, row.get("bool_value"), def_id, item.id.clone()));
            }
            _ => {
                log::warn!(
                    "[fill_in_entity_link_attributes] Unhandled attribute w/ value_type: '{}' name:'{}'.",
                    value_type,
                    name
                );
            }
        }
    }
}
