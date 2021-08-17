use rbatis::crud::CRUD;
use rbatis::rbatis::Rbatis;
use std::env;

use crate::rbatis::executor::Executor;

#[crud_table(table_name:"members")]
#[derive(Clone, Debug)]
pub struct Member {
    pub id: Option<u32>,
    pub phone_number: Option<String>,
    pub token: Option<String>,
}

pub async fn init_db() -> Rbatis {
    let db_path = env::var("AIAS_DB_PATH").unwrap_or("sqlite://aias.db".to_string());
    let log_path = env::var("AIAS_LOG_PATH").unwrap_or("requests.log".to_string());

    println!("{}", db_path);

    fast_log::init_log(&log_path, 1000, log::Level::Info, None, true).unwrap();

    let rb = Rbatis::new();
    rb.link(&db_path).await.unwrap();

    rb.exec(
        "CREATE TABLE IF NOT EXISTS 
            members(
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                phone_number TEXT,
                token TEXT
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
