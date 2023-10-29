/*
 * @Author: plucky
 * @Date: 2022-10-21 17:23:16
 * @LastEditTime: 2023-10-29 16:15:54
 * @Description: 
 */
fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests{
    #![allow(unused)]
    use anorm::{Crud, FromRow};

    // #[derive(sqlx::FromRow)]
    #[derive(Debug, Crud, FromRow)]
    #[anorm(rename = "users")] // rename table name
    struct Usera {
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

    impl Usera {
        pub fn new(id: u64, name: impl Into<String>, password: impl Into<String>) -> Self { 
            Self { id, name:name.into(), password: password.into(), addr: None } 
        }
    }

    #[cfg(feature = "mysql")]
    pub async fn get_pool() -> sqlx::Result<sqlx::MySqlPool> {
        sqlx::mysql::MySqlPoolOptions::new().connect("mysql://root:789789@192.168.1.199:3306/hello").await
    }

    #[cfg(feature = "postgres")]
    pub async fn get_pool() -> sqlx::Result<sqlx::PgPool> {
        sqlx::postgres::PgPoolOptions::new().connect("postgres://postgres:password@192.168.1.199:5432/postgres").await
    }

    #[tokio::test]
    async fn test_query() {
        let pool=get_pool().await.unwrap();
        
        let u = Usera::get(&pool, 1).await;
        println!("get {:?}", u);
        let u = Usera::get_by(&pool, "where id=1").await;
        println!("get_by {:?}", u);
        let u = Usera::query_by_name(&pool, "plucky".into()).await;
        println!("query_by_name {:?}", u);
        let u =Usera::query(&pool).await;
        println!("list {:?}",u);
    }

    #[tokio::test]
    async fn test_update(){
        let pool=get_pool().await.unwrap();

        let _u = Usera::new(2, "jack", "123456");
        
        let r = _u.update(&pool).await.unwrap();
        dbg!(r);
        let r = _u.update_by(&pool,"where id=2").await.unwrap();
        dbg!(r);
        let r =  _u.update_password(&pool).await.unwrap();
        dbg!(r);
        
    }

    #[tokio::test]
    async fn test_insert() {
        let pool=get_pool().await.unwrap();
        let _u = Usera::new(0, "lusy", "123456");
        let r =_u.insert(&pool).await.unwrap();
        println!("list: {:?}",r);
    }

    #[tokio::test]
    async fn test_delete() {
        let pool=get_pool().await.unwrap();
        
        let _u = Usera::new(10, "lusy", "123456");
        let r = _u.delete(&pool).await.unwrap();
        println!("delete: {:?}",r);
        let r =Usera::delete_by(&pool, "where name='leo'").await.unwrap();
        println!("delete: {:?}",r);
        let r =Usera::delete_by_name(&pool, "lusy".into()).await.unwrap();
        println!("delete: {:?}",r);
    }

    #[tokio::test]
    async fn test_insert_all() {
        let pool=get_pool().await.unwrap();
        let list = vec![Usera::new(0, "lusy3", "123456"),Usera::new(0, "lusy5", "123456")];
        let r =Usera::insert_all(&pool, list).await.unwrap();
        println!("list: {:?}",r);

    }

    
}