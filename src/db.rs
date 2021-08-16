use rbatis::rbatis::Rbatis;
use std::env;

#[crud_table]
#[derive(Clone, Debug)]
pub struct Member {
    pub id: Option<String>,
    pub name: Option<String>,
    pub xi: Option<String>,
    pub ax: Option<String>,
    pub gamma: Option<String>,
}

pub async fn init_db() -> Rbatis {
    let db_path = env::var("AIAS_DB_PATH").unwrap_or("sqlite3://aias.db".to_string());
    let log_path = env::var("AIAS_LOG_PATH").unwrap_or("requests.log".to_string());

    fast_log::init_log(&log_path, 1000, log::Level::Info, None, true).unwrap();

    let rb = Rbatis::new();
    rb.link(&db_path).await.unwrap();

    rb
}
