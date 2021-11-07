use std::{
    borrow::{Borrow, BorrowMut, Cow},
    cell::RefCell,
};

use axum::{extract::Extension, response::IntoResponse, Json};
use sea_query::MySqlQueryBuilder;
use serde_json::json;
use sqlx::{MySql, Pool};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CreateAlgorithmRequest {
    pub name: String,
    pub location: String,
    pub image: u64,
}

pub type CreateAlgorithmResponse = String;

std::thread_local! {
    static SNOWFLAKE_GENERATOR: RefCell<snowflake::SnowflakeIdGenerator> =
        RefCell::new(snowflake::SnowflakeIdGenerator::new(0, std::thread::current().id().as_u64().get() as i32));
}

#[derive(sea_query::Iden)]
pub enum Algorithm {
    Table,
    ID,
    Name,
    DisplayName,
    Location,
    Image,
}

pub async fn create(
    Extension(pool): Extension<Pool<MySql>>,
    Json(req): Json<CreateAlgorithmRequest>,
) -> impl IntoResponse {
    let display_name = req.name.to_lowercase();

    let id = SNOWFLAKE_GENERATOR.with(|gen| gen.borrow_mut().generate());

    let query = sea_query::Query::insert()
        .into_table(Algorithm::Table)
        .columns(vec![
            Algorithm::ID,
            Algorithm::Name,
            Algorithm::DisplayName,
            Algorithm::Location,
            Algorithm::Image,
        ])
        .values(vec![
            id.into(),
            display_name.into(),
            req.name.into(),
            req.location.into(),
            req.image.into(),
        ])
        .unwrap()
        .to_string(MySqlQueryBuilder {});

    match sqlx::query(&query).execute(&pool).await {
        Ok(_) => Result::<_, ()>::Ok(Json(json!({
            "code": "000000",
            "data": id.to_string(),
        }))),
        Err(sqlx::Error::Database(e)) => {
            // code: Some("42S02"), number: 1146, message: "Table 'testing.algorithm' doesn't exist"
            // code: Some("23000"), number: 1062, message: "Duplicate entry 'alg-0000001' for key 'algorithm.unique_name'"
            let code = e.code();
            let message = e.message();
            println!(
                "database error: {:?}. code: {:?}, message: {}",
                e, code, message
            );
            match code {
                Some(Cow::Borrowed("23000")) => Ok(Json(json! ({
                    "code": "000001", // duplicate name
                    "message": "duplicate name",
                }))),
                Some(code) => Ok(Json(json! ({
                    "code": "000002",
                    "message": "unknown error",
                }))),
                None => Ok(Json(json! ({
                    "code": "000003",
                    "message": "unknown error",
                }))),
            }
        }
        Err(e) => {
            println!("other error: {:?}", e);
            Ok(Json(json! ({
                "code": "000001",
                "message": "",
            })))
        }
    }

    // println!("id: {}", id);
    // id.to_string()
}
