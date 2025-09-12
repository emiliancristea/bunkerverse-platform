use crate::config::StubConfiguration;
use crate::stub::{AiDataStub, RequestContext, SmartStub};
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

pub struct AiDataGrpcService {
    stub: Arc<tokio::sync::Mutex<AiDataStub>>,
}

impl AiDataGrpcService {
    pub fn new(config: StubConfiguration) -> Self {
        Self {
            stub: Arc::new(tokio::sync::Mutex::new(AiDataStub::new(config))),
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
            return Err(Status::internal("Simulated AI Data service gRPC error"));
        }

        stub.log_response(context, method, latency.as_millis() as u64, 200, false);
        Ok(())
    }
}

#[tonic::async_trait]
impl ai_data_service_server::AiDataService for AiDataGrpcService {
    async fn get_ai_agent_input_data_context(
        &self,
        request: Request<GetAiAgentInputDataContextRequest>,
    ) -> Result<Response<GetAiAgentInputDataContextResponse>, Status> {
        let req = request.into_inner();
        let context = self.create_context(Some(req.trace_id.clone())).await;

        {
            let stub = self.stub.lock().await;
            stub.log_request(&context, "GetAIAgentInputDataContext", "gRPC");
        }

        self.simulate_latency_and_errors(&context, "GetAIAgentInputDataContext")
            .await?;

        // Create mock AI agent input data context
        let ai_context = AiAgentInputDataContextProto {
            context_id: Uuid::new_v4().to_string(),
            player_id: req.player_id.clone(),
            context_type: req.context_type,
            player_context: Some(PlayerContextProto {
                player_id: req.player_id.clone(),
                bunker_tag: "MockPlayer".to_string(),
                player_level: 25,
                total_xp: 125000,
                hours_played: 45,
                recent_achievements: vec!["Veteran Explorer".to_string(), "Combat Master".to_string()],
                recent_missions: vec!["Mission Alpha".to_string(), "Mission Beta".to_string()],
                last_activity: "Completed advanced combat training".to_string(),
                playstyle: Some(PlaystyleProfileProto {
                    primary_playstyle: "Explorer".to_string(),
                    secondary_playstyles: vec!["Combat".to_string(), "Social".to_string()],
                    combat_focus: 0.7,
                    exploration_focus: 0.9,
                    social_focus: 0.4,
                    collection_focus: 0.6,
                    session_length_hours: 2.5,
                    sessions_per_week: 4,
                    preferred_difficulty: "Normal".to_string(),
                    prefers_solo_play: true,
                }),
                narrative_prefs: Some(NarrativePreferencesProto {
                    preferred_tones: vec![1, 6], // Heroic, Adventurous
                    preferred_styles: vec![5, 8], // Cinematic, Narrative Story
                    preferred_themes: vec!["exploration".to_string(), "discovery".to_string()],
                    avoided_themes: vec!["horror".to_string()],
                    preferred_length_words: 200,
                    preferred_perspective: "second_person".to_string(),
                    include_dialogue: true,
                    include_action_sequences: true,
                    use_player_name: false,
                    use_robot_name: true,
                    immersion_level: 0.8,
                    reference_past_events: true,
                    filter_mature_content: true,
                    filter_violence: false,
                    content_warnings: vec![],
                    last_updated: Utc::now().timestamp(),
                }),
            }),
            robot_context: Some(RobotContextProto {
                robot_id: "robot_001".to_string(),
                robot_name: "Atlas".to_string(),
                robot_class: "Scout".to_string(),
                robot_affiliation: "Freelancer".to_string(),
                robot_level: 20,
                stats: Some(bunkerverse::core::v1::CoreStatsProto {
                    health: 100,
                    attack: 85,
                    defense: 70,
                    speed: 90,
                    energy: 80,
                }),
                equipped_items: vec!["Energy Rifle".to_string(), "Shield Generator".to_string()],
                special_abilities: vec!["Stealth Mode".to_string(), "Energy Boost".to_string()],
                robot_personality: Some("Analytical and tactical".to_string()),
                robot_quirks: vec!["Always scans environment twice".to_string()],
            }),
            mission_context: vec![
                MissionContextProto {
                    mission_id: "mission_001".to_string(),
                    mission_title: "Exploration Alpha".to_string(),
                    mission_type: "exploration".to_string(),
                    completion_status: "completed".to_string(),
                    objectives: vec!["Survey the northern sector".to_string(), "Collect mineral samples".to_string()],
                    difficulty_level: "Normal".to_string(),
                    rewards: vec!["1000 XP".to_string(), "Rare Minerals".to_string()],
                    completion_time_hours: Some(1.5),
                    attempts_count: Some(1),
                    performance_rating: Some("Excellent".to_string()),
                }
            ],
            inventory_context: vec![
                InventoryContextProto {
                    item_id: "item_001".to_string(),
                    item_name: "Advanced Scanner".to_string(),
                    item_type: "tool".to_string(),
                    item_rarity: "rare".to_string(),
                    is_equipped: true,
                    is_favorite: true,
                    acquisition_method: "mission_reward".to_string(),
                    acquisition_timestamp: (Utc::now() - chrono::Duration::days(3)).timestamp(),
                    stat_bonuses: Some(bunkerverse::core::v1::CoreStatsProto {
                        health: 0,
                        attack: 0,
                        defense: 5,
                        speed: 0,
                        energy: 10,
                    }),
                    special_properties: vec!["Enhanced Detection Range".to_string()],
                }
            ],
            progress_context: Some(GameProgressContextProto {
                completion_percentage: 35,
                main_story_progress: 25,
                side_missions_completed: 15,
                areas_explored: 8,
                major_milestones: vec!["First Robot Acquired".to_string(), "Veteran Explorer".to_string()],
                unlocked_features: vec!["Advanced Missions".to_string(), "Trading System".to_string()],
                current_focus_area: "Exploration and Discovery".to_string(),
                achievements_earned: 12,
                rare_achievements: vec!["Master Explorer".to_string()],
                next_achievement_goal: "Combat Specialist".to_string(),
            }),
            recent_narratives: vec!["Your latest exploration yielded fascinating discoveries...".to_string()],
            player_preferences: Some(NarrativePreferencesProto {
                preferred_tones: vec![1, 6], // Heroic, Adventurous
                preferred_styles: vec![5, 8], // Cinematic, Narrative Story
                preferred_themes: vec!["exploration".to_string(), "discovery".to_string()],
                avoided_themes: vec!["horror".to_string()],
                preferred_length_words: 200,
                preferred_perspective: "second_person".to_string(),
                include_dialogue: true,
                include_action_sequences: true,
                use_player_name: false,
                use_robot_name: true,
                immersion_level: 0.8,
                reference_past_events: true,
                filter_mature_content: true,
                filter_violence: false,
                content_warnings: vec![],
                last_updated: Utc::now().timestamp(),
            }),
            relevant_lore: vec![
                LoreEntryProto {
                    lore_id: "lore_001".to_string(),
                    lore_category: "world_history".to_string(),
                    lore_title: "The Great Discovery".to_string(),
                    lore_content: "Ancient artifacts scattered across the northern regions tell a story of advanced civilization...".to_string(),
                    lore_importance: 8,
                    related_lore_ids: vec!["lore_002".to_string()],
                    relevant_missions: vec!["mission_001".to_string()],
                    relevant_characters: vec!["Dr. Vex".to_string()],
                    is_unlocked: true,
                    unlock_trigger: "mission_completion".to_string(),
                    unlock_timestamp: (Utc::now() - chrono::Duration::days(2)).timestamp(),
                }
            ],
            world_context: Some(WorldStateContextProto {
                current_season: "Discovery Season".to_string(),
                active_events: vec!["Northern Expedition".to_string(), "Mining Festival".to_string()],
                world_mood: "optimistic".to_string(),
                recent_world_changes: vec!["New trading routes opened".to_string()],
                ntc_economy_state: 1.2,
                trending_items: vec!["Exploration Gear".to_string(), "Mining Equipment".to_string()],
                market_sentiment: "bullish".to_string(),
                popular_activities: vec!["Exploration".to_string(), "Trading".to_string()],
                community_achievements: vec!["Collective Mining Goal Reached".to_string()],
                social_trends: "Collaborative exploration".to_string(),
            }),
            context_token_count: 1500,
            context_relevance_score: 0.92,
            context_generated_timestamp: Utc::now().timestamp(),
            context_version: 1,
        };

        let response = GetAiAgentInputDataContextResponse {
            result: Some(get_ai_agent_input_data_context_response::Result::Context(
                ai_context,
            )),
        };

        Ok(Response::new(response))
    }

    async fn get_personalized_narrative_context(
        &self,
        request: Request<GetPersonalizedNarrativeContextRequest>,
    ) -> Result<Response<GetPersonalizedNarrativeContextResponse>, Status> {
        let req = request.into_inner();
        let context = self.create_context(Some(req.trace_id.clone())).await;

        {
            let stub = self.stub.lock().await;
            stub.log_request(&context, "GetPersonalizedNarrativeContext", "gRPC");
        }

        self.simulate_latency_and_errors(&context, "GetPersonalizedNarrativeContext")
            .await?;

        let personalized_context = PersonalizedNarrativeContextProto {
            context_id: Uuid::new_v4().to_string(),
            player_id: req.player_id.clone(),
            narrative_theme: req.narrative_theme.clone(),
            tone: req.desired_tone,
            player_personality: Some(PlayerPersonalityProto {
                exploration_preference: 0.9,
                competition_level: 0.6,
                social_engagement: 0.4,
                risk_tolerance: 0.7,
                creativity_score: 0.8,
                favorite_activities: vec!["exploration".to_string(), "discovery".to_string()],
                preferred_playtime: "evening".to_string(),
                decision_making_style: "analytical".to_string(),
                motivation_factors: vec!["discovery".to_string(), "achievement".to_string()],
                preferred_themes: vec!["adventure".to_string(), "mystery".to_string()],
                preferred_complexity: "moderate".to_string(),
                likes_humor: true,
                likes_drama: false,
            }),
            player_achievements: vec!["Master Explorer".to_string(), "Veteran Scout".to_string()],
            favorite_activities: vec!["Exploration".to_string(), "Trading".to_string()],
            playstyle: Some(PlaystyleProfileProto {
                primary_playstyle: "Explorer".to_string(),
                secondary_playstyles: vec!["Trader".to_string()],
                combat_focus: 0.5,
                exploration_focus: 0.95,
                social_focus: 0.3,
                collection_focus: 0.7,
                session_length_hours: 2.5,
                sessions_per_week: 4,
                preferred_difficulty: "Normal".to_string(),
                prefers_solo_play: true,
            }),
            background_setting: "The vast frontier awaits your exploration, with ancient mysteries hidden in every sector...".to_string(),
            characters: vec![
                CharacterContextProto {
                    character_id: "char_001".to_string(),
                    character_name: "Dr. Elena Vex".to_string(),
                    character_role: "Research Director".to_string(),
                    character_description: "A brilliant scientist obsessed with ancient discoveries".to_string(),
                    character_traits: vec!["Intelligent".to_string(), "Curious".to_string(), "Impatient".to_string()],
                    relationship_to_player: "Mission Coordinator".to_string(),
                    notable_interactions: vec!["Assigned first exploration mission".to_string()],
                    character_arc: "Gradually reveals deeper mysteries".to_string(),
                    is_major_character: true,
                }
            ],
            recent_events: vec![
                EventContextProto {
                    event_id: "event_001".to_string(),
                    event_name: "Ancient Artifact Discovery".to_string(),
                    event_type: "discovery".to_string(),
                    event_description: "Player discovered ancient artifacts in northern sector".to_string(),
                    event_participants: vec![req.player_id.clone(), "Dr. Vex".to_string()],
                    event_outcome: "successful_discovery".to_string(),
                    narrative_importance: 0.8,
                    consequences: vec!["New research opportunities".to_string(), "Increased reputation".to_string()],
                    event_timestamp: (Utc::now() - chrono::Duration::days(1)).timestamp(),
                }
            ],
            narrative_framework: "Second-person adventure narrative with emphasis on discovery and wonder".to_string(),
            suggested_plot_points: vec![
                "Reference recent artifact discovery".to_string(),
                "Hint at larger mysteries".to_string(),
                "Acknowledge player's exploration skills".to_string()
            ],
            template_variables: HashMap::from([
                ("player_achievement".to_string(), "Master Explorer".to_string()),
                ("recent_discovery".to_string(), "Ancient Artifacts".to_string()),
                ("robot_name".to_string(), "Atlas".to_string())
            ]),
            estimated_token_count: 800,
            generated_timestamp: Utc::now().timestamp(),
        };

        let response = GetPersonalizedNarrativeContextResponse {
            result: Some(
                get_personalized_narrative_context_response::Result::Context(personalized_context),
            ),
        };

        Ok(Response::new(response))
    }

    async fn get_mission_briefing_context(
        &self,
        request: Request<GetMissionBriefingContextRequest>,
    ) -> Result<Response<GetMissionBriefingContextResponse>, Status> {
        let req = request.into_inner();
        let context = self.create_context(Some(req.trace_id.clone())).await;

        {
            let stub = self.stub.lock().await;
            stub.log_request(&context, "GetMissionBriefingContext", "gRPC");
        }

        self.simulate_latency_and_errors(&context, "GetMissionBriefingContext")
            .await?;

        let briefing_context = MissionBriefingContextProto {
            context_id: Uuid::new_v4().to_string(),
            mission_id: req.mission_id.clone(),
            player_id: req.player_id.clone(),
            mission_title: "Deep Sector Survey".to_string(),
            mission_type: "exploration".to_string(),
            mission_difficulty: "Challenging".to_string(),
            mission_objectives: vec![
                "Survey the deep sector installations".to_string(),
                "Collect energy readings from core systems".to_string(),
                "Document any anomalous activity".to_string()
            ],
            player_robot: Some(RobotContextProto {
                robot_id: "robot_001".to_string(),
                robot_name: "Atlas".to_string(),
                robot_class: "Scout".to_string(),
                robot_affiliation: "Freelancer".to_string(),
                robot_level: 20,
                stats: Some(bunkerverse::core::v1::CoreStatsProto {
                    health: 100,
                    attack: 85,
                    defense: 70,
                    speed: 90,
                    energy: 80,
                }),
                equipped_items: vec!["Advanced Scanner".to_string(), "Energy Shield".to_string()],
                special_abilities: vec!["Enhanced Sensors".to_string(), "Stealth Mode".to_string()],
                robot_personality: Some("Methodical and thorough".to_string()),
                robot_quirks: vec!["Always double-checks sensor readings".to_string()],
            }),
            equipped_items: vec!["Advanced Scanner".to_string(), "Emergency Beacon".to_string()],
            player_readiness_assessment: "Well-equipped for exploration with high-quality scanning gear".to_string(),
            strategy_suggestions: if req.include_strategy_hints {
                vec![
                    "Use stealth mode to avoid detection".to_string(),
                    "Prioritize energy readings near core systems".to_string(),
                    "Document everything - anomalies may be significant".to_string()
                ]
            } else {
                vec![]
            },
            known_threats: vec!["Automated defense systems".to_string(), "Unstable energy fields".to_string()],
            recommended_preparations: vec![
                "Charge energy systems to full capacity".to_string(),
                "Calibrate scanning equipment".to_string(),
                "Review sector maps".to_string()
            ],
            mission_backstory: if req.include_lore_background {
                "The deep sector installations were abandoned decades ago during the Great Withdrawal. Recent energy signatures suggest something may still be active down there...".to_string()
            } else {
                "Survey mission to assess current status of deep sector installations.".to_string()
            },
            relevant_lore_entries: if req.include_lore_background {
                vec!["The Great Withdrawal historical records".to_string(), "Deep Sector Installation Blueprints".to_string()]
            } else {
                vec![]
            },
            environmental_description: "Dark corridors lit only by emergency lighting, with the constant hum of distant machinery echoing through empty halls.".to_string(),
            briefing_tone: req.briefing_tone,
            include_spoilers: false,
            briefing_style: "Professional mission briefing with tactical focus".to_string(),
            generated_timestamp: Utc::now().timestamp(),
        };

        let response = GetMissionBriefingContextResponse {
            result: Some(get_mission_briefing_context_response::Result::Context(
                briefing_context,
            )),
        };

        Ok(Response::new(response))
    }

    async fn get_combat_narrative_context(
        &self,
        request: Request<GetCombatNarrativeContextRequest>,
    ) -> Result<Response<GetCombatNarrativeContextResponse>, Status> {
        let req = request.into_inner();
        let context = self.create_context(Some(req.trace_id.clone())).await;

        {
            let stub = self.stub.lock().await;
            stub.log_request(&context, "GetCombatNarrativeContext", "gRPC");
        }

        self.simulate_latency_and_errors(&context, "GetCombatNarrativeContext")
            .await?;

        let combat_context = CombatNarrativeContextProto {
            context_id: Uuid::new_v4().to_string(),
            combat_session_id: req.combat_session_id.clone(),
            player_id: req.player_id.clone(),
            player_combatant: Some(CombatParticipantProto {
                participant_id: req.player_id.clone(),
                participant_name: "Player".to_string(),
                participant_type: "player".to_string(),
                combat_stats: Some(bunkerverse::core::v1::CoreStatsProto {
                    health: 85,
                    attack: 75,
                    defense: 65,
                    speed: 80,
                    energy: 70,
                }),
                health_points: 85,
                max_health_points: 100,
                equipped_weapons: vec!["Energy Rifle".to_string(), "Plasma Blade".to_string()],
                active_abilities: vec!["Shield Boost".to_string(), "Rapid Fire".to_string()],
                combat_style: "Balanced offense and defense".to_string(),
            }),
            enemies: if req.include_enemy_descriptions {
                vec![CombatParticipantProto {
                    participant_id: "enemy_001".to_string(),
                    participant_name: "Security Drone".to_string(),
                    participant_type: "enemy".to_string(),
                    combat_stats: Some(bunkerverse::core::v1::CoreStatsProto {
                        health: 60,
                        attack: 70,
                        defense: 80,
                        speed: 90,
                        energy: 50,
                    }),
                    health_points: 0, // Defeated
                    max_health_points: 60,
                    equipped_weapons: vec!["Laser Array".to_string()],
                    active_abilities: vec!["Evasive Maneuvers".to_string()],
                    combat_style: "High-speed hit and run".to_string(),
                }]
            } else {
                vec![]
            },
            combat_events: req.combat_events,
            combat_outcome: Some(CombatOutcomeProto {
                outcome_type: "victory".to_string(),
                survivors: vec![req.player_id.clone()],
                casualties: vec!["enemy_001".to_string()],
                combat_duration_seconds: 45,
                victory_condition: "All enemies defeated".to_string(),
                key_moments: vec![
                    "Player used shield boost to absorb heavy damage".to_string(),
                    "Critical hit with plasma blade finished the engagement".to_string(),
                ],
                rewards_earned: vec![
                    "Combat XP: 500".to_string(),
                    "Salvage Materials".to_string(),
                ],
                penalties_incurred: vec![],
                narrative_conclusion:
                    "Swift and decisive victory through tactical use of abilities".to_string(),
            }),
            combat_location: if req.include_environment_details {
                "Abandoned facility corridor with flickering lights and debris".to_string()
            } else {
                "Facility corridor".to_string()
            },
            environmental_factors: if req.include_environment_details {
                vec![
                    "Poor lighting".to_string(),
                    "Narrow space limits movement".to_string(),
                    "Metal debris provides cover".to_string(),
                ]
            } else {
                vec![]
            },
            atmosphere_description: "Tense atmosphere with sparks flying from damaged systems"
                .to_string(),
            dramatic_moments: vec![
                "Shield boost activation in critical moment".to_string(),
                "Final decisive strike with plasma weapon".to_string(),
            ],
            tactical_descriptions: vec![
                "Effective use of cover and positioning".to_string(),
                "Well-timed ability usage".to_string(),
            ],
            combat_pacing: "Fast-paced with moments of tactical decision-making".to_string(),
            combat_duration_seconds: 45,
            combat_intensity_score: 0.75,
            generated_timestamp: Utc::now().timestamp(),
        };

        let response = GetCombatNarrativeContextResponse {
            result: Some(get_combat_narrative_context_response::Result::Context(
                combat_context,
            )),
        };

        Ok(Response::new(response))
    }

    async fn request_narrative_generation(
        &self,
        request: Request<RequestNarrativeGenerationRequest>,
    ) -> Result<Response<RequestNarrativeGenerationResponse>, Status> {
        let req = request.into_inner();
        let context = self.create_context(Some(req.trace_id.clone())).await;

        {
            let stub = self.stub.lock().await;
            stub.log_request(&context, "RequestNarrativeGeneration", "gRPC");
        }

        self.simulate_latency_and_errors(&context, "RequestNarrativeGeneration")
            .await?;

        let generation_success = RequestNarrativeGenerationSuccess {
            narrative_request_id: Uuid::new_v4().to_string(),
            estimated_completion_time: (Utc::now() + chrono::Duration::seconds(30)).to_rfc3339(),
            generation_started: true,
            websocket_url: if req.use_streaming {
                format!(
                    "wss://ai-data.bunkerverse.com/narrative-stream/{}",
                    context.request_id
                )
            } else {
                "".to_string()
            },
        };

        let response = RequestNarrativeGenerationResponse {
            result: Some(request_narrative_generation_response::Result::Success(
                generation_success,
            )),
        };

        Ok(Response::new(response))
    }

    async fn get_narrative_history(
        &self,
        request: Request<GetNarrativeHistoryRequest>,
    ) -> Result<Response<GetNarrativeHistoryResponse>, Status> {
        let req = request.into_inner();
        let context = self.create_context(Some(req.trace_id.clone())).await;

        {
            let stub = self.stub.lock().await;
            stub.log_request(&context, "GetNarrativeHistory", "gRPC");
        }

        self.simulate_latency_and_errors(&context, "GetNarrativeHistory")
            .await?;

        let mock_narratives = vec![
            GeneratedNarrativeProto {
                narrative_id: "narr_001".to_string(),
                player_id: req.player_id.clone(),
                request_id: "req_001".to_string(),
                narrative_text: "Your exploration of the northern sector yields fascinating discoveries. Ancient structures emerge from the mist, their purpose shrouded in mystery...".to_string(),
                narrative_type: 4, // EXPLORATION_LOG
                context_summary: "Exploration mission in northern sector with artifact discovery".to_string(),
                token_count: 45,
                generation_time_seconds: 2.3,
                quality_score: 0.87,
                relevance_score: 0.92,
                player_rating: Some(4.5),
                interactions: vec![],
                is_favorited: true,
                generation_model: "NAR-v2.1".to_string(),
                generation_params: HashMap::from([
                    ("temperature".to_string(), "0.8".to_string()),
                    ("max_tokens".to_string(), "200".to_string())
                ]),
                was_cached: false,
                generated_timestamp: (Utc::now() - chrono::Duration::days(2)).timestamp(),
                last_accessed_timestamp: (Utc::now() - chrono::Duration::hours(6)).timestamp(),
            }
        ];

        let history_success = GetNarrativeHistorySuccess {
            narratives: mock_narratives,
            pagination: req.pagination,
            history_stats: Some(NarrativeHistoryStatsProto {
                total_narratives: 15,
                overall_satisfaction_rating: 4.2,
                narratives_this_week: 3,
                narratives_this_month: 8,
                most_common_theme: "exploration".to_string(),
                highest_rated_type: "EXPLORATION_LOG".to_string(),
                total_words_generated: 2500,
                average_generation_time: 2.1,
            }),
        };

        let response = GetNarrativeHistoryResponse {
            result: Some(get_narrative_history_response::Result::Success(
                history_success,
            )),
        };

        Ok(Response::new(response))
    }

    async fn update_narrative_preferences(
        &self,
        request: Request<UpdateNarrativePreferencesRequest>,
    ) -> Result<Response<UpdateNarrativePreferencesResponse>, Status> {
        let req = request.into_inner();
        let context = self.create_context(Some(req.trace_id.clone())).await;

        {
            let stub = self.stub.lock().await;
            stub.log_request(&context, "UpdateNarrativePreferences", "gRPC");
        }

        self.simulate_latency_and_errors(&context, "UpdateNarrativePreferences")
            .await?;

        let update_success = UpdateNarrativePreferencesSuccess {
            updated_preferences: req.preferences,
            updated_timestamp: Utc::now().timestamp(),
        };

        let response = UpdateNarrativePreferencesResponse {
            result: Some(update_narrative_preferences_response::Result::Success(
                update_success,
            )),
        };

        Ok(Response::new(response))
    }

    async fn record_narrative_interaction(
        &self,
        request: Request<RecordNarrativeInteractionRequest>,
    ) -> Result<Response<RecordNarrativeInteractionResponse>, Status> {
        let req = request.into_inner();
        let context = self.create_context(Some(req.trace_id.clone())).await;

        {
            let stub = self.stub.lock().await;
            stub.log_request(&context, "RecordNarrativeInteraction", "gRPC");
        }

        self.simulate_latency_and_errors(&context, "RecordNarrativeInteraction")
            .await?;

        let interaction_success = RecordNarrativeInteractionSuccess {
            interaction_recorded: true,
            recorded_timestamp: Utc::now().timestamp(),
        };

        let response = RecordNarrativeInteractionResponse {
            result: Some(record_narrative_interaction_response::Result::Success(
                interaction_success,
            )),
        };

        Ok(Response::new(response))
    }

    async fn get_narrative_analytics(
        &self,
        request: Request<GetNarrativeAnalyticsRequest>,
    ) -> Result<Response<GetNarrativeAnalyticsResponse>, Status> {
        let req = request.into_inner();
        let context = self.create_context(Some(req.trace_id.clone())).await;

        {
            let stub = self.stub.lock().await;
            stub.log_request(&context, "GetNarrativeAnalytics", "gRPC");
        }

        self.simulate_latency_and_errors(&context, "GetNarrativeAnalytics")
            .await?;

        let analytics = NarrativeAnalyticsProto {
            player_id: req.player_id,
            analytics_period: req.time_period.unwrap_or("30d".to_string()),
            total_narratives_generated: 25,
            narratives_read_completely: 22,
            average_rating: 4.3,
            narratives_favorited: 8,
            theme_popularity: HashMap::from([
                ("exploration".to_string(), 12),
                ("combat".to_string(), 8),
                ("discovery".to_string(), 5),
            ]),
            tone_ratings: HashMap::from([
                ("heroic".to_string(), 4.5),
                ("adventurous".to_string(), 4.2),
                ("mysterious".to_string(), 3.8),
            ]),
            most_engaging_elements: vec![
                "discovery moments".to_string(),
                "character interactions".to_string(),
            ],
            least_engaging_elements: vec!["technical descriptions".to_string()],
            average_reading_time_seconds: 45.0,
            preferred_narrative_length: "medium".to_string(),
            peak_engagement_times: vec!["evening".to_string(), "weekend_afternoon".to_string()],
            narrative_coherence_score: 0.89,
            narrative_relevance_score: 0.92,
            generation_failures: 2,
            analytics_generated_timestamp: Utc::now().timestamp(),
        };

        let response = GetNarrativeAnalyticsResponse {
            result: Some(get_narrative_analytics_response::Result::Analytics(
                analytics,
            )),
        };

        Ok(Response::new(response))
    }

    async fn optimize_context_generation(
        &self,
        request: Request<OptimizeContextGenerationRequest>,
    ) -> Result<Response<OptimizeContextGenerationResponse>, Status> {
        let req = request.into_inner();
        let context = self.create_context(Some(req.trace_id.clone())).await;

        {
            let stub = self.stub.lock().await;
            stub.log_request(&context, "OptimizeContextGeneration", "gRPC");
        }

        self.simulate_latency_and_errors(&context, "OptimizeContextGeneration")
            .await?;

        let optimization = ContextOptimizationProto {
            optimization_id: Uuid::new_v4().to_string(),
            player_id: req.player_id,
            recommended_adjustments: vec![
                "Increase exploration context weight".to_string(),
                "Reduce combat detail level".to_string(),
                "Include more discovery elements".to_string(),
            ],
            estimated_improvement: 0.15,
            focus_areas: vec![
                "exploration achievements".to_string(),
                "discovery moments".to_string(),
            ],
            reduce_areas: vec![
                "combat statistics".to_string(),
                "technical details".to_string(),
            ],
            player_preferences_learned: vec![
                "Prefers discovery narratives".to_string(),
                "Values character development".to_string(),
            ],
            content_patterns: vec![
                "Second-person perspective works well".to_string(),
                "Moderate length preferred".to_string(),
            ],
            confidence_score: 0.82,
            optimization_timestamp: Utc::now().timestamp(),
        };

        let response = OptimizeContextGenerationResponse {
            result: Some(optimize_context_generation_response::Result::Optimization(
                optimization,
            )),
        };

        Ok(Response::new(response))
    }

    async fn get_lore_database(
        &self,
        request: Request<GetLoreDatabaseRequest>,
    ) -> Result<Response<GetLoreDatabaseResponse>, Status> {
        let req = request.into_inner();
        let context = self.create_context(Some(req.trace_id.clone())).await;

        {
            let stub = self.stub.lock().await;
            stub.log_request(&context, "GetLoreDatabase", "gRPC");
        }

        self.simulate_latency_and_errors(&context, "GetLoreDatabase")
            .await?;

        let lore_database = LoreDatabaseProto {
            database_version: "2.1.0".to_string(),
            lore_entries: vec![
                LoreEntryProto {
                    lore_id: "lore_001".to_string(),
                    lore_category: "world_history".to_string(),
                    lore_title: "The Great Discovery".to_string(),
                    lore_content: "In the early days of exploration, vast networks of ancient installations were discovered scattered across multiple sectors...".to_string(),
                    lore_importance: 9,
                    related_lore_ids: vec!["lore_002".to_string(), "lore_003".to_string()],
                    relevant_missions: vec!["mission_001".to_string()],
                    relevant_characters: vec!["Dr. Vex".to_string(), "Archive Keeper".to_string()],
                    is_unlocked: true,
                    unlock_trigger: "exploration_milestone".to_string(),
                    unlock_timestamp: (Utc::now() - chrono::Duration::days(5)).timestamp(),
                }
            ],
            lore_categories: HashMap::from([
                ("world_history".to_string(), "lore_001,lore_002".to_string()),
                ("technology".to_string(), "lore_003,lore_004".to_string())
            ]),
            recently_updated_entries: vec!["lore_001".to_string()],
            player_lore_progress_level: req.player_lore_progress_level,
            unlocked_entries: vec!["lore_001".to_string()],
            spoiler_entries: if req.include_spoiler_content { vec![] } else { vec!["lore_005".to_string()] },
        };

        let response = GetLoreDatabaseResponse {
            result: Some(get_lore_database_response::Result::LoreDatabase(
                lore_database,
            )),
        };

        Ok(Response::new(response))
    }

    async fn update_player_lore_progress(
        &self,
        request: Request<UpdatePlayerLoreProgressRequest>,
    ) -> Result<Response<UpdatePlayerLoreProgressResponse>, Status> {
        let req = request.into_inner();
        let context = self.create_context(Some(req.trace_id.clone())).await;

        {
            let stub = self.stub.lock().await;
            stub.log_request(&context, "UpdatePlayerLoreProgress", "gRPC");
        }

        self.simulate_latency_and_errors(&context, "UpdatePlayerLoreProgress")
            .await?;

        let update_success = UpdatePlayerLoreProgressSuccess {
            updated_progress: Some(PlayerLoreProgressProto {
                player_id: req.player_id,
                lore_progress_level: 15,
                unlocked_lore_ids: vec!["lore_001".to_string(), req.lore_entry_id.clone()],
                recent_unlocks: vec![LoreUnlockProto {
                    lore_id: req.lore_entry_id,
                    unlock_trigger: req.unlock_trigger,
                    unlock_timestamp: req.unlock_timestamp,
                    is_major_revelation: false,
                }],
                total_lore_entries: 50,
                unlocked_entries_count: 15,
                completion_percentage: 30.0,
                category_progress: HashMap::from([
                    ("world_history".to_string(), 8),
                    ("technology".to_string(), 5),
                ]),
                completed_categories: vec!["basic_mechanics".to_string()],
            }),
            newly_unlocked_entries: vec![], // No cascade unlocks in this example
        };

        let response = UpdatePlayerLoreProgressResponse {
            result: Some(update_player_lore_progress_response::Result::Success(
                update_success,
            )),
        };

        Ok(Response::new(response))
    }

    async fn get_context_templates(
        &self,
        request: Request<GetContextTemplatesRequest>,
    ) -> Result<Response<GetContextTemplatesResponse>, Status> {
        let req = request.into_inner();
        let context = self.create_context(Some(req.trace_id.clone())).await;

        {
            let stub = self.stub.lock().await;
            stub.log_request(&context, "GetContextTemplates", "gRPC");
        }

        self.simulate_latency_and_errors(&context, "GetContextTemplates")
            .await?;

        let templates_success = GetContextTemplatesSuccess {
            templates: vec![
                NarrativeTemplateProto {
                    template_id: "template_001".to_string(),
                    template_name: "Exploration Discovery".to_string(),
                    context_type: req.context_type,
                    template_structure: "As you explore {location}, your {robot_name} detects {discovery}. {player_reaction} The implications of this find could {consequence}.".to_string(),
                    required_variables: vec!["location".to_string(), "robot_name".to_string(), "discovery".to_string()],
                    optional_variables: vec!["player_reaction".to_string(), "consequence".to_string()],
                    example_usage: "As you explore the northern sector, your Atlas detects ancient energy signatures. Your pulse quickens with excitement. The implications of this find could reshape our understanding of the region.".to_string(),
                    effectiveness_score: 0.92,
                    usage_count: 1250,
                    compatible_tones: vec!["HEROIC".to_string(), "ADVENTUROUS".to_string(), "MYSTERIOUS".to_string()],
                }
            ],
            template_variables: HashMap::from([
                ("location".to_string(), "Current mission location".to_string()),
                ("robot_name".to_string(), "Player's active robot name".to_string()),
                ("discovery".to_string(), "What was discovered".to_string()),
                ("player_reaction".to_string(), "Player's emotional response".to_string()),
                ("consequence".to_string(), "Potential impact or significance".to_string())
            ]),
        };

        let response = GetContextTemplatesResponse {
            result: Some(get_context_templates_response::Result::Success(
                templates_success,
            )),
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
            service_name: "ai-data-service".to_string(),
            version: "0.1.0".to_string(),
            details: HashMap::new(),
        };

        Ok(Response::new(response))
    }
}
