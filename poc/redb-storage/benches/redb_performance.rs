use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use redb::{Database, TableDefinition};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use uuid::Uuid;

// Table definitions
const BENCH_TABLE: TableDefinition<&str, &[u8]> = TableDefinition::new("bench_table");
const NUMERIC_TABLE: TableDefinition<u64, &[u8]> = TableDefinition::new("numeric_table");

#[derive(Serialize, Deserialize)]
struct BenchData {
    id: Uuid,
    name: String,
    value: u64,
    metadata: serde_json::Value,
}

fn benchmark_redb_operations(c: &mut Criterion) {
    // Setup test database
    let db = Database::create("bench_test.redb").unwrap();
    
    // Initialize tables
    {
        let write_txn = db.begin_write().unwrap();
        let _ = write_txn.open_table(BENCH_TABLE).unwrap();
        let _ = write_txn.open_table(NUMERIC_TABLE).unwrap();
        write_txn.commit().unwrap();
    }
    
    // Benchmark group for write operations
    let mut group = c.benchmark_group("redb_writes");
    group.measurement_time(Duration::from_secs(10));
    
    for size in [100, 1000, 10000].iter() {
        group.bench_with_input(BenchmarkId::new("create_records", size), size, |b, &size| {
            b.iter(|| {
                let write_txn = db.begin_write().unwrap();
                {
                    let mut table = write_txn.open_table(BENCH_TABLE).unwrap();
                    
                    for i in 0..size {
                        let data = BenchData {
                            id: Uuid::new_v4(),
                            name: format!("bench_item_{}", i),
                            value: i as u64,
                            metadata: serde_json::json!({
                                "category": "benchmark",
                                "index": i,
                                "timestamp": "2025-09-08T14:00:00Z"
                            }),
                        };
                        
                        let key = format!("key_{}", i);
                        let value = serde_json::to_vec(&data).unwrap();
                        table.insert(black_box(key.as_str()), black_box(value.as_slice())).unwrap();
                    }
                }
                write_txn.commit().unwrap();
            });
        });
    }
    group.finish();
    
    // Pre-populate database for read benchmarks
    {
        let write_txn = db.begin_write().unwrap();
        {
            let mut table = write_txn.open_table(BENCH_TABLE).unwrap();
            let mut numeric_table = write_txn.open_table(NUMERIC_TABLE).unwrap();
            
            for i in 0..10000u64 {
                let data = BenchData {
                    id: Uuid::new_v4(),
                    name: format!("read_bench_item_{}", i),
                    value: i,
                    metadata: serde_json::json!({"read_test": true}),
                };
                
                let key = format!("read_key_{}", i);
                let value = serde_json::to_vec(&data).unwrap();
                table.insert(key.as_str(), value.as_slice()).unwrap();
                
                // Also populate numeric table for range queries
                numeric_table.insert(&i, value.as_slice()).unwrap();
            }
        }
        write_txn.commit().unwrap();
    }
    
    // Benchmark group for read operations
    let mut group = c.benchmark_group("redb_reads");
    group.measurement_time(Duration::from_secs(10));
    
    group.bench_function("random_reads_1000", |b| {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        b.iter(|| {
            let read_txn = db.begin_read().unwrap();
            let table = read_txn.open_table(BENCH_TABLE).unwrap();
            
            for _ in 0..1000 {
                let key = format!("read_key_{}", rng.gen_range(0..10000));
                let _ = table.get(black_box(key.as_str())).unwrap();
            }
        });
    });
    
    group.bench_function("sequential_reads_1000", |b| {
        b.iter(|| {
            let read_txn = db.begin_read().unwrap();
            let table = read_txn.open_table(BENCH_TABLE).unwrap();
            
            for i in 0..1000 {
                let key = format!("read_key_{}", i);
                let _ = table.get(black_box(key.as_str())).unwrap();
            }
        });
    });
    
    group.finish();
    
    // Benchmark group for range queries
    let mut group = c.benchmark_group("redb_range_queries");
    group.measurement_time(Duration::from_secs(5));
    
    for range_size in [100, 1000, 5000].iter() {
        group.bench_with_input(BenchmarkId::new("range_scan", range_size), range_size, |b, &range_size| {
            b.iter(|| {
                let read_txn = db.begin_read().unwrap();
                let table = read_txn.open_table(NUMERIC_TABLE).unwrap();
                
                let start_key = 1000u64;
                let end_key = start_key + range_size;
                
                let mut count = 0;
                let range = table.range(start_key..end_key).unwrap();
                for result in range {
                    let _ = black_box(result.unwrap());
                    count += 1;
                }
                black_box(count);
            });
        });
    }
    group.finish();
    
    // Benchmark concurrent operations
    let mut group = c.benchmark_group("redb_concurrent");
    group.measurement_time(Duration::from_secs(10));
    
    group.bench_function("concurrent_reads", |b| {
        use std::sync::Arc;
        use std::thread;
        
        let db = Arc::new(db);
        
        b.iter(|| {
            let mut handles = Vec::new();
            
            for thread_id in 0..4 {
                let db_clone = db.clone();
                let handle = thread::spawn(move || {
                    let read_txn = db_clone.begin_read().unwrap();
                    let table = read_txn.open_table(BENCH_TABLE).unwrap();
                    
                    for i in 0..250 {
                        let key = format!("read_key_{}", (thread_id * 250) + i);
                        let _ = table.get(key.as_str()).unwrap();
                    }
                });
                handles.push(handle);
            }
            
            for handle in handles {
                handle.join().unwrap();
            }
        });
    });
    
    group.finish();
    
    // Cleanup
    drop(db);
    std::fs::remove_file("bench_test.redb").ok();
}

criterion_group!(benches, benchmark_redb_operations);
criterion_main!(benches);