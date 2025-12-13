use sqlx::{postgres::PgRow, FromRow, PgPool, Row};
use std::sync::Arc;

use crate::{
    domain::model::{AttributeDef, Cardinality, EntityLinkDef, Id},
    server::{AppError, AppResult},
};

pub struct EntityLinkDefRepo {
    pub dbcp: Arc<PgPool>,
}

impl EntityLinkDefRepo {
    //
    pub fn new(dbcp: Arc<PgPool>) -> Self {
        Self { dbcp }
    }

    pub async fn list(&self) -> AppResult<Vec<EntityLinkDef>> {
        //
        let query = "SELECT id, name, description, cardinality, source_entity_def_id, target_entity_def_id  
                     FROM entity_link_defs ORDER BY name";
        let mut items = sqlx::query_as::<_, EntityLinkDef>(query)
            .fetch_all(self.dbcp.as_ref())
            .await
            .map_err(|e| AppError::from(e))?;

        for item in &mut items {
            let attrs = sqlx::query_as::<_, AttributeDef>(
                "SELECT ad.id, ad.name, ad.description, ad.value_type, ad.default_value, ad.required, ad.tag_id 
                 FROM  attribute_defs ad 
                 JOIN entity_link_defs_attribute_defs_xref eld_ad_xref 
                 ON ad.id = eld_ad_xref.attribute_def_id 
                 WHERE eld_ad_xref.entity_link_def_id = $1 ORDER BY name",
            )
            .bind(item.id.as_str())
            .fetch_all(self.dbcp.as_ref())
            .await?
            .into_iter()
            .collect();
            item.attributes = Some(attrs);
        }
        Ok(items)
    }

    pub async fn add(&self, item: &EntityLinkDef) -> AppResult<()> {
        //
        log::debug!("Adding entity link def: {:?}.", item);

        let mut txn = self.dbcp.begin().await?;

        let query = "INSERT INTO entity_link_defs (id, name, description, cardinality, source_entity_def_id, target_entity_def_id) VALUES ($1, $2, $3, $4, $5, $6)";
        if let Err(e) = sqlx::query(query)
            .bind(item.id.as_str())
            .bind(&item.name)
            .bind(&item.description)
            .bind(&item.cardinality.as_string())
            .bind(item.source_entity_def_id.as_str())
            .bind(item.target_entity_def_id.as_str())
            .execute(&mut *txn)
            .await
        {
            txn.rollback().await?;
            log::error!("Failed to add entity link def. Cause: '{}'.", e);
            return AppResult::Err(e.into());
        }

        if item.attributes.is_some() {
            let attrs = item.attributes.as_ref().unwrap();
            for attr in attrs {
                if let Err(e) =
                    sqlx::query("INSERT INTO entity_link_defs_attribute_defs_xref (entity_link_def_id, attribute_def_id) VALUES ($1, $2)")
                        .bind(item.id.as_str())
                        .bind(attr.id.as_str())
                        .execute(&mut *txn)
                        .await
                {
                    txn.rollback().await?;
                    log::error!("Failed to add entity link def attribute. Cause: '{}'.", e);
                    return AppResult::Err(e.into());
                }
            }
        }

        txn.commit().await?;

        Ok(())
    }

    pub async fn get(&self, id: &Id) -> AppResult<Option<EntityLinkDef>> {
        //
        let query = "SELECT id, name, description, cardinality, source_entity_def_id, target_entity_def_id  
                     FROM entity_link_defs WHERE id = $1";

        let res = sqlx::query_as::<_, EntityLinkDef>(query)
            .bind(id.as_str())
            .fetch_optional(self.dbcp.as_ref())
            .await?;

        if res.is_none() {
            return Ok(None);
        }

        let mut res = res.unwrap();

        let attrs = sqlx::query_as::<_, AttributeDef>(
            "SELECT ad.id, ad.name, ad.description, ad.value_type, ad.default_value, ad.required, ad.tag_id 
             FROM  attribute_defs ad 
             JOIN entity_link_defs_attribute_defs_xref eld_ad_xref 
             ON ad.id = eld_ad_xref.attribute_def_id 
             WHERE eld_ad_xref.entity_link_def_id = $1 ORDER BY name",
        )
        .bind(id.as_str())
        .fetch_all(self.dbcp.as_ref())
        .await?
        .into_iter()
        .collect();
        res.attributes = Some(attrs);

        Ok(Some(res))
    }

    pub async fn update(&self, item: &EntityLinkDef) -> AppResult<()> {
        //
        let mut txn = self.dbcp.begin().await?;

        if let Err(e) = sqlx::query(
            "UPDATE entity_link_defs 
             SET name = $2, description = $3, cardinality = $4, source_entity_def_id = $5, target_entity_def_id = $6 
             WHERE id = $1",
        )
        .bind(item.id.as_str())
        .bind(&item.name)
        .bind(&item.description)
        .bind(&item.cardinality.as_string())
        .bind(item.source_entity_def_id.as_str())
        .bind(item.target_entity_def_id.as_str())
        .execute(&mut *txn)
        .await
        {
            txn.rollback().await?;
            log::error!("Failed to update entity link def. Cause: '{}'.", e);
            return AppResult::Err(e.into());
        }

        if let Err(e) = sqlx::query("DELETE FROM entity_link_defs_attribute_defs_xref WHERE entity_link_def_id = $1")
            .bind(item.id.as_str())
            .execute(&mut *txn)
            .await
        {
            txn.rollback().await?;
            log::error!("Failed to delete entity link def's (id:{}) attribute def id: {}", item.id, e);
            return AppResult::Err(e.into());
        }

        if item.attributes.is_some() {
            let attrs = item.attributes.as_ref().unwrap();
            for attr in attrs {
                if let Err(e) =
                    sqlx::query("INSERT INTO entity_link_defs_attribute_defs_xref (entity_link_def_id, attribute_def_id) VALUES ($1, $2)")
                        .bind(item.id.as_str())
                        .bind(attr.id.as_str())
                        .execute(&mut *txn)
                        .await
                {
                    txn.rollback().await?;
                    log::error!("Failed to add entity link def attribute. Cause: '{}'.", e);
                    return AppResult::Err(e.into());
                }
            }
        }

        txn.commit().await?;

        Ok(())
    }

    pub async fn remove(&self, id: &Id) -> AppResult<()> {
        //
        if let Err(e) = sqlx::query("DELETE FROM entity_link_defs WHERE id = $1")
            .bind(id.as_str())
            .execute(self.dbcp.as_ref())
            .await
        {
            if let Some(db_err) = e.as_database_error() {
                if let Some(db_err_code) = db_err.code() {
                    if db_err_code == "23503" {
                        return AppResult::Err(AppError::DependenciesExist);
                    }
                }
            }
            log::error!("Failed to delete entity link def by id:'{}'. Cause: '{}'.", id, e);
            return AppResult::Err(e.into());
        }

        Ok(())
    }
}

impl FromRow<'_, PgRow> for EntityLinkDef {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self::new(
            Id::new_from(row.get("id")),
            row.get("name"),
            row.get("description"),
            Cardinality::from(row.get::<&str, &str>("cardinality")),
            Id::new_from(row.get("source_entity_def_id")),
            Id::new_from(row.get("target_entity_def_id")),
            None,
        ))
    }
}
