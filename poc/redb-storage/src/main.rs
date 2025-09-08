use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use redb::{Database, ReadableTable, ReadableTableMetadata, TableDefinition};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::time::{Duration, Instant};
use tracing::{info, warn};
use uuid::Uuid;

// Table definitions for different entity types
const USERS_TABLE: TableDefinition<&str, &[u8]> = TableDefinition::new("users");
const SESSIONS_TABLE: TableDefinition<&str, &[u8]> = TableDefinition::new("sessions");
const PLAYER_DATA_TABLE: TableDefinition<&str, &[u8]> = TableDefinition::new("player_data");
const BLOCKCHAIN_EVENTS_TABLE: TableDefinition<u64, &[u8]> = TableDefinition::new("blockchain_events");

// Data structures for testing
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub provider: String,
    pub created_at: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub ip_address: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlayerData {
    pub user_id: Uuid,
    pub level: u32,
    pub experience: u64,
    pub inventory: Vec<String>,
    pub last_played: DateTime<Utc>,
    pub stats: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BlockchainEvent {
    pub block_number: u64,
    pub transaction_hash: String,
    pub event_type: String,
    pub contract_address: String,
    pub data: serde_json::Value,
    pub timestamp: DateTime<Utc>,
}

/// RedbStorage provides a high-level interface for database operations
pub struct RedbStorage {
    db: Database,
}

impl RedbStorage {
    /// Create a new RedbStorage instance
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let db = Database::create(&path)
            .with_context(|| format!("Failed to create database at {:?}", path.as_ref()))?;
        
        // Create tables if they don't exist
        let write_txn = db.begin_write()?;
        {
            let _ = write_txn.open_table(USERS_TABLE)?;
            let _ = write_txn.open_table(SESSIONS_TABLE)?;
            let _ = write_txn.open_table(PLAYER_DATA_TABLE)?;
            let _ = write_txn.open_table(BLOCKCHAIN_EVENTS_TABLE)?;
        }
        write_txn.commit()?;
        
        info!("RedbStorage initialized successfully");
        
        Ok(Self { db })
    }
    
    // User operations
    pub fn create_user(&self, user: &User) -> Result<()> {
        let write_txn = self.db.begin_write()?;
        {
            let mut table = write_txn.open_table(USERS_TABLE)?;
            let key = user.id.to_string();
            let value = serde_json::to_vec(user)?;
            table.insert(key.as_str(), value.as_slice())?;
        }
        write_txn.commit()?;
        info!("User created: {}", user.id);
        Ok(())
    }
    
    pub fn get_user(&self, user_id: &Uuid) -> Result<Option<User>> {
        let read_txn = self.db.begin_read()?;
        let table = read_txn.open_table(USERS_TABLE)?;
        
        let key = user_id.to_string();
        if let Some(value) = table.get(key.as_str())? {
            let user: User = serde_json::from_slice(value.value())?;
            return Ok(Some(user));
        }
        
        Ok(None)
    }
    
    pub fn update_user_last_login(&self, user_id: &Uuid, timestamp: DateTime<Utc>) -> Result<()> {
        let write_txn = self.db.begin_write()?;
        {
            let mut table = write_txn.open_table(USERS_TABLE)?;
            let key = user_id.to_string();
            
            // First, get the user data
            let user_data = if let Some(value) = table.get(key.as_str())? {
                let mut user: User = serde_json::from_slice(value.value())?;
                user.last_login = Some(timestamp);
                serde_json::to_vec(&user)?
            } else {
                return Ok(()); // User not found
            };
            
            // Then update it
            table.insert(key.as_str(), user_data.as_slice())?;
        }
        write_txn.commit()?;
        Ok(())
    }
    
    // Session operations
    pub fn create_session(&self, session: &Session) -> Result<()> {
        let write_txn = self.db.begin_write()?;
        {
            let mut table = write_txn.open_table(SESSIONS_TABLE)?;
            let key = session.token.clone();
            let value = serde_json::to_vec(session)?;
            table.insert(key.as_str(), value.as_slice())?;
        }
        write_txn.commit()?;
        Ok(())
    }
    
    pub fn get_session(&self, token: &str) -> Result<Option<Session>> {
        let read_txn = self.db.begin_read()?;
        let table = read_txn.open_table(SESSIONS_TABLE)?;
        
        if let Some(value) = table.get(token)? {
            let session: Session = serde_json::from_slice(value.value())?;
            
            // Check if session has expired
            if session.expires_at > Utc::now() {
                return Ok(Some(session));
            } else {
                warn!("Session expired: {}", token);
                // Note: In production, we'd clean up expired sessions
            }
        }
        
        Ok(None)
    }
    
    // Player data operations
    pub fn save_player_data(&self, player_data: &PlayerData) -> Result<()> {
        let write_txn = self.db.begin_write()?;
        {
            let mut table = write_txn.open_table(PLAYER_DATA_TABLE)?;
            let key = player_data.user_id.to_string();
            let value = serde_json::to_vec(player_data)?;
            table.insert(key.as_str(), value.as_slice())?;
        }
        write_txn.commit()?;
        Ok(())
    }
    
    pub fn get_player_data(&self, user_id: &Uuid) -> Result<Option<PlayerData>> {
        let read_txn = self.db.begin_read()?;
        let table = read_txn.open_table(PLAYER_DATA_TABLE)?;
        
        let key = user_id.to_string();
        if let Some(value) = table.get(key.as_str())? {
            let player_data: PlayerData = serde_json::from_slice(value.value())?;
            return Ok(Some(player_data));
        }
        
        Ok(None)
    }
    
    // Blockchain event operations
    pub fn store_blockchain_event(&self, event: &BlockchainEvent) -> Result<()> {
        let write_txn = self.db.begin_write()?;
        {
            let mut table = write_txn.open_table(BLOCKCHAIN_EVENTS_TABLE)?;
            let value = serde_json::to_vec(event)?;
            table.insert(&event.block_number, value.as_slice())?;
        }
        write_txn.commit()?;
        Ok(())
    }
    
    pub fn get_events_by_block_range(&self, from_block: u64, to_block: u64) -> Result<Vec<BlockchainEvent>> {
        let read_txn = self.db.begin_read()?;
        let table = read_txn.open_table(BLOCKCHAIN_EVENTS_TABLE)?;
        
        let mut events = Vec::new();
        
        // Range query for blocks
        let range = table.range(from_block..=to_block)?;
        for result in range {
            let (_, value) = result?;
            let event: BlockchainEvent = serde_json::from_slice(value.value())?;
            events.push(event);
        }
        
        Ok(events)
    }
    
    // Performance and maintenance operations
    pub fn get_database_stats(&self) -> Result<DatabaseStats> {
        let read_txn = self.db.begin_read()?;
        
        let users_table = read_txn.open_table(USERS_TABLE)?;
        let sessions_table = read_txn.open_table(SESSIONS_TABLE)?;
        let player_data_table = read_txn.open_table(PLAYER_DATA_TABLE)?;
        let blockchain_events_table = read_txn.open_table(BLOCKCHAIN_EVENTS_TABLE)?;
        
        let stats = DatabaseStats {
            users_count: users_table.len()?,
            sessions_count: sessions_table.len()?,
            player_data_count: player_data_table.len()?,
            blockchain_events_count: blockchain_events_table.len()?,
        };
        
        Ok(stats)
    }
    
    pub fn compact(&self) -> Result<()> {
        // Redb handles compaction automatically, but we can provide explicit compaction
        // This is mainly for demonstration purposes
        info!("Database compaction requested (handled automatically by redb)");
        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct DatabaseStats {
    pub users_count: u64,
    pub sessions_count: u64,
    pub player_data_count: u64,
    pub blockchain_events_count: u64,
}

// Performance testing and validation
pub async fn run_performance_tests(storage: &RedbStorage) -> Result<PerformanceReport> {
    info!("Starting Redb performance tests...");
    let mut report = PerformanceReport::default();
    
    // Test 1: User CRUD operations
    let user_test_start = Instant::now();
    let test_users = create_test_users(1000)?;
    
    // Create users
    let create_start = Instant::now();
    for user in &test_users {
        storage.create_user(user)?;
    }
    let create_time = create_start.elapsed();
    
    // Read users
    let read_start = Instant::now();
    for user in &test_users {
        let _ = storage.get_user(&user.id)?;
    }
    let read_time = read_start.elapsed();
    
    report.user_create_time = create_time;
    report.user_read_time = read_time;
    report.users_tested = test_users.len();
    
    // Test 2: Session operations
    let sessions_start = Instant::now();
    let test_sessions = create_test_sessions(&test_users, 5000)?;
    
    for session in &test_sessions {
        storage.create_session(session)?;
    }
    
    let session_lookup_start = Instant::now();
    for session in test_sessions.iter().take(1000) {
        let _ = storage.get_session(&session.token)?;
    }
    let session_lookup_time = session_lookup_start.elapsed();
    
    report.session_operations_time = sessions_start.elapsed();
    report.session_lookup_time = session_lookup_time;
    report.sessions_tested = test_sessions.len();
    
    // Test 3: Blockchain event storage and range queries
    let blockchain_start = Instant::now();
    let test_events = create_test_blockchain_events(10000)?;
    
    for event in &test_events {
        storage.store_blockchain_event(event)?;
    }
    
    let range_query_start = Instant::now();
    let _events = storage.get_events_by_block_range(1000, 2000)?;
    let range_query_time = range_query_start.elapsed();
    
    report.blockchain_storage_time = blockchain_start.elapsed();
    report.range_query_time = range_query_time;
    report.blockchain_events_tested = test_events.len();
    
    // Final stats
    report.total_test_time = user_test_start.elapsed();
    report.database_stats = storage.get_database_stats()?;
    
    info!("Redb performance tests completed");
    Ok(report)
}

#[derive(Debug, Default)]
pub struct PerformanceReport {
    pub user_create_time: Duration,
    pub user_read_time: Duration,
    pub users_tested: usize,
    pub session_operations_time: Duration,
    pub session_lookup_time: Duration,
    pub sessions_tested: usize,
    pub blockchain_storage_time: Duration,
    pub range_query_time: Duration,
    pub blockchain_events_tested: usize,
    pub total_test_time: Duration,
    pub database_stats: DatabaseStats,
}

impl PerformanceReport {
    pub fn print_summary(&self) {
        println!("\n🚀 REDB STORAGE PoC - PERFORMANCE REPORT");
        println!("==========================================");
        
        println!("\n📊 User Operations:");
        println!("  • Create {} users: {:?} ({:.2} ops/sec)", 
                 self.users_tested, 
                 self.user_create_time,
                 self.users_tested as f64 / self.user_create_time.as_secs_f64());
        println!("  • Read {} users: {:?} ({:.2} ops/sec)", 
                 self.users_tested, 
                 self.user_read_time,
                 self.users_tested as f64 / self.user_read_time.as_secs_f64());
        
        println!("\n🔐 Session Operations:");
        println!("  • Create {} sessions: {:?} ({:.2} ops/sec)", 
                 self.sessions_tested, 
                 self.session_operations_time,
                 self.sessions_tested as f64 / self.session_operations_time.as_secs_f64());
        println!("  • Lookup 1000 sessions: {:?} ({:.2} ops/sec)", 
                 self.session_lookup_time,
                 1000.0 / self.session_lookup_time.as_secs_f64());
        
        println!("\n🧱 Blockchain Operations:");
        println!("  • Store {} events: {:?} ({:.2} ops/sec)", 
                 self.blockchain_events_tested, 
                 self.blockchain_storage_time,
                 self.blockchain_events_tested as f64 / self.blockchain_storage_time.as_secs_f64());
        println!("  • Range query (1000 blocks): {:?}", self.range_query_time);
        
        println!("\n📈 Database Statistics:");
        println!("  • Total users: {}", self.database_stats.users_count);
        println!("  • Total sessions: {}", self.database_stats.sessions_count);
        println!("  • Total player data: {}", self.database_stats.player_data_count);
        println!("  • Total blockchain events: {}", self.database_stats.blockchain_events_count);
        
        println!("\n⏱️  Total test time: {:?}", self.total_test_time);
        println!("==========================================");
    }
}

// Helper functions for creating test data
fn create_test_users(count: usize) -> Result<Vec<User>> {
    let mut users = Vec::with_capacity(count);
    
    for i in 0..count {
        users.push(User {
            id: Uuid::new_v4(),
            email: format!("user{}@bunkerverse.com", i),
            provider: if i % 2 == 0 { "google".to_string() } else { "github".to_string() },
            created_at: Utc::now(),
            last_login: None,
            metadata: serde_json::json!({
                "preferences": {
                    "theme": "dark",
                    "language": "en"
                },
                "tier": if i % 10 == 0 { "premium" } else { "free" }
            }),
        });
    }
    
    Ok(users)
}

fn create_test_sessions(users: &[User], count: usize) -> Result<Vec<Session>> {
    let mut sessions = Vec::with_capacity(count);
    
    for i in 0..count {
        let user = &users[i % users.len()];
        sessions.push(Session {
            id: Uuid::new_v4(),
            user_id: user.id,
            token: format!("bunkerverse_token_{}", Uuid::new_v4()),
            created_at: Utc::now(),
            expires_at: Utc::now() + chrono::Duration::hours(24),
            ip_address: Some(format!("192.168.1.{}", i % 255)),
        });
    }
    
    Ok(sessions)
}

fn create_test_blockchain_events(count: usize) -> Result<Vec<BlockchainEvent>> {
    let mut events = Vec::with_capacity(count);
    
    for i in 0..count {
        events.push(BlockchainEvent {
            block_number: (i as u64) + 1000,
            transaction_hash: format!("0x{:064x}", i),
            event_type: if i % 3 == 0 { "NFTMinted".to_string() } 
                       else if i % 3 == 1 { "NFTTransferred".to_string() }
                       else { "NFTBurned".to_string() },
            contract_address: "0x742d35Cc6634C0532925a3b8D0B57FA6c98A6B50".to_string(),
            data: serde_json::json!({
                "token_id": i,
                "from": if i % 3 == 0 { "0x0000000000000000000000000000000000000000".to_string() } 
                       else { format!("0x{:040x}", i - 1) },
                "to": format!("0x{:040x}", i),
                "value": i * 1000
            }),
            timestamp: Utc::now(),
        });
    }
    
    Ok(events)
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    info!("🚀 Starting Redb Storage PoC");
    
    // Create test database
    let db_path = "test_bunkerverse.redb";
    let storage = RedbStorage::new(db_path)?;
    
    // Run comprehensive performance tests
    let report = run_performance_tests(&storage).await?;
    
    // Print results
    report.print_summary();
    
    // Security and integration tests
    info!("Running security and integration validation...");
    
    // Test concurrent access
    let concurrent_test_start = Instant::now();
    let storage = std::sync::Arc::new(storage);
    let mut handles = Vec::new();
    
    for i in 0..10 {
        let storage_clone = storage.clone();
        let handle = tokio::spawn(async move {
            let user = User {
                id: Uuid::new_v4(),
                email: format!("concurrent_user_{}@test.com", i),
                provider: "test".to_string(),
                created_at: Utc::now(),
                last_login: None,
                metadata: serde_json::json!({"test": true}),
            };
            
            storage_clone.create_user(&user).unwrap();
            let retrieved = storage_clone.get_user(&user.id).unwrap();
            assert!(retrieved.is_some());
        });
        handles.push(handle);
    }
    
    // Wait for all concurrent operations
    for handle in handles {
        handle.await?;
    }
    
    let concurrent_time = concurrent_test_start.elapsed();
    info!("✅ Concurrent access test passed in {:?}", concurrent_time);
    
    // Clean up
    std::fs::remove_file(db_path).unwrap_or_else(|e| warn!("Failed to cleanup test db: {}", e));
    
    info!("🎉 Redb Storage PoC completed successfully!");
    
    Ok(())
}