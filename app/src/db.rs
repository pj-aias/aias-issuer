use rbatis::crud::CRUD;
use rbatis::rbatis::Rbatis;
use std::env;

use crate::rbatis::executor::Executor;

#[crud_table(table_name:"members")]
#[derive(Clone, Debug)]
pub struct Member {
    pub id: Option<u32>,
    pub phone_number: Option<String>,
    pub cert: Option<String>,
}

pub async fn init_db() -> Rbatis {
    let db_path = env::var("AIAS_DB_PATH").unwrap_or("sqlite://aias.db".to_string());

    println!("{}", db_path);

    let rb = Rbatis::new();
    rb.link(&db_path).await.unwrap();

    rb.exec(
        "CREATE TABLE IF NOT EXISTS 
            members(
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                phone_number TEXT,
                cert TEXT
            )",
        &vec![],
    )
    .await
    .expect("Error creating");

    rb
}

pub async fn save(rb: &Rbatis, member: &Member) {
    rb.save(member, &[]).await.expect("Error DB");
}
