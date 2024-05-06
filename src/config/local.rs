use super::Config;
use dotenv::dotenv;
use std::env;

pub fn init() -> Config {
    dotenv().ok();
    Config {
        database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
    }
}
