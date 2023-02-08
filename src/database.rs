use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use rbatis::Rbatis;
use rbdc_mysql::driver::MysqlDriver;

pub static RB: Lazy<Rbatis> = Lazy::new(|| Rbatis::new());


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Post {
    pub id: Option<i32>,
    pub title: Option<String>,
    pub image_url: Option<String>,
    pub description: Option<String>
}

impl Post {
    pub fn new(title: Option<String>, description: Option<String>, image_url: Option<String>) -> Self {
        Self {
            id: None,
            title,
            description,
            image_url
        }
    } 
}

crud!(Post {}, "posts");

pub fn connect_db() {
    match RB.init(MysqlDriver {}, "mysql://root:@localhost/app-api-rust-salvo") {
        Ok(()) => {
            println!("Database is connecting...");
        },
        Err(err) => {
            println!("{:?}", err);
        } 
    }
}