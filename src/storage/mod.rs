use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};
use crate::response::Response;
use crate::error::StorageError;

pub mod converter;

pub struct Storage {
    data: Arc<RwLock<HashMap<String, StorageValue>>>,
    stats: Arc<RwLock<StorageStats>>,
    cleanup_handle: Option<std::thread::JoinHandle<()>>,
    stop_cleanup: Arc<std::sync::atomic::AtomicBool>,
}

#[derive(Debug, Clone)]
struct StorageValue {
    value: String,
    expires_at: Option<Instant>,
    created_at: Instant,
}

#[derive(Debug, Default, Clone)]
pub struct StorageStats {
    total_keys: usize,
}

impl Storage {
    pub fn new() -> Self {
        let data = Arc::new(RwLock::new(HashMap::new()));
        let stats = Arc::new(RwLock::new(StorageStats::default()));
        let stop_cleanup = Arc::new(std::sync::atomic::AtomicBool::new(false));
        
        let cleanup_handle = Some(Self::start_cleanup_thread(
            Arc::clone(&stop_cleanup),
            Arc::clone(&data),
            Arc::clone(&stats),
        ));
        
        Self {
            data,
            stats,
            cleanup_handle,
            stop_cleanup,
        }
    }
    
    fn start_cleanup_thread(
        stop: Arc<std::sync::atomic::AtomicBool>,
        data: Arc<RwLock<HashMap<String, StorageValue>>>,
        stats: Arc<RwLock<StorageStats>>,
    ) -> std::thread::JoinHandle<()> {
        std::thread::spawn(move || {
            let cleanup_interval = Duration::from_secs(10);
            
            while !stop.load(std::sync::atomic::Ordering::Relaxed) {
                std::thread::sleep(cleanup_interval);

                if let Ok(mut data) = data.write() {
                    data.retain(|_, value| {
                        value.expires_at
                            .map(|exp| exp > Instant::now())
                            .unwrap_or(true)
                    });
                }

                if let Ok(mut stats) = stats.write() {
                    stats.total_keys = data.read().map(|d| d.len()).unwrap_or(0);
                }
            }
        })
    }
    
    pub fn set(&self, key: String, value: String) -> Result<Response, StorageError> {
        let mut data = self.data.write().map_err(|_| StorageError::LockPoisoned)?;
        
        data.insert(key, StorageValue {
            value,
            expires_at: None,
            created_at: Instant::now(),
        });

        if let Ok(mut stats) = self.stats.write() {
            stats.total_keys = data.len();
        }
        
        Ok(Response::ok())
    }
    
    pub fn set_ex(&self, key: String, value: String, ttl_secs: u64) -> Result<Response, StorageError> {
        let mut data = self.data.write().map_err(|_| StorageError::LockPoisoned)?;
        
        data.insert(key, StorageValue {
            value,
            expires_at: Some(Instant::now() + Duration::from_secs(ttl_secs)),
            created_at: Instant::now(),
        });
        
        if let Ok(mut stats) = self.stats.write() {
            stats.total_keys = data.len();
        }
        
        Ok(Response::ok())
    }
    
    pub fn get(&self, key: &str) -> Result<Response, StorageError> {
        let data = self.data.read().map_err(|_| StorageError::LockPoisoned)?;
        
        let response = if let Some(value) = data.get(key) {
            let is_expired = value.expires_at
                .map(|exp| exp <= Instant::now())
                .unwrap_or(false);
            
            if is_expired {
                Response::BulkString(None)
            } else {
                Response::BulkString(Some(value.value.clone()))
            }
        } else {
            Response::BulkString(None)
        };
        
        Ok(response)
    }
    
    pub fn delete(&self, key: &str) -> Result<Response, StorageError> {
        let mut data = self.data.write().map_err(|_| StorageError::LockPoisoned)?;
        
        let removed = data.remove(key).is_some();
        
        if let Ok(mut stats) = self.stats.write() {
            stats.total_keys = data.len();
        }
        
        Ok(Response::Integer(if removed { 1 } else { 0 }))
    }
    
    pub fn exists(&self, key: &str) -> Result<Response, StorageError> {
        let data = self.data.read().map_err(|_| StorageError::LockPoisoned)?;
        
        let exists = data.get(key)
            .map(|v| {
                v.expires_at
                    .map(|exp| exp > Instant::now())
                    .unwrap_or(true)
            })
            .unwrap_or(false);
        
        Ok(Response::Integer(if exists { 1 } else { 0 }))
    }
    
    pub fn keys(&self, pattern: Option<&str>) -> Result<Response, StorageError> {
        let data = self.data.read().map_err(|_| StorageError::LockPoisoned)?;
        
        let keys: Vec<String> = data.keys()
            .filter(|key| {
                if let Some(pattern) = pattern {
                    if pattern.contains('*') {
                        let pattern = pattern.replace('*', "");
                        key.contains(&pattern)
                    } else {
                        key.as_str() == pattern
                    }
                } else {
                    true
                }
            })
            .cloned()
            .collect();
        
        let responses: Vec<Response> = keys.into_iter()
            .map(|s| Response::BulkString(Some(s)))
            .collect();
        
        Ok(Response::Array(responses))
    }
    
    pub fn clear(&self) -> Result<Response, StorageError> {
        let mut data = self.data.write().map_err(|_| StorageError::LockPoisoned)?;
        data.clear();
        
        if let Ok(mut stats) = self.stats.write() {
            stats.total_keys = 0;
        }
        
        Ok(Response::ok())
    }
    
    pub fn info(&self) -> Result<Response, StorageError> {
        let stats = self.stats.read().map_err(|_| StorageError::LockPoisoned)?;
        
        let info = format!(
            "Stats:\r\n\
            total_keys: {}\r\n",
            stats.total_keys
        );
        
        Ok(Response::BulkString(Some(info)))
    }
}