pub mod platform;
pub mod tables;

use anyhow::Result;
use redb::{
    Database, ReadableDatabase, ReadableTable, ReadableTableMetadata, TableDefinition, TableHandle,
};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::models::AppConfig;

const ZSTD_LEVEL: i32 = 3;

pub struct Storage {
    db: Database,
}

impl Storage {
    pub fn open() -> Result<Self> {
        let dir = platform::data_dir();
        std::fs::create_dir_all(&dir)?;
        let path = dir.join("wherebus.redb");
        let db = Database::create(path)?;

        let write_txn = db.begin_write()?;
        {
            write_txn.open_table(tables::CONFIG_TABLE)?;
            write_txn.open_table(tables::STATIONS_TABLE)?;
            write_txn.open_table(tables::STATION_LINES_TABLE)?;
            write_txn.open_table(tables::LINE_DETAIL_TABLE)?;
            write_txn.open_table(tables::ALL_LINES_TABLE)?;
            write_txn.open_table(tables::TTL_TABLE)?;
        }
        write_txn.commit()?;

        Ok(Self { db })
    }

    pub fn get_cached<T: serde::de::DeserializeOwned>(
        &self,
        table: TableDefinition<&str, &[u8]>,
        key: &str,
        max_age: Duration,
    ) -> Option<T> {
        let read_txn = self.db.begin_read().ok()?;
        let ttl_table = read_txn.open_table(tables::TTL_TABLE).ok()?;
        let data_table = read_txn.open_table(table).ok()?;

        let ttl_key = format!("{}:{}", table.name(), key);
        let inserted_at = ttl_table.get(ttl_key.as_str()).ok()??.value();
        let now = SystemTime::now().duration_since(UNIX_EPOCH).ok()?.as_secs();
        if now.saturating_sub(inserted_at) > max_age.as_secs() {
            return None;
        }

        let bytes = data_table.get(key).ok()??.value().to_vec();
        let json = zstd::decode_all(bytes.as_slice())
            .or_else(|_| Ok::<_, std::io::Error>(bytes.clone()))
            .ok()?;
        serde_json::from_slice(&json).ok()
    }

    pub fn get_stale<T: serde::de::DeserializeOwned>(
        &self,
        table: TableDefinition<&str, &[u8]>,
        key: &str,
    ) -> Option<T> {
        let read_txn = self.db.begin_read().ok()?;
        let data_table = read_txn.open_table(table).ok()?;
        let bytes = data_table.get(key).ok()??.value().to_vec();
        let json = zstd::decode_all(bytes.as_slice())
            .or_else(|_| Ok::<_, std::io::Error>(bytes.clone()))
            .ok()?;
        serde_json::from_slice(&json).ok()
    }

    pub fn put_cached<T: serde::Serialize>(
        &self,
        table: TableDefinition<&str, &[u8]>,
        key: &str,
        value: &T,
    ) -> Result<()> {
        let json = serde_json::to_vec(value)?;
        let bytes = zstd::encode_all(json.as_slice(), ZSTD_LEVEL)?;
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        let ttl_key = format!("{}:{}", table.name(), key);

        let write_txn = self.db.begin_write()?;
        {
            let mut data_table = write_txn.open_table(table)?;
            data_table.insert(key, bytes.as_slice())?;
            let mut ttl_table = write_txn.open_table(tables::TTL_TABLE)?;
            ttl_table.insert(ttl_key.as_str(), now)?;
        }
        write_txn.commit()?;
        Ok(())
    }

    pub fn clear_cache(&self) -> Result<()> {
        let write_txn = self.db.begin_write()?;
        {
            write_txn.delete_table(tables::STATIONS_TABLE)?;
            write_txn.delete_table(tables::STATION_LINES_TABLE)?;
            write_txn.delete_table(tables::LINE_DETAIL_TABLE)?;
            write_txn.delete_table(tables::ALL_LINES_TABLE)?;
            write_txn.delete_table(tables::TTL_TABLE)?;
            write_txn.open_table(tables::STATIONS_TABLE)?;
            write_txn.open_table(tables::STATION_LINES_TABLE)?;
            write_txn.open_table(tables::LINE_DETAIL_TABLE)?;
            write_txn.open_table(tables::ALL_LINES_TABLE)?;
            write_txn.open_table(tables::TTL_TABLE)?;
        }
        write_txn.commit()?;
        Ok(())
    }

    pub fn load_config(&self) -> Option<AppConfig> {
        let read_txn = self.db.begin_read().ok()?;
        let table = read_txn.open_table(tables::CONFIG_TABLE).ok()?;
        let bytes = table.get("config").ok()??.value().to_vec();
        let json = zstd::decode_all(bytes.as_slice())
            .or_else(|_| Ok::<_, std::io::Error>(bytes.clone()))
            .ok()?;
        serde_json::from_slice(&json).ok()
    }

    pub fn save_config(&self, config: &AppConfig) -> Result<()> {
        let json = serde_json::to_vec(config)?;
        let bytes = zstd::encode_all(json.as_slice(), ZSTD_LEVEL)?;
        let write_txn = self.db.begin_write()?;
        {
            let mut table = write_txn.open_table(tables::CONFIG_TABLE)?;
            table.insert("config", bytes.as_slice())?;
        }
        write_txn.commit()?;
        Ok(())
    }

    pub fn table_entry_count(&self, table: TableDefinition<&str, &[u8]>) -> usize {
        let Ok(read_txn) = self.db.begin_read() else {
            return 0;
        };
        read_txn
            .open_table(table)
            .ok()
            .and_then(|tbl| tbl.len().ok())
            .map(|n| n as usize)
            .unwrap_or(0)
    }

    pub fn clear_table(&self, table: TableDefinition<'static, &str, &[u8]>) -> Result<()> {
        let prefix = format!("{}:", table.name());
        let write_txn = self.db.begin_write()?;
        {
            write_txn.delete_table(table)?;
            write_txn.open_table(table)?;
            let mut ttl = write_txn.open_table(tables::TTL_TABLE)?;
            let keys: Vec<String> = {
                let iter = ttl.iter()?;
                iter.filter_map(|entry| {
                    let entry = entry.ok()?;
                    let k = entry.0.value().to_string();
                    if k.starts_with(&prefix) {
                        Some(k)
                    } else {
                        None
                    }
                })
                .collect()
            };
            for k in keys {
                ttl.remove(k.as_str())?;
            }
        }
        write_txn.commit()?;
        Ok(())
    }
}
