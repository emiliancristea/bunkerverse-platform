use crate::config::StubConfiguration;
use crate::stub::{MarketplaceStub, RequestContext, SmartStub};
use anyhow::Result;
use chrono::Utc;
use std::sync::Arc;
use tonic::{Request, Response, Status};
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

// Import all necessary types from generated protobuf modules
use bunkerverse::services::v1::{
    cancel_listing_response, create_listing_response, execute_trade_intent_response,
    get_listing_details_response, get_market_listings_response, get_nft_details_response,
    get_player_owned_nfts_response, marketplace_service_server, search_marketplace_response,
    CancelListingRequest, CancelListingResponse, CancelListingSuccess, CreateListingRequest,
    CreateListingResponse, CreateListingSuccess, ExecuteTradeIntentRequest,
    ExecuteTradeIntentResponse, ExecuteTradeIntentSuccess, GetListingDetailsRequest,
    GetListingDetailsResponse, GetMarketListingsRequest, GetMarketListingsResponse,
    GetMarketListingsSuccess, GetNftDetailsRequest, GetNftDetailsResponse,
    GetPlayerOwnedNftsRequest, GetPlayerOwnedNftsResponse, GetPlayerOwnedNftsSuccess,
    GetTransactionReceiptRequest, HealthRequest, HealthResponse, InventoryStatsProto,
    MarketAnalyticsProto, MarketListingDetailProto, MarketListingProto, MarketStatsProto,
    NftDetailsResponseProto, PlayerOwnedNftProto, SearchMarketplaceRequest,
    SearchMarketplaceResponse, SearchMarketplaceSuccess,
};

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

    fn create_mock_nft_details(
        nft_id: &str,
        enable_crypto: bool,
    ) -> bunkerverse::core::v1::NftDetailsProto {
        bunkerverse::core::v1::NftDetailsProto {
            identifier: Some(bunkerverse::core::v1::NftIdentifierProto {
                nft_id: nft_id.to_string(),
                token_id: 1,
                contract_address: if enable_crypto {
                    "0xabcdef1234567890".to_string()
                } else {
                    "".to_string()
                },
            }),
            item_type: bunkerverse::core::v1::ItemTypeProto::Head as i32,
            item_rarity: bunkerverse::core::v1::ItemRarityProto::Advanced as i32,
            base_stat_boosts: Some(bunkerverse::core::v1::CoreStatsProto {
                damage: 100,
                accuracy: 80,
                critical_chance: 20,
                armor_piercing: 30,
                speed: 50,
                agility: 60,
                stealth: 40,
                evasion: 70,
                health: 200,
                shield: 150,
                detection: 80,
                range: 90,
                combat_average: 57,
                mobility_average: 55,
                survivability_average: 175,
                sensors_average: 85,
            }),
            class_affinities: vec![bunkerverse::core::v1::BunkerClassProto::Explorer as i32],
            trait_affiliation: bunkerverse::core::v1::ClassAffiliationProto::Loyal as i32,
            construct_origin: "Factory Alpha".to_string(),
            metadata_pointer_uri: "Qm123456789abcdefghijklmnopqrstuvwxyzABCDEFGH".to_string(),
            schema_version: 1,
            created_timestamp: Utc::now().timestamp(),
        }
    }

    fn create_mock_nft_state(
        player_id: &str,
        enable_crypto: bool,
    ) -> bunkerverse::core::v1::NftMutableStateProto {
        bunkerverse::core::v1::NftMutableStateProto {
            current_owner_id: player_id.to_string(),
            current_condition: bunkerverse::core::v1::ItemConditionProto::NewState as i32,
            is_soulbound: false,
            market_status: bunkerverse::core::v1::MarketStatusProto::ListedForSale as i32,
            market_price_ntc: if enable_crypto {
                1000000000000000000u64
            } else {
                100
            },
            last_updated_timestamp: Utc::now().timestamp(),
        }
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
                nft_details: Some(Self::create_mock_nft_details("nft_001", true)),
                nft_state: Some(Self::create_mock_nft_state("player_123", true)),
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
                nft_details: Some(Self::create_mock_nft_details("item_001", false)),
                nft_state: Some(Self::create_mock_nft_state("player_456", false)),
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
                nft_details: Some(Self::create_mock_nft_details(
                    "nft_001",
                    context.enable_crypto,
                )),
                nft_state: Some(Self::create_mock_nft_state(
                    "player_123",
                    context.enable_crypto,
                )),
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
            nft_details: Some(Self::create_mock_nft_details(&req.nft_id, true)),
            nft_state: Some(bunkerverse::core::v1::NftMutableStateProto {
                current_owner_id: "player_123".to_string(),
                current_condition: bunkerverse::core::v1::ItemConditionProto::NewState as i32,
                is_soulbound: false,
                market_status: bunkerverse::core::v1::MarketStatusProto::NotListed as i32,
                market_price_ntc: 0,
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
            metadata_json: r#"{"name":"Mock NFT","description":"A mock NFT for testing","image":"https://example.com/nft.png","attributes":[{"trait_type":"rarity","value":"advanced"}]}"#.to_string(),
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
                nft_details: Some(Self::create_mock_nft_details("nft_player_001", true)),
                nft_state: Some(bunkerverse::core::v1::NftMutableStateProto {
                    current_owner_id: req.player_id.clone(),
                    current_condition: bunkerverse::core::v1::ItemConditionProto::NewState as i32,
                    is_soulbound: false,
                    market_status: bunkerverse::core::v1::MarketStatusProto::NotListed as i32,
                    market_price_ntc: 0,
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
        let context = self.create_context(Some(req.trace_id.clone())).await;

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
            gas_used: 21000,
            error_message: "".to_string(),
            emitted_events: vec![],
            confirmation_timestamp: Utc::now().timestamp(),
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
            gas_used: 21000,
            error_message: "".to_string(),
            emitted_events: vec![],
            confirmation_timestamp: Utc::now().timestamp(),
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
            version: "0.1.0".to_string(),
            timestamp: Utc::now().timestamp(),
            details: std::collections::HashMap::new(),
        };

        Ok(Response::new(response))
    }
}
