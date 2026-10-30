//! B档第九刀：内联原 `cloud_api_client` 的 HTTP/LLM token 能力，去掉对整 crate 与 cloud websocket 的硬链。

use std::pin::Pin;
use std::sync::Arc;

use anyhow::{Result, anyhow};
pub use cloud_api_types::*;
pub use cloud_api_types::websocket_protocol::MessageToClient;
use futures::stream::{self, Stream};
use futures::StreamExt as _;
use gpui::{App, Task};
use http_client::http::request;
use http_client::{
    AsyncBody, HttpClientWithUrl, HttpRequestExt, Json, Method, Request, Response, StatusCode,
};
use parking_lot::RwLock;
use serde::de::DeserializeOwned;
use thiserror::Error;
use tokio::sync::RwLock as AsyncRwLock;

pub type MessageStream = Pin<Box<dyn Stream<Item = Result<MessageToClient>> + Send>>;

struct Credentials {
    user_id: u32,
    access_token: String,
}

#[derive(Clone, Copy)]
enum Authentication {
    Credentials,
    Session,
}

#[derive(Debug, Error)]
pub enum ClientApiError {
    #[error("Unauthorized")]
    Unauthorized,
    #[error("not signed in")]
    NotSignedIn,
    #[error("connection to {host} failed")]
    ConnectionFailed {
        host: String,
        #[source]
        source: anyhow::Error,
    },
    #[error("{host} returned {status}")]
    ServerError {
        host: String,
        status: StatusCode,
        body: String,
    },
    #[error("invalid response")]
    InvalidResponse(#[source] anyhow::Error),
    #[error("failed to build request")]
    RequestBuildFailed(#[source] anyhow::Error),
}

pub struct CloudApiClient {
    credentials: RwLock<Option<Credentials>>,
    http_client: Arc<HttpClientWithUrl>,
    authentication: Authentication,
}

/// B档第九刀：cloud `/client/users/connect` websocket 占位，本地壳不连。
pub struct CloudWebSocketConnection;

impl CloudWebSocketConnection {
    pub fn spawn(self, _cx: &App) -> (MessageStream, Task<()>) {
        let messages = stream::empty::<Result<MessageToClient>>().boxed();
        (messages, Task::ready(()))
    }
}

impl CloudApiClient {
    pub fn new(http_client: Arc<HttpClientWithUrl>) -> Self {
        Self {
            credentials: RwLock::new(None),
            http_client,
            authentication: Authentication::Credentials,
        }
    }

    pub fn new_with_session_authentication(http_client: Arc<HttpClientWithUrl>) -> Self {
        Self {
            credentials: RwLock::new(None),
            http_client,
            authentication: Authentication::Session,
        }
    }

    pub fn has_credentials(&self) -> bool {
        self.credentials.read().is_some()
    }

    pub fn set_credentials(&self, user_id: u32, access_token: String) {
        *self.credentials.write() = Some(Credentials {
            user_id,
            access_token,
        });
    }

    pub fn clear_credentials(&self) {
        *self.credentials.write() = None;
    }

    pub fn cloud_host(&self) -> String {
        self.http_client
            .build_zed_cloud_url("/")
            .ok()
            .and_then(|url| url.host_str().map(String::from))
            .unwrap_or_else(|| "cloud.zed.dev".into())
    }

    /// B档第九刀：本地壳不建立 cloud websocket。
    pub fn connect(self: &Arc<Self>, _cx: &App) -> Result<Task<Result<CloudWebSocketConnection>>> {
        Ok(Task::ready(Err(anyhow!(
            "cloud websocket is disabled in this local-only build"
        ))))
    }

    pub async fn get_authenticated_user(
        &self,
        system_id: Option<String>,
    ) -> Result<GetAuthenticatedUserResponse, ClientApiError> {
        let request_builder = Request::builder()
            .method(Method::GET)
            .uri(
                self.http_client
                    .build_zed_cloud_url("/client/users/me")
                    .map_err(ClientApiError::RequestBuildFailed)?
                    .as_ref(),
            )
            .when_some(system_id, |builder, system_id| {
                builder.header(ZED_SYSTEM_ID_HEADER_NAME, system_id)
            });

        self.send_authenticated_json_request(request_builder, AsyncBody::default())
            .await
    }

    pub(crate) async fn create_llm_token(
        &self,
        system_id: Option<String>,
        organization_id: OrganizationId,
    ) -> Result<CreateLlmTokenResponse, ClientApiError> {
        let request_builder = Request::builder()
            .method(Method::POST)
            .uri(
                self.http_client
                    .build_zed_cloud_url("/client/llm_tokens")
                    .map_err(ClientApiError::RequestBuildFailed)?
                    .as_ref(),
            )
            .when_some(system_id, |builder, system_id| {
                builder.header(ZED_SYSTEM_ID_HEADER_NAME, system_id)
            });

        self.send_authenticated_json_request(
            request_builder,
            Json(CreateLlmTokenBody { organization_id }),
        )
        .await
    }

    pub async fn update_system_settings(
        &self,
        system_id: String,
        body: UpdateSystemSettingsBody,
    ) -> Result<SystemSettings, ClientApiError> {
        let request_builder = Request::builder()
            .method(Method::PATCH)
            .uri(
                self.http_client
                    .build_zed_cloud_url("/client/system_settings")
                    .map_err(ClientApiError::RequestBuildFailed)?
                    .as_ref(),
            )
            .header(ZED_SYSTEM_ID_HEADER_NAME, system_id);

        self.send_authenticated_json_request(request_builder, Json(body))
            .await
    }

    pub async fn send_authenticated_json_request<T: DeserializeOwned>(
        &self,
        request_builder: request::Builder,
        body: impl Into<AsyncBody>,
    ) -> Result<T, ClientApiError> {
        let mut response = self
            .send_authenticated_request(request_builder, body)
            .await?;
        Self::read_response_json(&mut response).await
    }

    async fn send_authenticated_request(
        &self,
        request_builder: request::Builder,
        body: impl Into<AsyncBody>,
    ) -> Result<Response<AsyncBody>, ClientApiError> {
        let request = if matches!(self.authentication, Authentication::Session) {
            build_request(request_builder, body, None)
                .map_err(ClientApiError::RequestBuildFailed)?
        } else {
            let credentials = self.credentials.read();
            let credentials = credentials.as_ref().ok_or(ClientApiError::NotSignedIn)?;
            build_request(request_builder, body, Some(credentials))
                .map_err(ClientApiError::RequestBuildFailed)?
        };

        let host = self.cloud_host();
        let mut response = self.http_client.send(request).await.map_err(|source| {
            ClientApiError::ConnectionFailed {
                host: host.clone(),
                source,
            }
        })?;

        let status = response.status();
        if status.is_success() {
            return Ok(response);
        }

        if status == StatusCode::UNAUTHORIZED {
            return Err(ClientApiError::Unauthorized);
        }

        let body = match Self::read_response_body(&mut response).await {
            Ok(body) => body,
            Err(error) => format!("failed to read response body: {error}"),
        };
        Err(ClientApiError::ServerError { host, status, body })
    }

    async fn read_response_json<T: DeserializeOwned>(
        response: &mut Response<AsyncBody>,
    ) -> Result<T, ClientApiError> {
        let body = Self::read_response_body(response).await?;
        serde_json::from_str(&body).map_err(|error| ClientApiError::InvalidResponse(error.into()))
    }

    async fn read_response_body(
        response: &mut Response<AsyncBody>,
    ) -> Result<String, ClientApiError> {
        use futures::AsyncReadExt as _;
        let mut body = String::new();
        response
            .body_mut()
            .read_to_string(&mut body)
            .await
            .map_err(|error| ClientApiError::InvalidResponse(error.into()))?;
        Ok(body)
    }

    pub async fn validate_credentials(&self, user_id: u32, access_token: &str) -> Result<bool> {
        let request = build_request(
            Request::builder().method(Method::GET).uri(
                self.http_client
                    .build_zed_cloud_url("/client/users/me")?
                    .as_ref(),
            ),
            AsyncBody::default(),
            Some(&Credentials {
                user_id,
                access_token: access_token.into(),
            }),
        )?;

        use futures::AsyncReadExt as _;
        let mut response = self.http_client.send(request).await?;

        if response.status().is_success() {
            Ok(true)
        } else {
            let mut body = String::new();
            response.body_mut().read_to_string(&mut body).await?;
            if response.status() == StatusCode::UNAUTHORIZED {
                Ok(false)
            } else {
                Err(anyhow!(
                    "Failed to get authenticated user.\nStatus: {:?}\nBody: {body}",
                    response.status()
                ))
            }
        }
    }

    pub async fn submit_agent_feedback(&self, body: SubmitAgentThreadFeedbackBody) -> Result<()> {
        let request = Request::builder().method(Method::POST).uri(
            self.http_client
                .build_zed_cloud_url("/client/feedback/agent_thread")?
                .as_ref(),
        );

        self.send_authenticated_request(request, AsyncBody::from(serde_json::to_string(&body)?))
            .await?;
        Ok(())
    }

    pub async fn submit_agent_feedback_comments(
        &self,
        body: SubmitAgentThreadFeedbackCommentsBody,
    ) -> Result<()> {
        let request = Request::builder().method(Method::POST).uri(
            self.http_client
                .build_zed_cloud_url("/client/feedback/agent_thread_comments")?
                .as_ref(),
        );

        self.send_authenticated_request(request, AsyncBody::from(serde_json::to_string(&body)?))
            .await?;
        Ok(())
    }

    pub async fn submit_edit_prediction_feedback(
        &self,
        body: SubmitEditPredictionFeedbackBody,
    ) -> Result<()> {
        let request = Request::builder().method(Method::POST).uri(
            self.http_client
                .build_zed_cloud_url("/client/feedback/edit_prediction")?
                .as_ref(),
        );

        self.send_authenticated_request(request, AsyncBody::from(serde_json::to_string(&body)?))
            .await?;
        Ok(())
    }
}

fn build_request(
    req: request::Builder,
    body: impl Into<AsyncBody>,
    credentials: Option<&Credentials>,
) -> Result<Request<AsyncBody>> {
    Ok(req
        .header("Content-Type", "application/json")
        .when_some(credentials, |request, credentials| {
            request.header(
                "Authorization",
                format!("{} {}", credentials.user_id, credentials.access_token),
            )
        })
        .body(body.into())?)
}

#[derive(Clone, Default)]
pub struct LlmApiToken(Arc<AsyncRwLock<Option<CachedLlmApiToken>>>);

struct CachedLlmApiToken {
    organization_id: OrganizationId,
    token: String,
}

impl LlmApiToken {
    pub async fn cached(
        &self,
        client: &CloudApiClient,
        system_id: Option<String>,
        organization_id: OrganizationId,
    ) -> Result<String, ClientApiError> {
        {
            let lock = self.0.read().await;
            if let Some(CachedLlmApiToken {
                organization_id: cached_organization_id,
                token,
            }) = lock.as_ref()
                && cached_organization_id == &organization_id
            {
                return Ok(token.to_string());
            }
        }
        Self::fetch(self.0.write().await, client, system_id, organization_id).await
    }

    pub async fn refresh(
        &self,
        client: &CloudApiClient,
        system_id: Option<String>,
        organization_id: OrganizationId,
    ) -> Result<String, ClientApiError> {
        Self::fetch(self.0.write().await, client, system_id, organization_id).await
    }

    pub async fn clear(&self) {
        *self.0.write().await = None;
    }

    pub async fn clear_and_refresh(
        &self,
        client: &CloudApiClient,
        system_id: Option<String>,
        organization_id: OrganizationId,
    ) -> Result<String, ClientApiError> {
        let mut lock = self.0.write().await;
        *lock = None;
        Self::fetch(lock, client, system_id, organization_id).await
    }

    async fn fetch(
        mut lock: tokio::sync::RwLockWriteGuard<'_, Option<CachedLlmApiToken>>,
        client: &CloudApiClient,
        system_id: Option<String>,
        organization_id: OrganizationId,
    ) -> Result<String, ClientApiError> {
        let result = client
            .create_llm_token(system_id, organization_id.clone())
            .await;
        match result {
            Ok(response) => {
                *lock = Some(CachedLlmApiToken {
                    organization_id,
                    token: response.token.0.clone(),
                });
                Ok(response.token.0)
            }
            Err(err) => {
                *lock = None;
                Err(err)
            }
        }
    }
}
