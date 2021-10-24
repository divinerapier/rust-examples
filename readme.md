# RUST Examples

## Diesel

### MySQL & ProgresQL

``` bash
sudo pacman -Syy libmysqlclient postgresql
```

``` bash
# 使用 mysql
cargo install diesel_cli

# 使用 postgre
cargo install diesel_cli --no-default-features --features postgres
```

### 初始化

``` bash
echo DATABASE_URL=mysql://root:rocket@127.0.0.1:3306/rocket > .env
# 或者
export DATABASE_URL=mysql://root:rocket@127.0.0.1:3306/rocket
diesel setup
```

### 执行 Migration 操作

创建 migration 需要使用的 `.sql` 文件:

``` bash
diesel migration generate create_posts
```

在 `up.sql` 中:

``` sql
CREATE TABLE posts (
    id          INT             PRIMARY KEY,
    title       VARCHAR(255)    NOT NULL,
    body        TEXT            NOT NULL,
    published   TINYINT         NOT NULL DEFAULT 0
)
```

在 `down.sql` 中:

``` sql
DROP TABLE posts
```

执行 migration:

``` bash
diesel migration run
```

回退最近操作:

``` bash
diesel migration revert
```

每次执行 `migration run` 后，会在数据库的 `__diesel_schema_migrations` 表中增加一条记录。

执行完成后，在 `main.rs` / `lib.rs` / `mod.rs` 中添加

``` rust
#[macro_use]
extern crate diesel;

mod schema;
```

在 `Cargo.toml` 中添加依赖:

``` toml
diesel = { version = "1.4.4", features = ["mysql", "r2d2", "numeric"] }
```

## Rocket 集成 Diesel

`Rocket` 框架中集成了若干数据库驱动或 `ORM`，其对应关系如下:

|Kind| Driver| Version| Poolable Type| Feature|
|:---|:-------|:-------|:----------------|:--------|
|MySQL| Diesel| 1| diesel::MysqlConnection| diesel_mysql_pool|
|Postgres| Diesel| 1| diesel::PgConnection| diesel_postgres_pool|
|Postgres| Rust-Postgres| 0.19| postgres::Client| postgres_pool|
|Sqlite| Diesel| 1| diesel::SqliteConnection| diesel_sqlite_pool|
|Sqlite| Rusqlite| 0.24| rusqlite::Connection| sqlite_pool|
|Memcache| memcache| 0.15| memcache::Client| memcache_pool|

根据实际使用 **Kind** 与 **Driver** 选择相应的 `Feature`，例如:

``` toml
[dependencies.rocket_sync_db_pools]
version = "0.1.0-rc.1"
default-features = false
features = ["diesel_mysql_pool"]
```

### 连接数据库

首先，在配置文件 `Rocket.toml` 中增加全局数据库连接配置，或环境相关数据库连接，例如，如下全局数据库配置:

``` toml
[global.databases]
posts = { url = "mysql://root:rocket@127.0.0.1/rocket" }
```

之后，通过 `rocket_sync_db_pools` 定义数据库连接类型:

``` rust
use rocket_sync_db_pools::database;

#[database("posts")]
struct PostsDbConn(diesel::MysqlConnection);
```

**#[database("posts")]** 中的 `posts` 对应配置文件中的 `global.databases.posts`。

然后，修改构建 `Rocket` 的代码:

``` rust
 rocket::build()
        .attach(PostsDbConn::fairing())
```

如此，即完成在 `Rocket` 中集成数据库连接。

最后，在需要使用数据库连接的路由处理函数的参数中添加数据库连接:

``` rust
#[post("/")]
async fn create_post(pool: PostsDbConn) {}
```

### 操作数据库

#### Insert

在 `models.rs` 中定义插入操作所需的数据结构:

``` rust
use crate::schema::posts;

#[derive(Insertable, serde::Deserialize, Clone)]
#[table_name = "posts"]
pub struct CreatePost {
    pub title: String,
    pub body: String,
    // https://docs.diesel.rs/diesel/sql_types/struct.TinyInt.html
    // impl AsExpression<TinyInt> for i8
    pub published: i8,
}
```

* `Insertable`: 该类型可用于 `insert_into` 操作
* `#[table_name = "posts"]`: 对应的数据库表名称
* `published: i8`: TinyInt 对应 i8 类型，参考注释中的文档，要求实现 `trait AsExpression`

##### 代码

``` rust
// format: 接收 JSON 格式输入
// data:   <> 中的字符串表示分序列之后的变量
#[post("/", format = "json", data = "<body>")]
async fn create_post(pool: PostsDbConn, body: Json<models::CreatePost>) {
    // 调用 execute 函数
    use diesel::RunQueryDsl;

    let crate_post = body.clone();

    pool.run(move |conn| {
        diesel::insert_into(schema::posts::table)
            .values(crate_post)
            // 或者如下，这里要求传入 Insertable<T>
            // .values(&*body)
            .execute(conn)
    })
    .await
    .unwrap();
}
```

* `#[post("/", format = "json", data = "<body>")]`: `API` 接收 `POST` 请求，数据格式为 `json`，使用参数 **body** 接收反序列化后的请求体
* `use diesel::RunQueryDsl;`: 包含 `execute` 函数的 `Trait`

## 参考

* [Diesel](https://diesel.rs/guides/getting-started)
* [Rocket Database](https://rocket.rs/v0.5-rc/guide/state/#databases)
* [Rocket JSON Request](https://rocket.rs/v0.5-rc/guide/requests/#json)
