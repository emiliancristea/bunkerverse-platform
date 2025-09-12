use crate::config::StubConfiguration;
use crate::stub::{MarketplaceStub, RequestContext, SmartStub};
use anyhow::Result;
use chrono::Utc;
use std::sync::Arc;
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

pub struct MarketplaceGrpcService {
    stub: Arc<tokio::sync::Mutex<MarketplaceStub>>,
}

impl MarketplaceGrpcService {
    pub fn new(config: StubConfiguration) -> Self {
        Self {
            stub: Arc::new(tokio::sync::Mutex::new(MarketplaceStub::new(config))),
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
            return Err(Status::internal("Simulated gRPC error"));
        }

        stub.log_response(context, method, latency.as_millis() as u64, 200, false);
        Ok(())
    }
}

#[tonic::async_trait]
impl marketplace_service_server::MarketplaceService for MarketplaceGrpcService {
    async fn get_market_listings(
        &self,
        request: Request<GetMarketListingsRequest>,
    ) -> Result<Response<GetMarketListingsResponse>, Status> {
        let req = request.into_inner();
        let context = self.create_context(Some(req.trace_id.clone())).await;

        {
            let stub = self.stub.lock().await;
            stub.log_request(&context, "GetMarketListings", "gRPC");
        }

        self.simulate_latency_and_errors(&context, "GetMarketListings")
            .await?;

        // Generate mock listings based on dual-mode configuration
        let listings = if context.enable_crypto {
            vec![MarketListingProto {
                listing_id: "listing_001".to_string(),
                nft_details: Some(bunkerverse::core::v1::NftDetailsProto {
                    nft_id: "nft_001".to_string(),
                    token_id: "1".to_string(),
                    contract_address: "0xabcdef1234567890".to_string(),
                    name: "Epic Weapon NFT".to_string(),
                    description: "A legendary weapon".to_string(),
                    image_url: "https://example.com/nft.png".to_string(),
                    metadata_json: "{}".to_string(),
                    item_type: bunkerverse::core::v1::ItemTypeProto::Weapon as i32,
                    item_rarity: bunkerverse::core::v1::ItemRarityProto::Epic as i32,
                    item_condition: bunkerverse::core::v1::ItemConditionProto::Excellent as i32,
                    mint_timestamp: Utc::now().timestamp(),
                    creator_address: "0x1234567890abcdef".to_string(),
                }),
                nft_state: Some(bunkerverse::core::v1::NftMutableStateProto {
                    nft_id: "nft_001".to_string(),
                    current_owner_id: "player_123".to_string(),
                    market_status: bunkerverse::core::v1::MarketStatusProto::ListedForSale as i32,
                    equipped_robot_id: None,
                    last_transaction_hash: Some("0xmocktxhash123".to_string()),
                    last_updated_timestamp: Utc::now().timestamp(),
                }),
                seller_player_id: "player_123".to_string(),
                seller_bunker_tag: "TestSeller".to_string(),
                listing_price_ntc_wei: 1000000000000000000u64, // 1 NTC
                listing_type: bunkerverse::core::v1::MarketStatusProto::ListedForSale as i32,
                listing_created_at: Utc::now().timestamp(),
                listing_expires_at: (Utc::now() + chrono::Duration::days(7)).timestamp(),
                view_count: 42,
                favorite_count: 5,
                similar_listings: vec!["listing_002".to_string()],
            }]
        } else {
            vec![MarketListingProto {
                listing_id: "listing_001".to_string(),
                nft_details: Some(bunkerverse::core::v1::NftDetailsProto {
                    nft_id: "item_001".to_string(),
                    token_id: "1".to_string(),
                    contract_address: "".to_string(),
                    name: "Basic Sword".to_string(),
                    description: "A basic sword item".to_string(),
                    image_url: "https://example.com/sword.png".to_string(),
                    metadata_json: "{}".to_string(),
                    item_type: bunkerverse::core::v1::ItemTypeProto::Weapon as i32,
                    item_rarity: bunkerverse::core::v1::ItemRarityProto::Common as i32,
                    item_condition: bunkerverse::core::v1::ItemConditionProto::Good as i32,
                    mint_timestamp: Utc::now().timestamp(),
                    creator_address: "system".to_string(),
                }),
                nft_state: Some(bunkerverse::core::v1::NftMutableStateProto {
                    nft_id: "item_001".to_string(),
                    current_owner_id: "player_456".to_string(),
                    market_status: bunkerverse::core::v1::MarketStatusProto::ListedForSale as i32,
                    equipped_robot_id: None,
                    last_transaction_hash: None,
                    last_updated_timestamp: Utc::now().timestamp(),
                }),
                seller_player_id: "player_456".to_string(),
                seller_bunker_tag: "MVEPlayer".to_string(),
                listing_price_ntc_wei: 100, // 100 credits in MVE mode
                listing_type: bunkerverse::core::v1::MarketStatusProto::ListedForSale as i32,
                listing_created_at: Utc::now().timestamp(),
                listing_expires_at: (Utc::now() + chrono::Duration::days(30)).timestamp(),
                view_count: 12,
                favorite_count: 1,
                similar_listings: vec![],
            }]
        };

        let response = GetMarketListingsResponse {
            result: Some(get_market_listings_response::Result::Success(
                GetMarketListingsSuccess {
                    listings,
                    pagination: req.pagination,
                    market_stats: Some(MarketStatsProto {
                        total_listings: if context.enable_crypto { 2 } else { 1 },
                        total_volume_24h_wei: if context.enable_crypto {
                            5000000000000000000u64
                        } else {
                            500
                        },
                        average_price_wei: if context.enable_crypto {
                            750000000000000000u64
                        } else {
                            150
                        },
                        total_sales_24h: 3,
                        item_type_stats: vec![],
                    }),
                },
            )),
        };

        Ok(Response::new(response))
    }

    async fn get_listing_details(
        &self,
        request: Request<GetListingDetailsRequest>,
    ) -> Result<Response<GetListingDetailsResponse>, Status> {
        let req = request.into_inner();
        let context = self.create_context(Some(req.trace_id.clone())).await;

        {
            let stub = self.stub.lock().await;
            stub.log_request(&context, "GetListingDetails", "gRPC");
        }

        self.simulate_latency_and_errors(&context, "GetListingDetails")
            .await?;

        let listing_detail = MarketListingDetailProto {
            listing: Some(MarketListingProto {
                listing_id: req.listing_id.clone(),
                nft_details: Some(bunkerverse::core::v1::NftDetailsProto {
                    nft_id: "nft_001".to_string(),
                    token_id: "1".to_string(),
                    contract_address: if context.enable_crypto {
                        "0xabcdef1234567890".to_string()
                    } else {
                        "".to_string()
                    },
                    name: "Mock NFT".to_string(),
                    description: "A mock NFT for testing".to_string(),
                    image_url: "https://example.com/nft.png".to_string(),
                    metadata_json: "{}".to_string(),
                    item_type: bunkerverse::core::v1::ItemTypeProto::Weapon as i32,
                    item_rarity: bunkerverse::core::v1::ItemRarityProto::Epic as i32,
                    item_condition: bunkerverse::core::v1::ItemConditionProto::Excellent as i32,
                    mint_timestamp: Utc::now().timestamp(),
                    creator_address: if context.enable_crypto {
                        "0x1234567890abcdef".to_string()
                    } else {
                        "system".to_string()
                    },
                }),
                nft_state: Some(bunkerverse::core::v1::NftMutableStateProto {
                    nft_id: "nft_001".to_string(),
                    current_owner_id: "player_123".to_string(),
                    market_status: bunkerverse::core::v1::MarketStatusProto::ListedForSale as i32,
                    equipped_robot_id: None,
                    last_transaction_hash: if context.enable_crypto {
                        Some("0xmocktxhash123".to_string())
                    } else {
                        None
                    },
                    last_updated_timestamp: Utc::now().timestamp(),
                }),
                seller_player_id: "player_123".to_string(),
                seller_bunker_tag: "TestSeller".to_string(),
                listing_price_ntc_wei: if context.enable_crypto {
                    1000000000000000000u64
                } else {
                    100
                },
                listing_type: bunkerverse::core::v1::MarketStatusProto::ListedForSale as i32,
                listing_created_at: (Utc::now() - chrono::Duration::hours(1)).timestamp(),
                listing_expires_at: (Utc::now() + chrono::Duration::days(1)).timestamp(),
                view_count: 42,
                favorite_count: 5,
                similar_listings: vec![],
            }),
            price_history: vec![],
            similar_listings: vec![],
            analytics: Some(MarketAnalyticsProto {
                floor_price_wei: if context.enable_crypto {
                    500000000000000000u64
                } else {
                    50
                },
                average_price_7d_wei: if context.enable_crypto {
                    750000000000000000u64
                } else {
                    75
                },
                average_price_30d_wei: if context.enable_crypto {
                    800000000000000000u64
                } else {
                    80
                },
                total_sales_7d: 15,
                total_sales_30d: 50,
                recent_sales: vec![],
            }),
            is_favorited_by_requester: false,
        };

        let response = GetListingDetailsResponse {
            result: Some(get_listing_details_response::Result::Listing(
                listing_detail,
            )),
        };

        Ok(Response::new(response))
    }

    async fn search_marketplace(
        &self,
        request: Request<SearchMarketplaceRequest>,
    ) -> Result<Response<SearchMarketplaceResponse>, Status> {
        let req = request.into_inner();
        let context = self.create_context(Some(req.trace_id.clone())).await;

        {
            let stub = self.stub.lock().await;
            stub.log_request(&context, "SearchMarketplace", "gRPC");
        }

        self.simulate_latency_and_errors(&context, "SearchMarketplace")
            .await?;

        let response = SearchMarketplaceResponse {
            result: Some(search_marketplace_response::Result::Success(
                SearchMarketplaceSuccess {
                    listings: vec![], // Empty for now
                    pagination: req.pagination,
                    search_suggestions: vec!["weapon".to_string(), "armor".to_string()],
                    facet_counts: std::collections::HashMap::new(),
                },
            )),
        };

        Ok(Response::new(response))
    }

    async fn get_nft_details(
        &self,
        request: Request<GetNftDetailsRequest>,
    ) -> Result<Response<GetNftDetailsResponse>, Status> {
        let req = request.into_inner();
        let context = self.create_context(Some(req.trace_id.clone())).await;

        {
            let stub = self.stub.lock().await;
            stub.log_request(&context, "GetNftDetails", "gRPC");
        }

        // Check crypto features
        if !context.enable_crypto {
            return Err(Status::permission_denied(
                "NFT features not enabled in MVE mode",
            ));
        }

        self.simulate_latency_and_errors(&context, "GetNftDetails")
            .await?;

        let nft_details = NftDetailsResponseProto {
            nft_details: Some(bunkerverse::core::v1::NftDetailsProto {
                nft_id: req.nft_id.clone(),
                token_id: "1".to_string(),
                contract_address: "0xabcdef1234567890".to_string(),
                name: "Mock NFT".to_string(),
                description: "A mock NFT for testing".to_string(),
                image_url: "https://example.com/nft.png".to_string(),
                metadata_json: r#"{"attributes":[{"trait_type":"rarity","value":"epic"},{"trait_type":"level","value":"5"}]}"#.to_string(),
                item_type: bunkerverse::core::v1::ItemTypeProto::Weapon as i32,
                item_rarity: bunkerverse::core::v1::ItemRarityProto::Epic as i32,
                item_condition: bunkerverse::core::v1::ItemConditionProto::Excellent as i32,
                mint_timestamp: Utc::now().timestamp(),
                creator_address: "0x1234567890abcdef".to_string(),
            }),
            nft_state: Some(bunkerverse::core::v1::NftMutableStateProto {
                nft_id: req.nft_id.clone(),
                current_owner_id: "player_123".to_string(),
                market_status: bunkerverse::core::v1::MarketStatusProto::Owned as i32,
                equipped_robot_id: Some("robot_001".to_string()),
                last_transaction_hash: Some("0xmocktxhash123".to_string()),
                last_updated_timestamp: Utc::now().timestamp(),
            }),
            price_history: vec![],
            market_analytics: Some(MarketAnalyticsProto {
                floor_price_wei: 500000000000000000u64,
                average_price_7d_wei: 750000000000000000u64,
                average_price_30d_wei: 800000000000000000u64,
                total_sales_7d: 15,
                total_sales_30d: 50,
                recent_sales: vec![],
            }),
            metadata_json: r#"{"name":"Mock NFT","description":"A mock NFT for testing","image":"https://example.com/nft.png","attributes":[{"trait_type":"rarity","value":"epic"}]}"#.to_string(),
        };

        let response = GetNftDetailsResponse {
            result: Some(get_nft_details_response::Result::Nft(nft_details)),
        };

        Ok(Response::new(response))
    }

    async fn get_player_owned_nfts(
        &self,
        request: Request<GetPlayerOwnedNftsRequest>,
    ) -> Result<Response<GetPlayerOwnedNftsResponse>, Status> {
        let req = request.into_inner();
        let context = self.create_context(Some(req.trace_id.clone())).await;

        {
            let stub = self.stub.lock().await;
            stub.log_request(&context, "GetPlayerOwnedNfts", "gRPC");
        }

        self.simulate_latency_and_errors(&context, "GetPlayerOwnedNfts")
            .await?;

        let owned_nfts = if context.enable_crypto {
            vec![PlayerOwnedNftProto {
                nft_details: Some(bunkerverse::core::v1::NftDetailsProto {
                    nft_id: "nft_player_001".to_string(),
                    token_id: "42".to_string(),
                    contract_address: "0xabcdef1234567890".to_string(),
                    name: "Player's Epic Weapon".to_string(),
                    description: "A legendary weapon NFT owned by the player".to_string(),
                    image_url: "https://example.com/player-weapon.png".to_string(),
                    metadata_json: "{}".to_string(),
                    item_type: bunkerverse::core::v1::ItemTypeProto::Weapon as i32,
                    item_rarity: bunkerverse::core::v1::ItemRarityProto::Epic as i32,
                    item_condition: bunkerverse::core::v1::ItemConditionProto::Excellent as i32,
                    mint_timestamp: Utc::now().timestamp(),
                    creator_address: "0x1234567890abcdef".to_string(),
                }),
                nft_state: Some(bunkerverse::core::v1::NftMutableStateProto {
                    nft_id: "nft_player_001".to_string(),
                    current_owner_id: req.player_id.clone(),
                    market_status: bunkerverse::core::v1::MarketStatusProto::Owned as i32,
                    equipped_robot_id: Some("robot_001".to_string()),
                    last_transaction_hash: Some("0xmocktxhash456".to_string()),
                    last_updated_timestamp: Utc::now().timestamp(),
                }),
                acquired_at: (Utc::now() - chrono::Duration::days(30)).timestamp(),
                acquisition_method: "purchase".to_string(),
                is_equipped: true,
                estimated_market_value_wei: 1200000000000000000u64,
            }]
        } else {
            vec![] // Empty for MVE mode
        };

        let response = GetPlayerOwnedNftsResponse {
            result: Some(get_player_owned_nfts_response::Result::Success(
                GetPlayerOwnedNftsSuccess {
                    owned_nfts,
                    pagination: req.pagination,
                    inventory_stats: Some(InventoryStatsProto {
                        total_nfts: if context.enable_crypto { 1 } else { 0 },
                        estimated_total_value_wei: if context.enable_crypto {
                            1200000000000000000u64
                        } else {
                            0
                        },
                        item_type_counts: std::collections::HashMap::new(),
                        rarity_counts: std::collections::HashMap::new(),
                        equipped_items: if context.enable_crypto { 1 } else { 0 },
                        marketable_items: if context.enable_crypto { 1 } else { 0 },
                    }),
                },
            )),
        };

        Ok(Response::new(response))
    }

    async fn create_listing(
        &self,
        request: Request<CreateListingRequest>,
    ) -> Result<Response<CreateListingResponse>, Status> {
        let req = request.into_inner();
        let context = self.create_context(Some(req.trace_id.clone())).await;

        {
            let stub = self.stub.lock().await;
            stub.log_request(&context, "CreateListing", "gRPC");

            // Check crypto features for blockchain operations
            if let Err(err) = stub.check_crypto_features(&context) {
                return Err(Status::permission_denied(err));
            }
        }

        self.simulate_latency_and_errors(&context, "CreateListing")
            .await?;

        let response = CreateListingResponse {
            result: Some(create_listing_response::Result::Success(
                CreateListingSuccess {
                    listing_id: Uuid::new_v4().to_string(),
                    transaction_hash: if context.enable_crypto {
                        "0xmocktxhash12345".to_string()
                    } else {
                        "".to_string()
                    },
                    transaction_status: bunkerverse::core::v1::TransactionStatusProto::Pending
                        as i32,
                },
            )),
        };

        Ok(Response::new(response))
    }

    async fn cancel_listing(
        &self,
        request: Request<CancelListingRequest>,
    ) -> Result<Response<CancelListingResponse>, Status> {
        let req = request.into_inner();
        let context = self.create_context(Some(req.trace_id.clone())).await;

        {
            let stub = self.stub.lock().await;
            stub.log_request(&context, "CancelListing", "gRPC");
        }

        self.simulate_latency_and_errors(&context, "CancelListing")
            .await?;

        let response = CancelListingResponse {
            result: Some(cancel_listing_response::Result::Success(
                CancelListingSuccess {
                    cancelled: true,
                    transaction_hash: if context.enable_crypto {
                        "0xmocktxhash67890".to_string()
                    } else {
                        "".to_string()
                    },
                    transaction_status: bunkerverse::core::v1::TransactionStatusProto::Pending
                        as i32,
                },
            )),
        };

        Ok(Response::new(response))
    }

    async fn execute_trade_intent(
        &self,
        request: Request<ExecuteTradeIntentRequest>,
    ) -> Result<Response<ExecuteTradeIntentResponse>, Status> {
        let req = request.into_inner();
        let context = self.create_context(Some(req.trace_id.clone())).await;

        {
            let stub = self.stub.lock().await;
            stub.log_request(&context, "ExecuteTradeIntent", "gRPC");

            // Check crypto features for blockchain operations
            if let Err(err) = stub.check_crypto_features(&context) {
                return Err(Status::permission_denied(err));
            }
        }

        self.simulate_latency_and_errors(&context, "ExecuteTradeIntent")
            .await?;

        let response = ExecuteTradeIntentResponse {
            result: Some(execute_trade_intent_response::Result::Success(
                ExecuteTradeIntentSuccess {
                    transaction_hash: "0xmocktxhash11111".to_string(),
                    transaction_status: bunkerverse::core::v1::TransactionStatusProto::Pending
                        as i32,
                    final_price_paid_wei: req.offered_price_ntc_wei,
                    marketplace_fee_wei: req.offered_price_ntc_wei / 40, // 2.5% fee
                },
            )),
        };

        Ok(Response::new(response))
    }

    async fn submit_transaction(
        &self,
        request: Request<bunkerverse::core::v1::TransactionRequestProto>,
    ) -> Result<Response<bunkerverse::core::v1::TransactionReceiptProto>, Status> {
        let req = request.into_inner();
        let context = self.create_context(req.trace_id.clone()).await;

        {
            let stub = self.stub.lock().await;
            stub.log_request(&context, "SubmitTransaction", "gRPC");
        }

        self.simulate_latency_and_errors(&context, "SubmitTransaction")
            .await?;

        let response = bunkerverse::core::v1::TransactionReceiptProto {
            transaction_hash: "0xmocktxhash99999".to_string(),
            status: bunkerverse::core::v1::TransactionStatusProto::Confirmed as i32,
            block_number: 12345,
            transaction_index: 0,
            gas_used: 21000,
            gas_price: req.max_gas_price.unwrap_or(20000000000),
            confirmation_count: 1,
            transaction_timestamp: Utc::now().timestamp(),
            events: vec![],
        };

        Ok(Response::new(response))
    }

    async fn get_transaction_receipt(
        &self,
        request: Request<GetTransactionReceiptRequest>,
    ) -> Result<Response<bunkerverse::core::v1::TransactionReceiptProto>, Status> {
        let req = request.into_inner();
        let context = self.create_context(Some(req.trace_id.clone())).await;

        {
            let stub = self.stub.lock().await;
            stub.log_request(&context, "GetTransactionReceipt", "gRPC");
        }

        self.simulate_latency_and_errors(&context, "GetTransactionReceipt")
            .await?;

        let response = bunkerverse::core::v1::TransactionReceiptProto {
            transaction_hash: req.transaction_hash,
            status: bunkerverse::core::v1::TransactionStatusProto::Confirmed as i32,
            block_number: 12345,
            transaction_index: 0,
            gas_used: 21000,
            gas_price: 20000000000,
            confirmation_count: 6,
            transaction_timestamp: Utc::now().timestamp(),
            events: vec![],
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
            service_name: "marketplace-service".to_string(),
            version: "0.1.0".to_string(),
            details: std::collections::HashMap::new(),
        };

        Ok(Response::new(response))
    }
}
