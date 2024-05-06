pub mod local;

#[derive(Debug, serde::Deserialize)]
pub struct Config {
    pub database_url: String,
}
