use crate::CommonQueryResponse;
#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::api_model;

use super::Vote;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Eq, PartialEq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
#[api_model(base = "/topics/v1/:topic-id/comments", iter_type=CommonQueryResponse)]
pub struct Comment {
    #[api_model(summary)]
    pub id: String,
    #[api_model(summary)]
    pub profile_url: String,
    #[api_model(summary)]
    pub choice: Option<Vote>,
    #[api_model(summary)]
    pub nickname: String,
    #[api_model(summary, action = comment, related = String)]
    pub content: String,
    #[api_model(summary)]
    pub created_at: u64,
    #[api_model(summary)]
    pub likes: u64,
    #[api_model(action_by_id = like, related = bool)]
    pub is_liked: bool,
}
