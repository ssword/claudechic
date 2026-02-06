use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub supabase_url: String,
    pub supabase_key: String,
    pub home_dir: PathBuf,
    pub sessions_dir: PathBuf,
    pub history_file: PathBuf,
}

impl Config {
    pub fn from_env() -> crate::Result<Self> {
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgresql://localhost/claudechic".to_string());

        let supabase_url = std::env::var("SUPABASE_URL")
            .unwrap_or_else(|_| String::new());

        let supabase_key = std::env::var("SUPABASE_KEY")
            .unwrap_or_else(|_| String::new());

        let home_dir = dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."));

        let sessions_dir = home_dir.join(".claude/sessions");
        let history_file = home_dir.join(".claude/history.jsonl");

        Ok(Self {
            database_url,
            supabase_url,
            supabase_key,
            home_dir,
            sessions_dir,
            history_file,
        })
    }
}
