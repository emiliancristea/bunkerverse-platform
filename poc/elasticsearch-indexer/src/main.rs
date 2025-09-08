use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::time::{Duration, Instant};
use tracing::{info, warn, error};

// Data structures for blockchain indexing
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BlockchainEvent {
    pub id: String,
    pub block_number: u64,
    pub transaction_hash: String,
    pub event_type: String,
    pub contract_address: String,
    pub from_address: Option<String>,
    pub to_address: Option<String>,
    pub token_id: Option<u64>,
    pub value: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub raw_data: Value,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserActivity {
    pub id: String,
    pub user_id: String,
    pub activity_type: String,
    pub description: String,
    pub metadata: Value,
    pub timestamp: DateTime<Utc>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NFTMetadata {
    pub id: String,
    pub token_id: u64,
    pub contract_address: String,
    pub name: String,
    pub description: String,
    pub image_url: String,
    pub attributes: Vec<NFTAttribute>,
    pub owner: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NFTAttribute {
    pub trait_type: String,
    pub value: Value,
    pub display_type: Option<String>,
}

/// Elasticsearch indexer for Bunkerverse Platform (HTTP client implementation)
pub struct ElasticsearchIndexer {
    client: Client,
    base_url: String,
    blockchain_index: String,
    activity_index: String,
    nft_index: String,
}

impl ElasticsearchIndexer {
    /// Create new Elasticsearch indexer
    pub async fn new(elasticsearch_url: &str) -> Result<Self> {
        info!("🔍 Connecting to Elasticsearch at: {}", elasticsearch_url);
        
        let client = Client::new();
        
        // Test connection
        let health_url = format!("{}/_cluster/health", elasticsearch_url);
        let response = client.get(&health_url).send().await;
        
        match response {
            Ok(resp) if resp.status().is_success() => {
                let health_info: Value = resp.json().await.context("Failed to parse health response")?;
                info!("✅ Connected to Elasticsearch cluster: {}", health_info["cluster_name"].as_str().unwrap_or("unknown"));
            }
            Ok(resp) => {
                warn!("⚠️  Elasticsearch health check returned status: {}", resp.status());
                return Err(anyhow::anyhow!("Elasticsearch health check failed"));
            }
            Err(e) => {
                error!("❌ Failed to connect to Elasticsearch: {}", e);
                return Err(e.into());
            }
        }
        
        let indexer = Self {
            client,
            base_url: elasticsearch_url.to_string(),
            blockchain_index: "bunkerverse-blockchain-events".to_string(),
            activity_index: "bunkerverse-user-activity".to_string(),
            nft_index: "bunkerverse-nft-metadata".to_string(),
        };
        
        // Initialize indices
        indexer.initialize_indices().await?;
        
        Ok(indexer)
    }
    
    /// Initialize Elasticsearch indices with proper mappings
    async fn initialize_indices(&self) -> Result<()> {
        info!("🏗️  Initializing Elasticsearch indices...");
        
        // Define index mappings
        let indices = vec![
            (&self.blockchain_index, self.blockchain_mapping()),
            (&self.activity_index, self.activity_mapping()),
            (&self.nft_index, self.nft_mapping()),
        ];
        
        for (index_name, mapping) in indices {
            // Delete existing index
            let delete_url = format!("{}/{}", self.base_url, index_name);
            let _ = self.client.delete(&delete_url).send().await;
            
            // Create new index with mapping
            let create_url = format!("{}/{}", self.base_url, index_name);
            let response = self.client.put(&create_url)
                .json(&mapping)
                .send()
                .await
                .context(format!("Failed to create index: {}", index_name))?;
            
            if response.status().is_success() {
                info!("📋 Created index: {}", index_name);
            } else {
                let error_body: Value = response.json().await.unwrap_or_else(|_| json!({}));
                warn!("⚠️  Index creation response for {}: {:?}", index_name, error_body);
            }
        }
        
        info!("✅ Elasticsearch indices initialized");
        Ok(())
    }
    
    fn blockchain_mapping(&self) -> Value {
        json!({
            "mappings": {
                "properties": {
                    "id": { "type": "keyword" },
                    "block_number": { "type": "long" },
                    "transaction_hash": { "type": "keyword" },
                    "event_type": { "type": "keyword" },
                    "contract_address": { "type": "keyword" },
                    "from_address": { "type": "keyword" },
                    "to_address": { "type": "keyword" },
                    "token_id": { "type": "long" },
                    "value": { "type": "keyword" },
                    "timestamp": { "type": "date" },
                    "raw_data": { "type": "object" }
                }
            },
            "settings": {
                "number_of_shards": 1,
                "number_of_replicas": 0,
                "index": {
                    "max_result_window": 100000
                }
            }
        })
    }
    
    fn activity_mapping(&self) -> Value {
        json!({
            "mappings": {
                "properties": {
                    "id": { "type": "keyword" },
                    "user_id": { "type": "keyword" },
                    "activity_type": { "type": "keyword" },
                    "description": { "type": "text", "analyzer": "standard" },
                    "metadata": { "type": "object" },
                    "timestamp": { "type": "date" },
                    "ip_address": { "type": "ip" },
                    "user_agent": { "type": "text", "index": false }
                }
            },
            "settings": {
                "number_of_shards": 1,
                "number_of_replicas": 0
            }
        })
    }
    
    fn nft_mapping(&self) -> Value {
        json!({
            "mappings": {
                "properties": {
                    "id": { "type": "keyword" },
                    "token_id": { "type": "long" },
                    "contract_address": { "type": "keyword" },
                    "name": { "type": "text", "analyzer": "standard" },
                    "description": { "type": "text", "analyzer": "standard" },
                    "image_url": { "type": "keyword" },
                    "attributes": {
                        "type": "nested",
                        "properties": {
                            "trait_type": { "type": "keyword" },
                            "value": { "type": "keyword" },
                            "display_type": { "type": "keyword" }
                        }
                    },
                    "owner": { "type": "keyword" },
                    "created_at": { "type": "date" },
                    "updated_at": { "type": "date" }
                }
            },
            "settings": {
                "number_of_shards": 1,
                "number_of_replicas": 0
            }
        })
    }
    
    /// Index documents using bulk API
    async fn bulk_index(&self, index_name: &str, documents: &[Value]) -> Result<IndexingStats> {
        let start_time = Instant::now();
        
        if documents.is_empty() {
            return Ok(IndexingStats::default());
        }
        
        info!("📝 Bulk indexing {} documents to {}...", documents.len(), index_name);
        
        // Build bulk request body
        let mut bulk_body = String::new();
        
        for doc in documents {
            // Add index action
            let action = json!({
                "index": {
                    "_index": index_name,
                    "_id": doc.get("id").unwrap_or(&json!("")).as_str().unwrap_or("")
                }
            });
            bulk_body.push_str(&serde_json::to_string(&action)?);
            bulk_body.push('\n');
            
            // Add document
            bulk_body.push_str(&serde_json::to_string(doc)?);
            bulk_body.push('\n');
        }
        
        let bulk_url = format!("{}/_bulk", self.base_url);
        let response = self.client.post(&bulk_url)
            .header("Content-Type", "application/x-ndjson")
            .body(bulk_body)
            .send()
            .await
            .context("Failed to execute bulk indexing")?;
        
        let response_body: Value = response.json().await?;
        
        let took = response_body["took"].as_u64().unwrap_or(0);
        let errors = response_body["errors"].as_bool().unwrap_or(false);
        let items = response_body["items"].as_array().map(|arr| arr.len()).unwrap_or(0);
        
        let stats = IndexingStats {
            documents_indexed: items,
            took_ms: took,
            has_errors: errors,
            processing_time: start_time.elapsed(),
        };
        
        if errors {
            warn!("⚠️  Some documents failed to index in {}", index_name);
        } else {
            info!("✅ Indexed {} documents in {} ({}ms)", items, index_name, took);
        }
        
        Ok(stats)
    }
    
    /// Index blockchain events
    pub async fn index_blockchain_events(&self, events: &[BlockchainEvent]) -> Result<IndexingStats> {
        let documents: Result<Vec<Value>, _> = events.iter()
            .map(|event| serde_json::to_value(event))
            .collect();
        
        self.bulk_index(&self.blockchain_index, &documents?).await
    }
    
    /// Index user activities
    pub async fn index_user_activity(&self, activities: &[UserActivity]) -> Result<IndexingStats> {
        let documents: Result<Vec<Value>, _> = activities.iter()
            .map(|activity| serde_json::to_value(activity))
            .collect();
        
        self.bulk_index(&self.activity_index, &documents?).await
    }
    
    /// Index NFT metadata
    pub async fn index_nft_metadata(&self, nfts: &[NFTMetadata]) -> Result<IndexingStats> {
        let documents: Result<Vec<Value>, _> = nfts.iter()
            .map(|nft| serde_json::to_value(nft))
            .collect();
        
        self.bulk_index(&self.nft_index, &documents?).await
    }
    
    /// Search documents
    pub async fn search(&self, index_name: &str, query: &Value) -> Result<Vec<Value>> {
        let search_url = format!("{}/{}/_search", self.base_url, index_name);
        
        let response = self.client.post(&search_url)
            .json(query)
            .send()
            .await
            .context("Failed to execute search")?;
        
        let response_body: Value = response.json().await?;
        let empty_vec = Vec::new();
        let hits = response_body["hits"]["hits"].as_array().unwrap_or(&empty_vec);
        
        let documents: Vec<Value> = hits.iter()
            .map(|hit| hit["_source"].clone())
            .collect();
        
        info!("🔍 Found {} documents in {}", documents.len(), index_name);
        Ok(documents)
    }
    
    /// Get index statistics
    pub async fn get_index_stats(&self) -> Result<IndexStatistics> {
        let mut stats = IndexStatistics::default();
        
        let indices = [
            (&self.blockchain_index, &mut stats.blockchain_events),
            (&self.activity_index, &mut stats.user_activities),
            (&self.nft_index, &mut stats.nft_metadata),
        ];
        
        for (index_name, stat_field) in indices {
            let count_url = format!("{}/_cat/count/{}?format=json", self.base_url, index_name);
            
            if let Ok(response) = self.client.get(&count_url).send().await {
                if let Ok(count_data) = response.json::<Vec<Value>>().await {
                    if let Some(count_info) = count_data.first() {
                        if let Some(count_str) = count_info["count"].as_str() {
                            if let Ok(count) = count_str.parse::<u64>() {
                                *stat_field = count;
                            }
                        }
                    }
                }
            }
        }
        
        Ok(stats)
    }
}

#[derive(Debug, Default)]
pub struct IndexingStats {
    pub documents_indexed: usize,
    pub took_ms: u64,
    pub has_errors: bool,
    pub processing_time: Duration,
}

#[derive(Debug, Default)]
pub struct IndexStatistics {
    pub blockchain_events: u64,
    pub user_activities: u64,
    pub nft_metadata: u64,
}

// Performance testing
pub async fn run_performance_tests(indexer: &ElasticsearchIndexer) -> Result<PerformanceReport> {
    info!("🚀 Starting Elasticsearch indexer performance tests...");
    let mut report = PerformanceReport::default();
    
    // Test 1: Blockchain event indexing
    let blockchain_start = Instant::now();
    let test_events = create_test_blockchain_events(5000)?;
    
    let indexing_stats = indexer.index_blockchain_events(&test_events).await?;
    report.blockchain_indexing_time = blockchain_start.elapsed();
    report.blockchain_events_indexed = indexing_stats.documents_indexed;
    report.blockchain_indexing_rate = if indexing_stats.processing_time.as_secs_f64() > 0.0 {
        (indexing_stats.documents_indexed as f64 / indexing_stats.processing_time.as_secs_f64()) as u64
    } else { 0 };
    
    // Test 2: User activity indexing
    let activity_start = Instant::now();
    let test_activities = create_test_user_activities(3000)?;
    
    let activity_stats = indexer.index_user_activity(&test_activities).await?;
    report.activity_indexing_time = activity_start.elapsed();
    report.user_activities_indexed = activity_stats.documents_indexed;
    report.activity_indexing_rate = if activity_stats.processing_time.as_secs_f64() > 0.0 {
        (activity_stats.documents_indexed as f64 / activity_stats.processing_time.as_secs_f64()) as u64
    } else { 0 };
    
    // Test 3: NFT metadata indexing
    let nft_start = Instant::now();
    let test_nfts = create_test_nft_metadata(1000)?;
    
    let nft_stats = indexer.index_nft_metadata(&test_nfts).await?;
    report.nft_indexing_time = nft_start.elapsed();
    report.nft_metadata_indexed = nft_stats.documents_indexed;
    report.nft_indexing_rate = if nft_stats.processing_time.as_secs_f64() > 0.0 {
        (nft_stats.documents_indexed as f64 / nft_stats.processing_time.as_secs_f64()) as u64
    } else { 0 };
    
    // Test 4: Search performance
    let search_start = Instant::now();
    let search_query = json!({
        "size": 100,
        "query": {
            "range": {
                "block_number": {
                    "gte": 1000,
                    "lte": 2000
                }
            }
        },
        "sort": [{"timestamp": {"order": "desc"}}]
    });
    
    let _search_results = indexer.search("bunkerverse-blockchain-events", &search_query).await?;
    report.search_time = search_start.elapsed();
    
    // Get final statistics
    report.final_stats = indexer.get_index_stats().await?;
    
    info!("✅ Elasticsearch performance tests completed");
    Ok(report)
}

#[derive(Debug, Default)]
pub struct PerformanceReport {
    pub blockchain_indexing_time: Duration,
    pub blockchain_events_indexed: usize,
    pub blockchain_indexing_rate: u64,
    pub activity_indexing_time: Duration,
    pub user_activities_indexed: usize,
    pub activity_indexing_rate: u64,
    pub nft_indexing_time: Duration,
    pub nft_metadata_indexed: usize,
    pub nft_indexing_rate: u64,
    pub search_time: Duration,
    pub final_stats: IndexStatistics,
}

impl PerformanceReport {
    pub fn print_summary(&self) {
        println!("\n🔍 ELASTICSEARCH INDEXER PoC - PERFORMANCE REPORT");
        println!("==================================================");
        
        println!("\n🧱 Blockchain Event Indexing:");
        println!("  • Indexed {} events in {:?} ({} events/sec)", 
                 self.blockchain_events_indexed,
                 self.blockchain_indexing_time,
                 self.blockchain_indexing_rate);
        
        println!("\n👤 User Activity Indexing:");
        println!("  • Indexed {} activities in {:?} ({} activities/sec)", 
                 self.user_activities_indexed,
                 self.activity_indexing_time,
                 self.activity_indexing_rate);
        
        println!("\n🎨 NFT Metadata Indexing:");
        println!("  • Indexed {} NFTs in {:?} ({} NFTs/sec)", 
                 self.nft_metadata_indexed,
                 self.nft_indexing_time,
                 self.nft_indexing_rate);
        
        println!("\n🔍 Search Performance:");
        println!("  • Range query completed in {:?}", self.search_time);
        
        println!("\n📊 Final Index Statistics:");
        println!("  • Blockchain events: {}", self.final_stats.blockchain_events);
        println!("  • User activities: {}", self.final_stats.user_activities);
        println!("  • NFT metadata: {}", self.final_stats.nft_metadata);
        
        println!("==================================================");
    }
}

// Helper functions for creating test data
fn create_test_blockchain_events(count: usize) -> Result<Vec<BlockchainEvent>> {
    let mut events = Vec::with_capacity(count);
    
    for i in 0..count {
        events.push(BlockchainEvent {
            id: format!("event_{}", i),
            block_number: 1000 + (i as u64),
            transaction_hash: format!("0x{:064x}", i),
            event_type: match i % 4 {
                0 => "NFTMinted".to_string(),
                1 => "NFTTransferred".to_string(),
                2 => "NFTBurned".to_string(),
                _ => "TokenTransfer".to_string(),
            },
            contract_address: "0x742d35Cc6634C0532925a3b8D0B57FA6c98A6B50".to_string(),
            from_address: if i % 4 == 0 { None } else { Some(format!("0x{:040x}", i)) },
            to_address: Some(format!("0x{:040x}", i + 1)),
            token_id: if i % 2 == 0 { Some(i as u64) } else { None },
            value: Some(format!("{}", i * 1000)),
            timestamp: Utc::now(),
            raw_data: json!({
                "index": i,
                "test": true,
                "metadata": format!("Event data for {}", i)
            }),
        });
    }
    
    Ok(events)
}

fn create_test_user_activities(count: usize) -> Result<Vec<UserActivity>> {
    let mut activities = Vec::with_capacity(count);
    
    for i in 0..count {
        activities.push(UserActivity {
            id: format!("activity_{}", i),
            user_id: format!("user_{}", i % 100),
            activity_type: match i % 5 {
                0 => "login".to_string(),
                1 => "nft_mint".to_string(),
                2 => "nft_transfer".to_string(),
                3 => "marketplace_view".to_string(),
                _ => "logout".to_string(),
            },
            description: format!("User activity #{}", i),
            metadata: json!({
                "session_id": format!("session_{}", i),
                "platform": "web",
                "version": "1.0.0"
            }),
            timestamp: Utc::now(),
            ip_address: Some(format!("192.168.1.{}", i % 255)),
            user_agent: Some("BunkerverseClient/1.0.0".to_string()),
        });
    }
    
    Ok(activities)
}

fn create_test_nft_metadata(count: usize) -> Result<Vec<NFTMetadata>> {
    let mut nfts = Vec::with_capacity(count);
    
    for i in 0..count {
        nfts.push(NFTMetadata {
            id: format!("nft_{}", i),
            token_id: i as u64,
            contract_address: "0x742d35Cc6634C0532925a3b8D0B57FA6c98A6B50".to_string(),
            name: format!("Bunkerverse NFT #{}", i),
            description: format!("A unique Bunkerverse collectible item #{}", i),
            image_url: format!("ipfs://QmTest{}/image.png", i),
            attributes: vec![
                NFTAttribute {
                    trait_type: "Rarity".to_string(),
                    value: json!(match i % 4 {
                        0 => "Common",
                        1 => "Uncommon", 
                        2 => "Rare",
                        _ => "Epic"
                    }),
                    display_type: None,
                },
                NFTAttribute {
                    trait_type: "Level".to_string(),
                    value: json!(i % 10 + 1),
                    display_type: Some("number".to_string()),
                },
            ],
            owner: format!("0x{:040x}", i),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        });
    }
    
    Ok(nfts)
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    info!("🚀 Starting Elasticsearch Indexer PoC");
    
    // Try to connect to Elasticsearch (using the port from docker-compose)
    let elasticsearch_url = "http://localhost:9200";
    
    let indexer = ElasticsearchIndexer::new(elasticsearch_url).await
        .context("Failed to connect to Elasticsearch - ensure it's running via docker-compose up -d elasticsearch")?;
    
    // Run comprehensive performance tests
    let report = run_performance_tests(&indexer).await?;
    
    // Print results
    report.print_summary();
    
    info!("🎉 Elasticsearch Indexer PoC completed successfully!");
    
    Ok(())
}

