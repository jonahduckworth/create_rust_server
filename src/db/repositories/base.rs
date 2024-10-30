use crate::errors::AppResult;
use diesel::PgConnection;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::db::models::base::BaseModel;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PaginationParams {
    pub page: i64,
    pub per_page: i64,
}

impl Default for PaginationParams {
    fn default() -> Self {
        Self {
            page: 1,
            per_page: 10,
        }
    }
}

pub trait BaseRepository<M: BaseModel> {
    fn find_by_id(conn: &mut PgConnection, id: Uuid) -> AppResult<M>;
    fn create(conn: &mut PgConnection, model: &M) -> AppResult<M>;
    fn update(conn: &mut PgConnection, id: Uuid, model: &M) -> AppResult<M>;
    fn soft_delete(conn: &mut PgConnection, id: Uuid) -> AppResult<M>;
    fn list(conn: &mut PgConnection, pagination: &PaginationParams) -> AppResult<Vec<M>>;
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub meta: PaginationMeta,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PaginationMeta {
    pub current_page: i64,
    pub per_page: i64,
    pub total_items: i64,
    pub total_pages: i64,
    pub has_next_page: bool,
    pub has_previous_page: bool,
}

impl<T> PaginatedResponse<T> {
    pub fn new(data: Vec<T>, total: i64, pagination: &PaginationParams) -> Self {
        let total_pages = (total as f64 / pagination.per_page as f64).ceil() as i64;
        Self {
            data,
            meta: PaginationMeta {
                current_page: pagination.page,
                per_page: pagination.per_page,
                total_items: total,
                total_pages,
                has_next_page: pagination.page < total_pages,
                has_previous_page: pagination.page > 1,
            },
        }
    }
}