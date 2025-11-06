use std::env;

use dotenvy::dotenv;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct EnvConfig {
    pub port: u16,
    pub env: String,
}

impl EnvConfig {
    pub fn from_env() -> Self {
        if let Err(err) = dotenv() {
            eprintln!("\x1B[31m[Erro] Sem arquivo .env: {err}\x1B[0m");
        }

        Self {
            port: env::var("PORT")
                .unwrap_or_else(|_| {
                    eprintln!("Porta no env inválida, usando 3000");
                    "3000".into()
                })
                .parse()
                .expect("Porta inválida"),
            env: env::var("env").unwrap_or_else(|_| {
                eprintln!("Sem ambiente no env, usando \"dev\"");
                "dev".into()
            }),
        }
    }
}
