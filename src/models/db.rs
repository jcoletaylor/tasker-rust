use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Command {
    pub command_id: i64,
    pub named_command_id: i32,
    pub status: String,
    pub complete: bool,
    pub requested_at: DateTime<Utc>,
    pub initiator: Option<String>,
    pub source_system: Option<String>,
    pub reason: Option<String>,
    pub bypass_steps: Option<JsonValue>,
    pub tags: Option<JsonValue>,
    pub context: Option<JsonValue>,
    pub identity_hash: String,
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
    pub workflow_step_id: i64,
    pub command_id: i64,
    pub named_step_id: i32,
    pub depends_on_step_id: Option<i64>,
    pub status: String,
    pub retryable: bool,
    pub retry_limit: Option<i32>,
    pub in_process: bool,
    pub processed: bool,
    pub processed_at: Option<DateTime<Utc>>,
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
    pub retry_limit: Option<i32>,
    pub in_process: bool,
    pub processed: bool,
    pub processed_at: Option<DateTime<Utc>>,
    pub attempts: Option<i32>,
    pub last_attempted_at: Option<DateTime<Utc>>,
    pub backoff_request_seconds: Option<i32>,
    pub inputs: Option<JsonValue>,
    pub results: Option<JsonValue>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AnnotationType {
    pub annotation_type_id: i32,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CommandAnnotation {
    pub command_annotation_id: i64,
    pub command_id: i32,
    pub annotation_type_id: i32,
    pub annotation: Option<JsonValue>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DependentSystemObjectMap {
    pub dependent_system_object_map_id: i64,
    pub dependent_system_one_id: i32,
    pub dependent_system_two_id: i32,
    pub remote_id_one: String,
    pub remote_id_two: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DependentSystem {
    pub dependent_system_id: i32,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct NamedCommand {
    pub named_command_id: i32,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct NamedCommandsNamedStep {
    pub id: i32,
    pub named_command_id: i32,
    pub named_step_id: i32,
    pub skippable: bool,
    pub default_retryable: bool,
    pub default_retry_limit: i32,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct NamedStep {
    pub named_step_id: i32,
    pub dependent_system_id: i32,
    pub name: String,
    pub description: Option<String>,
}
