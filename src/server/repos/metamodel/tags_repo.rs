use std::sync::Arc;

use sqlx::{postgres::PgRow, FromRow, PgPool, Row};

use crate::{
    domain::model::{Id, Tag},
    server::{AppError, AppResult, Pagination},
};

pub struct TagsRepo {
    pub dbcp: Arc<PgPool>,
}

impl TagsRepo {
    //
    pub fn new(dbcp: Arc<PgPool>) -> Self {
        Self { dbcp }
    }

    pub async fn get(&self, id: String) -> AppResult<Option<Tag>> {
        //
        match sqlx::query_as::<_, Tag>("SELECT id, name, description FROM tags WHERE id = $1")
            .bind(id)
            .fetch_one(self.dbcp.as_ref())
            .await
        {
            Ok(tag) => Ok(Some(tag)),
            Err(sqlx::Error::RowNotFound) => Ok(None),
            Err(err) => Err(AppError::from(err)),
        }
    }

    pub async fn list(&self, pagination_opts: Option<&Pagination>) -> AppResult<Vec<Tag>> {
        //
        let (offset, limit) = Pagination::from(pagination_opts).get_offset_limit();
        let query = format!("SELECT id, name, description FROM tags ORDER BY name LIMIT {limit} OFFSET {offset}");
        log::debug!("Listing tags w/ limit: {}, offset: {}.", limit, offset);

        sqlx::query_as::<_, Tag>(query.as_str()) // FYI: Binding (such as .bind(limit) didn't work, that's why the query.
            .fetch_all(self.dbcp.as_ref())
            .await
            .map(|res| AppResult::Ok(res))?
    }

    pub async fn update(&self, tag: Tag) -> AppResult<()> {
        //
        log::debug!("Updating tag: {:?}", tag);
        sqlx::query("UPDATE tags SET name=$1, description=$2 WHERE id = $3")
            .bind(tag.name)
            .bind(tag.description)
            .bind(tag.id.as_str())
            .execute(self.dbcp.as_ref())
            .await
            .map(|_| Ok(()))?
    }

    pub async fn add(&self, tag: Tag) -> AppResult<()> {
        //
        sqlx::query("INSERT INTO tags (id, name, description) VALUES ($1, $2, $3)")
            .bind(tag.id.as_str())
            .bind(tag.name)
            .bind(tag.description)
            .execute(self.dbcp.as_ref())
            .await
            .map(|_| Ok(()))?
    }

    pub async fn remove(&self, id: Id) -> AppResult<()> {
        //
        sqlx::query("DELETE FROM tags WHERE id = $1")
            .bind(id.as_str())
            .execute(self.dbcp.as_ref())
            .await
            .map(|_| Ok(()))?
    }
}

impl FromRow<'_, PgRow> for Tag {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(Tag::new(Id::new_from(row.get("id")), row.get("name"), row.get("description")))
    }
}
