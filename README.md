# Rust derive macro implement Create, Read, Update, and Delete (CRUD) methods base on sqlx.

<div align="center">
  <!-- Version -->
  <a href="https://crates.io/crates/anorm">
    <img src="https://img.shields.io/crates/v/anorm.svg?style=flat-square"
    alt="Crates.io version" />
  </a>
  
  <!-- Docs -->
  <a href="https://docs.rs/anorm">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="docs.rs docs" />
  </a>
  <!-- Downloads -->
  <a href="https://crates.io/crates/anorm">
    <img src="https://img.shields.io/crates/d/anorm.svg?style=flat-square"
      alt="Download" />
  </a>
</div>

## Use
 adding the following to your project's Cargo.toml:
 ```toml
[dependencies]
anorm = { virsion = "0.1", features = ["mysql"] }
sqlx = { version = "0.7", features = ["mysql","runtime-tokio-native-tls"] }

[dev-dependencies]
tokio = { version = "1", features = ["macros"] }

 ```
 
 * features: mysql, postgres, sqlite, mssql

## Examples
```rust
// #[derive(sqlx::FromRow)]
#[derive(Debug, Crud, FromRow)]
#[anorm(rename = "users")] // rename table name
struct User {
    // anorm(id) // default first field is primary key
    #[anorm(seq)] // sequence field, insert will ignore this field
    pub id: u64,
    #[anorm(rename = "name")] // rename field name
    #[anorm(by)] // generate query_by_field,update_by_field,delete_by_field
    pub name: String,
    #[anorm(update)] // generate method update_xxx. 
    pub password: String,
    #[anorm(skip)] // ignore field
    pub addr: Option<String>,
    
    // #[sqlx(skip)]
    // pub age: i32,
}


pub async fn get_pool() -> Result<MySqlPool> {
    MySqlPoolOptions::new()
        .connect("mysql://root:password@192.168.1.199:3306/hello").await
}

#[tokio::test]
async fn test_query() {
    let pool=get_pool().await.unwrap();
        
    let u = User::get(&pool, 1).await;
    println!("get {:?}", u);
    let u = User::get_by(&pool, "where id=1").await;
    println!("get_by {:?}", u);
    let u = User::query_by_name(&pool, "plucky".into()).await;
    println!("query_by_name {:?}", u);
    let u =User::query(&pool).await;
    println!("list {:?}",u);
    
}

```
### #[derive(Crud)]
```
`#[derive(FromRow)]`
generate method: get, get_by, query, query_by, update, delete, insert, insert_all.

attributes:

`#[anorm(id)]`
default first field is primary key or set.

`#[anorm(seq)]`
sequence field, auto increment. insert will skip this field.

`#[anorm(rename="name")]`
rename table name or field name. 
default table name by struct name to_table_case: UserDetail => user_detail. 
default field name by field name to_snake_case: UserDetail => user_detail. 

`#[anorm(skip)]`
ignore field.

`#[anorm(update)]`
generate method update_xxx. 

`#[anorm(by)]`
generate query_by_field,update_by_field,delete_by_field.
```
### #[derive(FromRow)]
```
`#[derive(FromRow)]`
impl sqlx::FromRow trait.

or use `#[derive(sqlx::FromRow)]` macro or impl `sqlx::FromRow` trait.

if using sqlx::FromRow, change `#[anorm(skip)]` to `#[sqlx(skip)]` .

```


