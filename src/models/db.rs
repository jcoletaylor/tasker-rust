use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::types::Decimal;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Command {
    pub command_id: i64,
    pub named_command_id: i32,
    pub status: String,
    pub requested_at: DateTime<Utc>,
    pub complete: bool,
    pub initiator: Option<String>,
    pub source_system: Option<String>,
    pub bypass_steps: Option<JsonValue>,
    pub reason: Option<String>,
    pub context: Option<JsonValue>,
    pub identity_hash: String,
    pub tags: Option<JsonValue>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct UpsertCommand {
    pub named_command_id: i32,
    pub status: String,
    pub requested_at: DateTime<Utc>,
    pub complete: bool,
    pub initiator: Option<String>,
    pub source_system: Option<String>,
    pub bypass_steps: Option<JsonValue>,
    pub reason: Option<String>,
    pub context: Option<JsonValue>,
    pub identity_hash: String,
    pub tags: Option<JsonValue>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct WorkflowStep {
    pub command_id: i64,
    pub workflow_step_id: i64,
    pub named_step_id: i32,
    pub depends_on_step_id: Option<i64>,
    pub status: String,
    pub retryable: bool,
    pub retry_limit: Option<i32>,
    pub processed_at: Option<DateTime<Utc>>,
    pub in_process: bool,
    pub processed: bool,
    pub attempts: Option<i32>,
    pub last_attempted_at: Option<DateTime<Utc>>,
    pub backoff_request_seconds: Option<i32>,
    pub inputs: Option<JsonValue>,
    pub results: Option<JsonValue>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct UpsertWorkflowStep {
    pub named_step_id: i32,
    pub depends_on_step_id: Option<i64>,
    pub status: String,
    pub retryable: bool,
    pub retry_limit: i32,
    pub processed_at: Option<DateTime<Utc>>,
    pub in_process: Option<bool>,
    pub processed: bool,
    pub attempts: i32,
    pub last_attempted_at: Option<DateTime<Utc>>,
    pub backoff_request_seconds: Option<i32>,
    pub inputs: Option<JsonValue>,
    pub results: Option<JsonValue>,
}
