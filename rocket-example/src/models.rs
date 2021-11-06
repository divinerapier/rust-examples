// use crate::schema::posts::{self, *};

use crate::schema::posts;

#[derive(Queryable, serde::Serialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: i8,
}

#[derive(Insertable, serde::Deserialize, Clone)]
#[table_name = "posts"]
pub struct CreatePost {
    pub title: String,
    pub body: String,
    // https://docs.diesel.rs/diesel/sql_types/struct.TinyInt.html
    // impl AsExpression<TinyInt> for i8
    pub published: i8,
}
