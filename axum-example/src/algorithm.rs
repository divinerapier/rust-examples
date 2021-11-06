use axum::{extract::Extension, Json};
use sea_query::MySqlQueryBuilder;
use sqlx::{MySql, Pool};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CreateAlgorithmRequest {
    pub id: u64,
    pub name: String,
    pub location: String,
    pub image: u64,
}

#[derive(sea_query::Iden)]
pub enum Algorithm {
    Table,
    ID,
    Name,
    Location,
    Image,
}

pub async fn create(
    Extension(pool): Extension<Pool<MySql>>,
    Json(req): Json<CreateAlgorithmRequest>,
) -> String {
    let query = sea_query::Query::insert()
        .into_table(Algorithm::Table)
        .columns(vec![
            Algorithm::ID,
            Algorithm::Name,
            Algorithm::Location,
            Algorithm::Image,
        ])
        .values(vec![
            req.id.into(),
            req.name.into(),
            req.location.into(),
            req.image.into(),
        ])
        .unwrap()
        .to_string(MySqlQueryBuilder {});

    let id = sqlx::query(&query)
        .execute(&pool)
        .await
        .unwrap()
        .last_insert_id()
        .to_string();
    println!("id: {}", id);
    id
}
