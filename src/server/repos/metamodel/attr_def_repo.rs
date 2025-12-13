use crate::domain::model::Id;
use crate::{
    domain::model::{AttributeDef, AttributeValueType},
    server::{AppError, AppResult, Pagination},
};
use sqlx::{postgres::PgRow, FromRow, PgPool, Row};
use std::sync::Arc;

pub struct AttributeDefRepo {
    pub dbcp: Arc<PgPool>,
}

impl AttributeDefRepo {
    //
    pub fn new(dbcp: Arc<PgPool>) -> Self {
        Self { dbcp }
    }

    pub async fn get(&self, id: &Id) -> Option<AttributeDef> {
        //
        sqlx::query_as::<_, AttributeDef>(
            "SELECT id, name, description, value_type, default_value, required, tag_id 
             FROM attribute_defs WHERE id = $1",
        )
        .bind(id.as_str())
        .fetch_one(self.dbcp.as_ref())
        .await
        .ok()
    }

    pub async fn list(&self, pagination_opts: Option<&Pagination>) -> Vec<AttributeDef> {
        //
        let (offset, limit) = Pagination::from(pagination_opts).get_offset_limit();
        let query = format!(
            "SELECT id, name, description, value_type, default_value, required, tag_id 
             FROM attribute_defs ORDER BY name LIMIT {limit} OFFSET {offset}"
        );
        log::debug!("Listing attribute defs w/ limit: {}, offset: {}.", limit, offset);

        sqlx::query_as::<_, AttributeDef>(query.as_str())
            .fetch_all(self.dbcp.as_ref())
            .await
            .ok()
            .unwrap_or_default()
    }

    /// Add a new attribute definition. It returns the id of the repository entry.
    pub async fn add(&self, item: &AttributeDef) -> AppResult<()> {
        //
        sqlx::query(
            "INSERT INTO attribute_defs (id, name, description, value_type, default_value, required, tag_id)
             VALUES ($1, $2, $3, $4, $5, $6, $7)",
        )
        .bind(&item.id.as_str())
        .bind(&item.name)
        .bind(&item.description)
        .bind(item.value_type.to_string())
        .bind(&item.default_value)
        .bind(item.is_required)
        .bind(item.tag_id.as_ref().map(|id| id.as_str()))
        .execute(self.dbcp.as_ref())
        .await
        .map(|_| Ok(()))
        .map_err(|e| {
            if e.to_string().contains("name_desc_unique") {
                AppError::NameDescriptionNotUnique
            } else {
                log::error!("Failed to add attribute definition. Reason: '{}'.", e);
                AppError::Err("An internal error occurred.".into())
            }
        })?
    }

    /// Edit an existing attribute definition.
    pub async fn update(&self, item: &AttributeDef) -> AppResult<()> {
        //
        let tag_id = item.tag_id.as_ref().map(|id| id.as_str());
        sqlx::query(
            "UPDATE attribute_defs 
             SET name=$2, description=$3, value_type=$4, default_value=$5, required=$6, tag_id=$7 
             WHERE id = $1",
        )
        .bind(&item.id.as_str())
        .bind(&item.name)
        .bind(&item.description)
        .bind(item.value_type.to_string())
        .bind(&item.default_value)
        .bind(item.is_required)
        .bind(tag_id)
        .execute(self.dbcp.as_ref())
        .await
        .map(|_| Ok(()))
        .map_err(|e| {
            if e.to_string().contains("name_desc_unique") {
                AppError::NameDescriptionNotUnique
            } else {
                log::error!("Failed to update attribute definition. Reason: '{}'.", e);
                AppError::Err("An internal error occurred.".into())
            }
        })?
    }

    /// Remove (delete) an existing attribute definition.
    pub async fn remove(&self, id: &Id) -> AppResult<()> {
        //
        match sqlx::query("DELETE FROM attribute_defs WHERE id = $1")
            .bind(id.as_str())
            .execute(self.dbcp.as_ref())
            .await
        {
            Ok(_) => AppResult::Ok(()),
            Err(e) => {
                if let Some(db_err) = e.as_database_error() {
                    if let Some(db_err_code) = db_err.code() {
                        // 23503 is postgres specific code for dependencies (named "foreign_key_violation").
                        // See: https://www.postgresql.org/docs/16/errcodes-appendix.htm
                        if db_err_code.as_ref() == "23503" {
                            return AppResult::Err(AppError::Err(
                                "Cannot delete it because it is included in the following entity definitions:".to_string(),
                            ));
                        }
                    }
                }
                log::error!("Failed to delete entry: {}", e);
                AppResult::Err(AppError::InternalErr)
            }
        }
    }
}

impl FromRow<'_, PgRow> for AttributeDef {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        let tag_id = match row.try_get("tag_id") {
            Ok(tag_id) => Some(Id::new_from(tag_id)),
            Err(_) => None,
        };
        Ok(Self {
            id: Id::new_from(row.get("id")),
            name: row.get("name"),
            description: row.get("description"),
            value_type: AttributeValueType::from(row.get::<&str, &str>("value_type")),
            default_value: row.get("default_value"),
            is_required: row.get("required"),
            tag_id,
        })
    }
}
