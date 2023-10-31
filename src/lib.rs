

/*
 * @Author: plucky
 * @Date: 2022-10-21 16:53:21
 * @LastEditTime: 2023-10-31 18:55:45
 * @Description: 
 */

//! Derive macro for sqlx to implement Create, Read, Update, and Delete (CRUD) methods.
//! # Use
//! adding the following to your project's Cargo.toml:
//! ```toml
//! [dependencies]
//! anorm = { virsion = "0.1", features = ["mysql"] }
//! sqlx = { version = "0.7", features = ["mysql"] }
//! ```
//! features: mysql, postgres, sqlite, mssql
//! 
//! # Examples
//! ```
//! use anorm::{Crud, FromRow};
//! 
//! #[derive(Debug, Crud, FromRow)]
//! #[anorm(rename = "users")] // rename table name
//! pub struct User {
//!     // anorm(id) // default first field is primary key
//!      #[anorm(seq)] // sequence field, insert will ignore this field
//!      pub id: u64,
//!      #[anorm(rename = "name")] // rename field name
//!      #[anorm(by)] // generate query_by_field,update_by_field,delete_by_field
//!      pub name: String,
//!      #[anorm(update)] // generate method update_xxx. 
//!      pub password: String,
//!      #[anorm(skip)] // ignore field
//!       pub addr: Option<String>,
//! }
//! 
//! 
//! let u = User::get(&pool, 1).await;
//! println!("get {:?}", u);
//! let u = User::get_by(&pool, "where id=?", sql_args!(1)).await;
//! println!("get_by {:?}", u);
//! ```

pub use anorm_macros::Crud;
pub use anorm_macros::FromRow;


/// sqlx::query_as
/// ``` no_run
/// query_as!(User, "select * from users where name = ?", name).fetch_one(&pool).await
/// ```
#[macro_export]
macro_rules! query_as (
    ($out_struct:path, $query:expr) => ( {
        sqlx::query_as::<_, $out_struct>($query)
    });
    ($out_struct:path, $query:expr, $($args:expr),*) => ( {
        sqlx::query_as::<_, $out_struct>($query)
        $(.bind($args))*
    })
);

/// sqlx::query
/// # Examples
/// 
/// ``` no_run
/// query!("insert into users (name, password) values (?,?)", name, password).execute(&pool).await
/// ```
#[macro_export]
macro_rules! query (
    ($query:expr) => ( {
        sqlx::query($query)
    });
    ($query:expr, $($args:expr),*) => ( {
        sqlx::query($query)
        $(.bind($args))*
    })
);


/// # Examples
///
/// ```no_run
/// 
/// let args = sql_args!(&name, age);
/// ```
#[macro_export]
macro_rules! sql_args {

    // ($sql:expr) => {
    //     sql_args!($sql,);
    // };
    
    ($($args:expr),*) => {{
        use sqlx::Arguments;
        #[cfg(feature = "mysql")]{
            let mut sqlargs = sqlx::mysql::MySqlArguments::default();
            $(sqlargs.add($args);)*
            sqlargs
        }
        #[cfg(feature = "postgres")]{
            let mut sqlargs = sqlx::postgres::PgArguments::default();
            $(sqlargs.add($args);)*
            sqlargs
        }
        #[cfg(feature = "mssql")]{
            let mut sqlargs = sqlx::mysql::MySqlArguments::default();
            $(sqlargs.add($args);)*
            sqlargs
        }
        #[cfg(feature = "sqlite")]{
            let mut sqlargs = sqlx::sqlite::SqliteArguments::default();
            $(sqlargs.add($args);)*
            sqlargs
        }
        // let mut sqlargs = sqlx::any::AnyArguments::default();
       
    }};
}