use crate::config::StubConfiguration;
use crate::stub::{IdentityStub, RequestContext, SmartStub};
use anyhow::Result;
use chrono::Utc;
use std::{collections::HashMap, sync::Arc};
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

use bunkerverse::services::v1::*;

pub struct IdentityGrpcService {
    stub: Arc<tokio::sync::Mutex<IdentityStub>>,
}

impl IdentityGrpcService {
    pub fn new(config: StubConfiguration) -> Self {
        Self {
            stub: Arc::new(tokio::sync::Mutex::new(IdentityStub::new(config))),
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
            return Err(Status::internal("Simulated identity service gRPC error"));
        }

        stub.log_response(context, method, latency.as_millis() as u64, 200, false);
        Ok(())
    }
}

#[tonic::async_trait]
impl identity_service_server::IdentityService for IdentityGrpcService {
    async fn initiate_zk_login(
        &self,
        request: Request<InitiateZkLoginRequest>,
    ) -> Result<Response<InitiateZkLoginResponse>, Status> {
        let req = request.into_inner();
        let context = self.create_context(Some(req.trace_id.clone())).await;

        {
            let stub = self.stub.lock().await;
            stub.log_request(&context, "InitiateZkLogin", "gRPC");
        }

        self.simulate_latency_and_errors(&context, "InitiateZkLogin")
            .await?;

        let login_success = InitiateZkLoginSuccess {
            authorization_url: format!(
                "https://oauth-provider.example.com/auth?client_id={}&redirect_uri={}&state={}",
                req.client_id, req.redirect_uri, req.state
            ),
            pkce_verifier: "mock_pkce_verifier_".to_string() + &Uuid::new_v4().to_string(),
            session_id: Uuid::new_v4().to_string(),
            expires_at: (Utc::now() + chrono::Duration::minutes(10)).timestamp(),
        };

        let response = InitiateZkLoginResponse {
            result: Some(initiate_zk_login_response::Result::Success(login_success)),
        };

        Ok(Response::new(response))
    }

    async fn complete_zk_login(
        &self,
        request: Request<CompleteZkLoginRequest>,
    ) -> Result<Response<CompleteZkLoginResponse>, Status> {
        let req = request.into_inner();
        let context = self.create_context(Some(req.trace_id.clone())).await;

        {
            let stub = self.stub.lock().await;
            stub.log_request(&context, "CompleteZkLogin", "gRPC");
        }

        self.simulate_latency_and_errors(&context, "CompleteZkLogin")
            .await?;

        let user_profile = UserProfileProto {
            player_id: Uuid::new_v4().to_string(),
            bunker_tag: "MockPlayer".to_string(),
            email: "mock.player@example.com".to_string(),
            display_name: "Mock Player".to_string(),
            avatar_url: "https://example.com/avatar.png".to_string(),
            roles: vec!["user".to_string()],
            created_at: Utc::now().timestamp(),
            last_login_at: Utc::now().timestamp(),
        };

        let login_success = CompleteZkLoginSuccess {
            jwt_token: "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.mock_jwt_token".to_string(),
            refresh_token: "mock_refresh_token_".to_string() + &Uuid::new_v4().to_string(),
            expires_at: (Utc::now() + chrono::Duration::hours(24)).timestamp(),
            user_profile: Some(user_profile),
            is_new_user: false,
        };

        let response = CompleteZkLoginResponse {
            result: Some(complete_zk_login_response::Result::Success(login_success)),
        };

        Ok(Response::new(response))
    }

    async fn refresh_token(
        &self,
        request: Request<RefreshTokenRequest>,
    ) -> Result<Response<RefreshTokenResponse>, Status> {
        let req = request.into_inner();
        let context = self.create_context(Some(req.trace_id.clone())).await;

        {
            let stub = self.stub.lock().await;
            stub.log_request(&context, "RefreshToken", "gRPC");
        }

        self.simulate_latency_and_errors(&context, "RefreshToken")
            .await?;

        let refresh_success = RefreshTokenSuccess {
            jwt_token: "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.refreshed_mock_jwt_token".to_string(),
            refresh_token: "new_mock_refresh_token_".to_string() + &Uuid::new_v4().to_string(),
            expires_at: (Utc::now() + chrono::Duration::hours(24)).timestamp(),
        };

        let response = RefreshTokenResponse {
            result: Some(refresh_token_response::Result::Success(refresh_success)),
        };

        Ok(Response::new(response))
    }

    async fn validate_token(
        &self,
        request: Request<ValidateTokenRequest>,
    ) -> Result<Response<ValidateTokenResponse>, Status> {
        let req = request.into_inner();
        let context = self.create_context(Some(req.trace_id.clone())).await;

        {
            let stub = self.stub.lock().await;
            stub.log_request(&context, "ValidateToken", "gRPC");
        }

        self.simulate_latency_and_errors(&context, "ValidateToken")
            .await?;

        // Simple validation - check if token is not empty
        let is_valid = !req.jwt_token.is_empty();

        let validate_success = ValidateTokenSuccess {
            is_valid,
            player_id: if is_valid {
                "player_123".to_string()
            } else {
                "".to_string()
            },
            permissions: if is_valid {
                vec![
                    "read:profile".to_string(),
                    "update:profile".to_string(),
                    "play:game".to_string(),
                ]
            } else {
                vec![]
            },
            expires_at: if is_valid {
                (Utc::now() + chrono::Duration::hours(23)).timestamp()
            } else {
                0
            },
        };

        let response = ValidateTokenResponse {
            result: Some(validate_token_response::Result::Success(validate_success)),
        };

        Ok(Response::new(response))
    }

    async fn revoke_token(
        &self,
        request: Request<RevokeTokenRequest>,
    ) -> Result<Response<RevokeTokenResponse>, Status> {
        let req = request.into_inner();
        let context = self.create_context(Some(req.trace_id.clone())).await;

        {
            let stub = self.stub.lock().await;
            stub.log_request(&context, "RevokeToken", "gRPC");
        }

        self.simulate_latency_and_errors(&context, "RevokeToken")
            .await?;

        let revoke_success = RevokeTokenSuccess { revoked: true };

        let response = RevokeTokenResponse {
            result: Some(revoke_token_response::Result::Success(revoke_success)),
        };

        Ok(Response::new(response))
    }

    async fn create_session(
        &self,
        request: Request<CreateSessionRequest>,
    ) -> Result<Response<CreateSessionResponse>, Status> {
        let req = request.into_inner();
        let context = self.create_context(Some(req.trace_id.clone())).await;

        {
            let stub = self.stub.lock().await;
            stub.log_request(&context, "CreateSession", "gRPC");
        }

        self.simulate_latency_and_errors(&context, "CreateSession")
            .await?;

        let session_success = CreateSessionSuccess {
            session_id: Uuid::new_v4().to_string(),
            expires_at: (Utc::now() + chrono::Duration::hours(8)).timestamp(),
            session_token: "session_token_".to_string() + &Uuid::new_v4().to_string(),
        };

        let response = CreateSessionResponse {
            result: Some(create_session_response::Result::Success(session_success)),
        };

        Ok(Response::new(response))
    }

    async fn get_session_info(
        &self,
        request: Request<GetSessionInfoRequest>,
    ) -> Result<Response<GetSessionInfoResponse>, Status> {
        let req = request.into_inner();
        let context = self.create_context(Some(req.trace_id.clone())).await;

        {
            let stub = self.stub.lock().await;
            stub.log_request(&context, "GetSessionInfo", "gRPC");
        }

        self.simulate_latency_and_errors(&context, "GetSessionInfo")
            .await?;

        let session_info = GetSessionInfoSuccess {
            session_id: req.session_id,
            player_id: "player_123".to_string(),
            device_id: "device_456".to_string(),
            created_at: (Utc::now() - chrono::Duration::hours(2)).timestamp(),
            last_active_at: (Utc::now() - chrono::Duration::minutes(5)).timestamp(),
            expires_at: (Utc::now() + chrono::Duration::hours(6)).timestamp(),
            is_active: true,
        };

        let response = GetSessionInfoResponse {
            result: Some(get_session_info_response::Result::Success(session_info)),
        };

        Ok(Response::new(response))
    }

    async fn end_session(
        &self,
        request: Request<EndSessionRequest>,
    ) -> Result<Response<EndSessionResponse>, Status> {
        let req = request.into_inner();
        let context = self.create_context(Some(req.trace_id.clone())).await;

        {
            let stub = self.stub.lock().await;
            stub.log_request(&context, "EndSession", "gRPC");
        }

        self.simulate_latency_and_errors(&context, "EndSession")
            .await?;

        let end_success = EndSessionSuccess { ended: true };

        let response = EndSessionResponse {
            result: Some(end_session_response::Result::Success(end_success)),
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
            details: HashMap::new(),
        };

        Ok(Response::new(response))
    }
}
