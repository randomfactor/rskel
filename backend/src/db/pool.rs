use async_trait::async_trait;
use serde_json::Value;
use std::sync::Arc;
use surrealdb::engine::local::{Db as LocalDb, Mem, RocksDb};
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

use super::{Error, KVStore, Result};

#[derive(Clone)]
pub struct DbPool<T: surrealdb::Connection = Client> {
    inner: Arc<Surreal<T>>,
}

impl DbPool<Client> {
    pub async fn new(
        endpoint: &str,
        namespace: &str,
        database: &str,
        username: &str,
        password: &str,
    ) -> Result<Self> {
        let db = Surreal::new::<Ws>(endpoint).await?;
        db.signin(Root {
            username,
            password,
        })
        .await?;
        db.use_ns(namespace).use_db(database).await?;

        Ok(Self {
            inner: Arc::new(db),
        })
    }

}

impl DbPool<LocalDb> {
    pub async fn from_env() -> Result<Self> {
        let path = std::env::var("SURREALDB_PATH").unwrap_or_else(|_| "./data/surrealdb".to_string());
        let namespace = std::env::var("SURREALDB_NS").unwrap_or_else(|_| "rskel_ns".to_string());
        let database = std::env::var("SURREALDB_DB").unwrap_or_else(|_| "rskel_db".to_string());
        let username = std::env::var("SURREALDB_USER").unwrap_or_else(|_| "rskel_local".to_string());
        let password = std::env::var("SURREALDB_PASS").unwrap_or_else(|_| "local_only_duh".to_string());

        Self::new_local_rocksdb(&path, &namespace, &database, &username, &password).await
    }

    pub async fn new_in_memory(namespace: &str, database: &str) -> Result<Self> {
        let db = Surreal::new::<Mem>(()).await?;
        db.use_ns(namespace).use_db(database).await?;

        Ok(Self {
            inner: Arc::new(db),
        })
    }

    pub async fn new_local_rocksdb(
        path: &str,
        namespace: &str,
        database: &str,
        username: &str,
        password: &str,
    ) -> Result<Self> {
        let db = Surreal::new::<RocksDb>(path).await?;
        db.signin(Root {
            username,
            password,
        })
        .await?;
        db.use_ns(namespace).use_db(database).await?;

        Ok(Self {
            inner: Arc::new(db),
        })
    }
}

#[async_trait]
impl<T: surrealdb::Connection> KVStore for DbPool<T> {
    async fn get(&self, key: &str) -> Result<Option<Value>> {
        if key.trim().is_empty() {
            return Err(Error::InvalidKey("key cannot be empty".to_string()));
        }

        let key = key.to_string();
        let mut response = self
            .inner
            .query("SELECT VALUE value FROM type::thing($key)")
            .bind(("key", key))
            .await?;

        let value: Option<Value> = response.take(0)?;
        Ok(value)
    }

    async fn set(&self, key: &str, value: Value) -> Result<()> {
        if key.trim().is_empty() {
            return Err(Error::InvalidKey("key cannot be empty".to_string()));
        }

        let key = key.to_string();
        self.inner
            .query("UPSERT type::thing($key) CONTENT { value: $value }")
            .bind(("key", key))
            .bind(("value", value))
            .await?;

        Ok(())
    }

    async fn increment(&self, key: &str, delta: i64) -> Result<i64> {
        if key.trim().is_empty() {
            return Err(Error::InvalidKey("key cannot be empty".to_string()));
        }

        let key = key.to_string();
        let mut response = self
            .inner
            .query("UPDATE type::thing($key) SET value = value + $delta RETURN VALUE value")
            .bind(("key", key))
            .bind(("delta", delta))
            .await?;

        let value: Option<Value> = response.take(0)?;
        Ok(match value {
            Some(Value::Number(number)) => number.as_i64().unwrap_or(0),
            Some(Value::String(text)) => text.parse::<i64>().unwrap_or(0),
            _ => 0,
        })
    }
}
