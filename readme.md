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
diesel = { version = "1.4.4", features = ["postgres"] }
```

## 参考

* [Diesel](https://diesel.rs/guides/getting-started)
