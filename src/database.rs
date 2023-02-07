use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use rbatis::Rbatis;
use rbdc_mysql::driver::MysqlDriver;

pub static RB: Lazy<Rbatis> = Lazy::new(|| Rbatis::new());


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Post {
    pub id: Option<i32>,
    pub title: Option<String>,
    pub description: Option<String>
}

crud!(Post {}, "posts");

pub fn connect_db() {
    RB.init(MysqlDriver {}, "url-database").unwrap();
}