use std::env;
use dotenv::dotenv;

pub struct Config {
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok(); 

        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL environment variable must be set!");

        Config {
            database_url
        }
    }
}
