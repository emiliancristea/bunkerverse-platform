use crate::config::StubConfiguration;
use crate::stub::{IndexerStub, RequestContext, SmartStub};
use anyhow::Result;
use chrono::Utc;
use std::{collections::HashMap, sync::Arc};
use tonic::{Request, Response, Status};
use tracing::info;
use uuid::Uuid;

// Include the generated protobuf code
pub mod bunkerverse {
    pub mod services {
        pub mod v1 {
            tonic::include_proto!("bunkerverse.services.v1");
        }
    }
    pub mod core {
        pub mod v1 {
            tonic::include_proto!("bunkerverse.core.v1");
        }
    }
}

use bunkerverse::services::v1::*;

pub struct IndexerGrpcService {
    stub: Arc<tokio::sync::Mutex<IndexerStub>>,
}

impl IndexerGrpcService {
    pub fn new(config: StubConfiguration) -> Self {
        Self {
            stub: Arc::new(tokio::sync::Mutex::new(IndexerStub::new(config))),
        }
    }

    async fn create_context(&self, trace_id: Option<String>) -> RequestContext {
        let stub = self.stub.lock().await;
        RequestContext {
            request_id: Uuid::new_v4().to_string(),
            trace_id,
            timestamp: Utc::now(),
            enable_crypto: stub.get_configuration().dual_mode.enable_crypto,
        }
    }

    async fn simulate_latency_and_errors(
        &self,
        context: &RequestContext,
        method: &str,
    ) -> Result<(), Status> {
        let stub = self.stub.lock().await;

        // Simulate latency
        let latency = stub.calculate_response_latency();
        tokio::time::sleep(tokio::time::Duration::from_millis(
            latency.as_millis() as u64
        ))
        .await;

        // Check for error injection
        if stub.should_inject_error_response() {
            stub.log_response(context, method, latency.as_millis() as u64, 500, true);
            return Err(Status::internal("Simulated indexer gRPC error"));
        }

        stub.log_response(context, method, latency.as_millis() as u64, 200, false);
        Ok(())
    }
}

#[tonic::async_trait]
impl indexer_service_server::IndexerService for IndexerGrpcService {
    async fn get_events(
        &self,
        request: Request<GetEventsRequest>,
    ) -> Result<Response<GetEventsResponse>, Status> {
        let req = request.into_inner();
        let context = self.create_context(Some(req.trace_id.clone())).await;

        {
            let stub = self.stub.lock().await;
            stub.log_request(&context, "GetEvents", "gRPC");

            // Check crypto features
            if let Err(err) = stub.check_crypto_features(&context) {
                return Err(Status::permission_denied(err));
            }
        }

        self.simulate_latency_and_errors(&context, "GetEvents")
            .await?;

        let mock_events = vec![
            bunkerverse::core::v1::CanonicalEventProto {
                event_id: "event_001".to_string(),
                event_type: "NftTransfer".to_string(),
                player_id: "player_123".to_string(),
                block_number: 12345,
                transaction_hash: "0xabc123".to_string(),
                log_index: 0,
                timestamp: Utc::now().timestamp(),
                data_json: r#"{"nft_id":"nft_001","from":"player_456","to":"player_123","price_wei":"1000000000000000000"}"#.to_string(),
            }
        ];

        let response = GetEventsResponse {
            result: Some(get_events_response::Result::Success(GetEventsSuccess {
                events: mock_events,
                pagination: req.pagination,
                indexing_stats: Some(IndexingStatsProto {
                    total_events_indexed: 10000,
                    total_blocks_indexed: 5000,
                    events_per_second_current: 50,
                    events_per_second_average: 45,
                    contract_event_counts: HashMap::new(),
                    event_type_counts: HashMap::from([
                        ("NftTransfer".to_string(), 3000),
                        ("MissionCompleted".to_string(), 2000),
                    ]),
                    indexing_start_time: (Utc::now() - chrono::Duration::hours(24)).timestamp(),
                    last_successful_sync: Utc::now().timestamp(),
                    recent_errors: vec![],
                    database_size_bytes: 1048576, // 1MB
                }),
            })),
        };

        Ok(Response::new(response))
    }

    async fn get_events_by_player(
        &self,
        request: Request<GetEventsByPlayerRequest>,
    ) -> Result<Response<GetEventsByPlayerResponse>, Status> {
        let req = request.into_inner();
        let context = self.create_context(Some(req.trace_id.clone())).await;

        {
            let stub = self.stub.lock().await;
            stub.log_request(&context, "GetEventsByPlayer", "gRPC");
        }

        self.simulate_latency_and_errors(&context, "GetEventsByPlayer")
            .await?;

        let mock_events = vec![bunkerverse::core::v1::CanonicalEventProto {
            event_id: "event_player_001".to_string(),
            event_type: "MissionCompleted".to_string(),
            player_id: req.player_id.clone(),
            block_number: 12340,
            transaction_hash: "0xdef456".to_string(),
            log_index: 1,
            timestamp: (Utc::now() - chrono::Duration::hours(1)).timestamp(),
            data_json:
                r#"{"mission_id":"mission_001","reward_wei":"500000000000000000","experience":100}"#
                    .to_string(),
        }];

        let response = GetEventsByPlayerResponse {
            result: Some(get_events_by_player_response::Result::Success(
                GetEventsByPlayerSuccess {
                    events: mock_events,
                    pagination: req.pagination,
                    player_stats: Some(PlayerEventStatsProto {
                        player_id: req.player_id,
                        total_events: 25,
                        nft_events: 10,
                        transaction_events: 8,
                        mission_events: 5,
                        staking_events: 2,
                        first_event_timestamp: (Utc::now() - chrono::Duration::days(30))
                            .timestamp(),
                        last_event_timestamp: Utc::now().timestamp(),
                        event_type_counts: HashMap::from([
                            ("NftTransfer".to_string(), 10),
                            ("MissionCompleted".to_string(), 5),
                            ("StakeDeposit".to_string(), 2),
                        ]),
                    }),
                },
            )),
        };

        Ok(Response::new(response))
    }

    async fn get_events_by_type(
        &self,
        request: Request<GetEventsByTypeRequest>,
    ) -> Result<Response<GetEventsByTypeResponse>, Status> {
        let req = request.into_inner();
        let context = self.create_context(Some(req.trace_id.clone())).await;

        {
            let stub = self.stub.lock().await;
            stub.log_request(&context, "GetEventsByType", "gRPC");
        }

        self.simulate_latency_and_errors(&context, "GetEventsByType")
            .await?;

        let mock_events = vec![bunkerverse::core::v1::CanonicalEventProto {
            event_id: "event_type_001".to_string(),
            event_type: req.event_type.clone(),
            player_id: "player_789".to_string(),
            block_number: 12350,
            transaction_hash: "0xghi789".to_string(),
            log_index: 0,
            timestamp: Utc::now().timestamp(),
            data_json: "{}".to_string(),
        }];

        let response = GetEventsByTypeResponse {
            result: Some(get_events_by_type_response::Result::Success(
                GetEventsByTypeSuccess {
                    events: mock_events,
                    pagination: req.pagination,
                    type_stats: Some(EventTypeStatsProto {
                        event_type: req.event_type,
                        total_count: 1500,
                        count_24h: 50,
                        count_7d: 300,
                        count_30d: 1200,
                        average_per_day: 40.0,
                        first_occurrence: (Utc::now() - chrono::Duration::days(30)).timestamp(),
                        last_occurrence: Utc::now().timestamp(),
                        top_players: vec!["player_123".to_string(), "player_456".to_string()],
                    }),
                },
            )),
        };

        Ok(Response::new(response))
    }

    async fn get_events_by_block(
        &self,
        request: Request<GetEventsByBlockRequest>,
    ) -> Result<Response<GetEventsByBlockResponse>, Status> {
        let req = request.into_inner();
        let context = self.create_context(Some(req.trace_id.clone())).await;

        {
            let stub = self.stub.lock().await;
            stub.log_request(&context, "GetEventsByBlock", "gRPC");

            // Check crypto features for blockchain queries
            if let Err(err) = stub.check_crypto_features(&context) {
                return Err(Status::permission_denied(err));
            }
        }

        self.simulate_latency_and_errors(&context, "GetEventsByBlock")
            .await?;

        let mock_events = vec![bunkerverse::core::v1::CanonicalEventProto {
            event_id: "event_block_001".to_string(),
            event_type: "BlockProcessed".to_string(),
            player_id: "system".to_string(),
            block_number: req.start_block,
            transaction_hash: "0xjkl012".to_string(),
            log_index: 0,
            timestamp: Utc::now().timestamp(),
            data_json: r#"{"gas_used":"8500000","transaction_count":42}"#.to_string(),
        }];

        let response = GetEventsByBlockResponse {
            result: Some(get_events_by_block_response::Result::Success(
                GetEventsByBlockSuccess {
                    events: mock_events,
                    block_stats: Some(BlockRangeStatsProto {
                        start_block: req.start_block,
                        end_block: req.end_block,
                        total_events: 5,
                        total_transactions: 3,
                        contract_event_counts: HashMap::new(),
                        event_type_counts: HashMap::from([
                            ("BlockProcessed".to_string(), 1),
                            ("NftTransfer".to_string(), 2),
                        ]),
                        range_start_timestamp: (Utc::now() - chrono::Duration::seconds(30))
                            .timestamp(),
                        range_end_timestamp: Utc::now().timestamp(),
                    }),
                },
            )),
        };

        Ok(Response::new(response))
    }

    async fn get_player_chain_state(
        &self,
        request: Request<GetPlayerChainStateRequest>,
    ) -> Result<Response<GetPlayerChainStateResponse>, Status> {
        let req = request.into_inner();
        let context = self.create_context(Some(req.trace_id.clone())).await;

        {
            let stub = self.stub.lock().await;
            stub.log_request(&context, "GetPlayerChainState", "gRPC");
        }

        self.simulate_latency_and_errors(&context, "GetPlayerChainState")
            .await?;

        // Return mock chain state
        let chain_state = bunkerverse::core::v1::AgentChainStateProto {
            player_id: req.player_id,
            bunker_tag: "TestPlayer".to_string(),
            ntc_balance_wei: if context.enable_crypto {
                5000000000000000000u64
            } else {
                5000
            },
            staked_ntc_wei: if context.enable_crypto {
                1000000000000000000u64
            } else {
                1000
            },
            unclaimed_rewards_wei: if context.enable_crypto {
                250000000000000000u64
            } else {
                250
            },
            owned_nfts: if context.enable_crypto {
                vec![bunkerverse::core::v1::NftDetailsProto {
                    nft_id: "nft_player_001".to_string(),
                    token_id: "42".to_string(),
                    contract_address: "0xcontract123".to_string(),
                    name: "Player Weapon".to_string(),
                    description: "A powerful weapon".to_string(),
                    image_url: "https://example.com/weapon.png".to_string(),
                    metadata_json: "{}".to_string(),
                    item_type: bunkerverse::core::v1::ItemTypeProto::Weapon as i32,
                    item_rarity: bunkerverse::core::v1::ItemRarityProto::Epic as i32,
                    item_condition: bunkerverse::core::v1::ItemConditionProto::Excellent as i32,
                    mint_timestamp: Utc::now().timestamp(),
                    creator_address: "0xcreator123".to_string(),
                }]
            } else {
                vec![] // No NFTs in MVE mode
            },
            equipped_items: HashMap::new(),
            active_missions: vec!["mission_001".to_string()],
            completed_missions: vec!["mission_starter".to_string()],
            social_connections: vec![],
            last_activity_timestamp: Utc::now().timestamp(),
            account_creation_timestamp: (Utc::now() - chrono::Duration::days(30)).timestamp(),
        };

        let response = GetPlayerChainStateResponse {
            result: Some(get_player_chain_state_response::Result::ChainState(
                chain_state,
            )),
        };

        Ok(Response::new(response))
    }

    async fn get_nft_ownership(
        &self,
        request: Request<GetNftOwnershipRequest>,
    ) -> Result<Response<GetNftOwnershipResponse>, Status> {
        let req = request.into_inner();
        let context = self.create_context(Some(req.trace_id.clone())).await;

        {
            let stub = self.stub.lock().await;
            stub.log_request(&context, "GetNftOwnership", "gRPC");

            // Check crypto features for NFT queries
            if let Err(err) = stub.check_crypto_features(&context) {
                return Err(Status::permission_denied(err));
            }
        }

        self.simulate_latency_and_errors(&context, "GetNftOwnership")
            .await?;

        let ownership_data = GetNftOwnershipSuccess {
            nft_details: Some(bunkerverse::core::v1::NftDetailsProto {
                nft_id: req.nft_id.clone(),
                token_id: "1".to_string(),
                contract_address: "0xcontract456".to_string(),
                name: "Mock NFT".to_string(),
                description: "A mock NFT for testing".to_string(),
                image_url: "https://example.com/nft.png".to_string(),
                metadata_json: "{}".to_string(),
                item_type: bunkerverse::core::v1::ItemTypeProto::Armor as i32,
                item_rarity: bunkerverse::core::v1::ItemRarityProto::Rare as i32,
                item_condition: bunkerverse::core::v1::ItemConditionProto::Good as i32,
                mint_timestamp: (Utc::now() - chrono::Duration::days(15)).timestamp(),
                creator_address: "0xcreator456".to_string(),
            }),
            current_state: Some(bunkerverse::core::v1::NftMutableStateProto {
                nft_id: req.nft_id.clone(),
                current_owner_id: "player_123".to_string(),
                market_status: bunkerverse::core::v1::MarketStatusProto::Owned as i32,
                equipped_robot_id: None,
                last_transaction_hash: Some("0xlasttx789".to_string()),
                last_updated_timestamp: Utc::now().timestamp(),
            }),
            ownership_history: if req.include_history {
                vec![
                    NftOwnershipHistoryProto {
                        previous_owner_id: "system".to_string(),
                        new_owner_id: "player_456".to_string(),
                        transfer_type: "mint".to_string(),
                        transfer_price_wei: 0,
                        transaction_hash: "0xmint123".to_string(),
                        block_number: 12000,
                        transfer_timestamp: (Utc::now() - chrono::Duration::days(15)).timestamp(),
                    },
                    NftOwnershipHistoryProto {
                        previous_owner_id: "player_456".to_string(),
                        new_owner_id: "player_123".to_string(),
                        transfer_type: "purchase".to_string(),
                        transfer_price_wei: 500000000000000000u64,
                        transaction_hash: "0xpurchase456".to_string(),
                        block_number: 12100,
                        transfer_timestamp: (Utc::now() - chrono::Duration::days(7)).timestamp(),
                    },
                ]
            } else {
                vec![]
            },
        };

        let response = GetNftOwnershipResponse {
            result: Some(get_nft_ownership_response::Result::Success(ownership_data)),
        };

        Ok(Response::new(response))
    }

    async fn get_contract_state(
        &self,
        request: Request<GetContractStateRequest>,
    ) -> Result<Response<GetContractStateResponse>, Status> {
        let req = request.into_inner();
        let context = self.create_context(Some(req.trace_id.clone())).await;

        {
            let stub = self.stub.lock().await;
            stub.log_request(&context, "GetContractState", "gRPC");

            // Check crypto features for contract state queries
            if let Err(err) = stub.check_crypto_features(&context) {
                return Err(Status::permission_denied(err));
            }
        }

        self.simulate_latency_and_errors(&context, "GetContractState")
            .await?;

        let contract_state = GetContractStateSuccess {
            contract_address: req.contract_address,
            state_key: req.state_key,
            state_value: b"mock_state_value".to_vec(),
            state_value_json: r#"{"value":"mock_state_value","type":"string"}"#.to_string(),
            last_updated_block: 12345,
            last_updated_timestamp: Utc::now().timestamp(),
        };

        let response = GetContractStateResponse {
            result: Some(get_contract_state_response::Result::Success(contract_state)),
        };

        Ok(Response::new(response))
    }

    async fn get_indexing_status(
        &self,
        request: Request<GetIndexingStatusRequest>,
    ) -> Result<Response<GetIndexingStatusResponse>, Status> {
        let req = request.into_inner();
        let context = self.create_context(Some(req.trace_id.clone())).await;

        {
            let stub = self.stub.lock().await;
            stub.log_request(&context, "GetIndexingStatus", "gRPC");
        }

        self.simulate_latency_and_errors(&context, "GetIndexingStatus")
            .await?;

        let indexing_status = IndexingStatusProto {
            current_block: 12345,
            latest_block: 12350,
            blocks_behind: 5,
            is_syncing: true,
            sync_progress_percent: 99.96,
            last_sync_timestamp: Utc::now().timestamp(),
            events_processed_total: 50000,
            events_per_second: 45,
            unhealthy_contracts: vec![],
            stats: Some(IndexingStatsProto {
                total_events_indexed: 50000,
                total_blocks_indexed: 12345,
                events_per_second_current: 45,
                events_per_second_average: 42,
                contract_event_counts: HashMap::new(),
                event_type_counts: HashMap::from([
                    ("NftTransfer".to_string(), 15000),
                    ("MissionCompleted".to_string(), 10000),
                    ("StakeDeposit".to_string(), 5000),
                ]),
                indexing_start_time: (Utc::now() - chrono::Duration::hours(48)).timestamp(),
                last_successful_sync: Utc::now().timestamp(),
                recent_errors: vec![],
                database_size_bytes: 104857600, // 100MB
            }),
        };

        let response = GetIndexingStatusResponse {
            result: Some(get_indexing_status_response::Result::Status(
                indexing_status,
            )),
        };

        Ok(Response::new(response))
    }

    async fn reindex_from_block(
        &self,
        request: Request<ReindexFromBlockRequest>,
    ) -> Result<Response<ReindexFromBlockResponse>, Status> {
        let req = request.into_inner();
        let context = self.create_context(Some(req.trace_id.clone())).await;

        {
            let stub = self.stub.lock().await;
            stub.log_request(&context, "ReindexFromBlock", "gRPC");

            // Check crypto features for reindexing operations
            if let Err(err) = stub.check_crypto_features(&context) {
                return Err(Status::permission_denied(err));
            }
        }

        self.simulate_latency_and_errors(&context, "ReindexFromBlock")
            .await?;

        let reindex_result = ReindexFromBlockSuccess {
            reindex_job_id: Uuid::new_v4().to_string(),
            start_block: req.start_block,
            estimated_blocks_to_process: 1000,
            estimated_completion_time: (Utc::now() + chrono::Duration::minutes(30)).timestamp(),
        };

        let response = ReindexFromBlockResponse {
            result: Some(reindex_from_block_response::Result::Success(reindex_result)),
        };

        Ok(Response::new(response))
    }

    async fn health(
        &self,
        request: Request<HealthRequest>,
    ) -> Result<Response<HealthResponse>, Status> {
        let _req = request.into_inner();
        let context = self.create_context(None).await;

        {
            let stub = self.stub.lock().await;
            stub.log_request(&context, "Health", "gRPC");
        }

        let response = HealthResponse {
            status: "HEALTHY".to_string(),
            timestamp: Utc::now().timestamp(),
            service_name: "indexer-service".to_string(),
            version: "0.1.0".to_string(),
            details: HashMap::new(),
        };

        Ok(Response::new(response))
    }
}
