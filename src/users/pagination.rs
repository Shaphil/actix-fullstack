use actix_web::web::Query;
use entity::user::{Entity, Entity as User, Model};
use sea_orm::{EntityTrait, QueryOrder, Select};
use serde::Deserialize;
use serde_json::{json, Value};

#[derive(Deserialize, Clone)]
pub struct PaginationQuery {
    page: Option<u64>,
    page_size: Option<u64>,
}

pub struct Pagination {
    pub(crate) query: Query<PaginationQuery>,
}

impl Pagination {
    pub fn paginate(&self) -> Select<Entity> {
        let page_size = self.query.page_size.unwrap_or(5);
        let page = self.query.page.unwrap_or(1);
        let offset = page * page_size;

        let mut user_pages = User::find()
            .order_by_asc(entity::user::Column::Id);

        sea_orm::QueryTrait::query(&mut user_pages)
            .offset(offset)
            .limit(page_size);

        user_pages
    }

    pub fn response(&self, users: Vec<Model>) -> Value {
        let page = self.query.page.unwrap_or(1);
        json!({
            "page": page,
            "users": users
        })
    }
}

