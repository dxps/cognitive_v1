use indexmap::IndexMap;
use sqlx::{postgres::PgRow, FromRow, PgPool, Row};
use std::sync::Arc;

use crate::{
    domain::model::{AttributeDef, EntityDef, Id},
    server::{AppError, AppResult, Pagination},
    ui::pages::Name,
};

pub struct EntityDefRepo {
    pub dbcp: Arc<PgPool>,
}

impl EntityDefRepo {
    //
    pub fn new(dbcp: Arc<PgPool>) -> Self {
        Self { dbcp }
    }

    pub async fn list_ids_names(&self) -> AppResult<IndexMap<Id, String>> {
        //
        let query = "SELECT id, name FROM entity_defs ORDER BY name";
        sqlx::query_as::<_, (String, String)>(query)
            .fetch_all(self.dbcp.as_ref())
            .await
            .map(|res| {
                let rs = res.into_iter().map(|(id, name)| (Id::from(id), name)).collect();
                AppResult::Ok(rs)
            })?
    }

    pub async fn list(&self, pagination_opts: Option<&Pagination>) -> AppResult<Vec<EntityDef>> {
        //
        let (offset, limit) = Pagination::from(pagination_opts).get_offset_limit();
        let query = format!(
            "SELECT id, name, description, listing_attr_def_id 
             FROM entity_defs ORDER BY name LIMIT {limit} OFFSET {offset}"
        );

        let mut ent_defs = sqlx::query_as::<_, EntityDef>(query.as_str()) // FYI: Binding (such as .bind(limit) didn't work, that's why the query.
            .fetch_all(self.dbcp.as_ref())
            .await?;

        for ent_def in &mut ent_defs {
            if let Ok(attrs) = sqlx::query_as::<_, AttributeDef>(
                "SELECT id, name, description, value_type, default_value, required, tag_id
                 FROM attribute_defs ad 
                 JOIN entity_defs_attribute_defs_xref edad
                    ON ad.id = edad.attribute_def_id 
                 WHERE edad.entity_def_id = $1 
                 ORDER BY edad.show_index",
            )
            .bind(&ent_def.id.as_str())
            .fetch_all(self.dbcp.as_ref())
            .await
            {
                ent_def.attributes = attrs;
            }
        }

        Ok(ent_defs)
    }

    pub async fn list_refs_by_attr_def_id(&self, attr_def_id: &Id) -> AppResult<Vec<(Id, Name)>> {
        //
        let res = sqlx::query_as::<_, (String, Name)>(
            "SELECT id, name FROM entity_defs 
             JOIN entity_defs_attribute_defs_xref ed_ad_xref ON entity_defs.id = ed_ad_xref.entity_def_id
             WHERE ed_ad_xref.attribute_def_id = $1
             ORDER BY ed_ad_xref.show_index",
        )
        .bind(attr_def_id.as_str())
        .fetch_all(self.dbcp.as_ref())
        .await?
        .into_iter()
        .map(|(id, name)| (Id::from(id), name))
        .collect();

        Ok(res)
    }

    pub async fn add(&self, ent_def: &EntityDef) -> AppResult<()> {
        //
        let mut txn = self.dbcp.begin().await?;

        if let Err(e) = sqlx::query("INSERT INTO entity_defs (id, name, description, listing_attr_def_id) VALUES ($1, $2, $3, $4)")
            .bind(ent_def.id.as_str())
            .bind(ent_def.name.clone())
            .bind(ent_def.description.clone())
            .bind(ent_def.listing_attr_def_id.as_str())
            .execute(&mut *txn)
            .await
        {
            txn.rollback().await?;
            log::error!("Failed to add entity def. Cause: '{}'.", e);
            return AppResult::Err(e.into());
        }

        for (index, attr_def) in ent_def.attributes.clone().iter().enumerate() {
            if let Err(e) =
                sqlx::query("INSERT INTO entity_defs_attribute_defs_xref (entity_def_id, attribute_def_id, show_index) VALUES ($1, $2, $3)")
                    .bind(ent_def.id.as_str())
                    .bind(attr_def.id.as_str())
                    .bind((index + 1) as i16)
                    .execute(&mut *txn)
                    .await
            {
                txn.rollback().await?;
                log::error!("Failed to add entity def's attribute defs: {}", e);
                return AppResult::Err(e.into());
            }
        }

        txn.commit().await?;
        AppResult::Ok(())
    }

    pub async fn get(&self, id: &Id) -> Option<EntityDef> {
        //
        let mut res = None;
        if let Ok(res_opt) =
            sqlx::query_as::<_, EntityDef>("SELECT id, name, description, listing_attr_def_id FROM entity_defs WHERE id = $1")
                .bind(id.as_str())
                .fetch_optional(self.dbcp.as_ref())
                .await
        {
            if let Some(mut ent_def) = res_opt {
                if let Ok(attrs) = sqlx::query_as::<_, AttributeDef>(
                    "SELECT id, name, description, value_type, default_value, required, tag_id 
                     FROM attribute_defs ad JOIN entity_defs_attribute_defs_xref ed_ad_xref 
                     ON ad.id = ed_ad_xref.attribute_def_id where ed_ad_xref.entity_def_id = $1 
                     ORDER BY ed_ad_xref.show_index",
                )
                .bind(id.as_str())
                .fetch_all(self.dbcp.as_ref())
                .await
                {
                    ent_def.attributes = attrs;
                    res = Some(ent_def);
                }
            }
        };
        res
    }

    pub async fn update(&self, ent_def: &EntityDef) -> AppResult<()> {
        //
        let mut txn = self.dbcp.begin().await?;
        if let Err(e) = sqlx::query("UPDATE entity_defs SET name = $1, description = $2, listing_attr_def_id = $3 WHERE id = $4")
            .bind(ent_def.name.clone())
            .bind(ent_def.description.clone())
            .bind(ent_def.listing_attr_def_id.as_str())
            .bind(ent_def.id.as_str())
            .execute(&mut *txn)
            .await
        {
            txn.rollback().await?;
            log::error!("Failed to update entity def: {}", e);
            return AppResult::Err(e.into());
        }

        if let Err(e) = sqlx::query("DELETE FROM entity_defs_attribute_defs_xref WHERE entity_def_id = $1")
            .bind(&ent_def.id.as_str())
            .execute(&mut *txn)
            .await
        {
            txn.rollback().await?;
            log::error!("Failed to delete entity def's (id:{}) attribute def id: {}", ent_def.id, e);
            return AppResult::Err(e.into());
        }

        for (index, attr_def) in ent_def.attributes.clone().iter().enumerate() {
            if let Err(e) =
                sqlx::query("INSERT INTO entity_defs_attribute_defs_xref (entity_def_id, attribute_def_id, show_index) VALUES ($1, $2, $3)")
                    .bind(ent_def.id.as_str())
                    .bind(attr_def.id.as_str())
                    .bind((index + 1) as i16)
                    .execute(&mut *txn)
                    .await
            {
                txn.rollback().await?;
                log::error!("Failed to update entity def's attribute defs: {}", e);
                return AppResult::Err(e.into());
            }
        }

        txn.commit().await?;
        AppResult::Ok(())
    }

    pub async fn remove(&self, id: &Id) -> AppResult<()> {
        //
        let mut txn = self.dbcp.begin().await?;

        if let Err(e) = sqlx::query("DELETE FROM entity_defs_attribute_defs_xref WHERE entity_def_id = $1")
            .bind(id.as_str())
            .execute(&mut *txn)
            .await
        {
            txn.rollback().await?;
            log::error!("Failed to delete entity def attribute def xref: {}", e);
            return AppResult::Err(e.into());
        }

        if let Err(e) = sqlx::query("DELETE FROM entity_defs WHERE id = $1")
            .bind(id.as_str())
            .execute(&mut *txn)
            .await
        {
            txn.rollback().await?;
            if let Some(db_err) = e.as_database_error() {
                if let Some(db_err_code) = db_err.code() {
                    if db_err_code == "23503" {
                        return AppResult::Err(AppError::DependenciesExist);
                    }
                }
            }
            log::error!("Failed to delete entity def: {}", e);
            return AppResult::Err(e.into());
        }

        txn.commit().await?;
        AppResult::Ok(())
    }
}

impl FromRow<'_, PgRow> for EntityDef {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self::new(
            Id::new_from(row.get("id")),
            row.get("name"),
            row.get("description"),
            Id::new_from(row.get("listing_attr_def_id")),
        ))
    }
}
