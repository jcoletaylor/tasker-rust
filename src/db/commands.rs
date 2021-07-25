use anyhow;
use sqlx::{query_as, PgPool};

use crate::errors::RepositoryError;
use crate::models::db;

pub async fn get(
    command_id: i64,
    db_pool: &PgPool,
) -> Result<db::CommandWithJoinsRow, anyhow::Error> {
    let maybe_command = query_as!(
        db::CommandWithJoinsRow,
        r#"
        SELECT 
            commands.command_id,
            commands.named_command_id,
            named_commands.name AS named_command,
            commands.status,
            commands.complete,
            commands.requested_at,
            commands.initiator,
            commands.source_system,
            commands.reason,
            commands.bypass_steps,
            commands.tags,
            commands.context,
            commands.identity_hash,
            workflow_steps.workflow_step_id,
            workflow_steps.named_step_id,
            named_steps.name AS named_step,
            workflow_steps.depends_on_step_id AS workflow_step_depends_on_step_id,
            workflow_steps.status AS workflow_step_status,
            workflow_steps.retryable AS workflow_step_retryable,
            workflow_steps.retry_limit AS workflow_step_retry_limit,
            workflow_steps.in_process AS workflow_step_in_process,
            workflow_steps.processed AS workflow_step_processed,
            workflow_steps.processed_at AS workflow_step_processed_at,
            workflow_steps.attempts AS workflow_step_attempts,
            workflow_steps.last_attempted_at AS workflow_step_last_attempted_at,
            workflow_steps.backoff_request_seconds AS workflow_step_backoff_request_seconds,
            workflow_steps.inputs AS workflow_step_inputs,
            workflow_steps.results AS workflow_step_results
        FROM commands
            INNER JOIN named_commands USING (named_command_id)
            INNER JOIN workflow_steps USING (command_id)
            INNER JOIN named_steps ON workflow_steps.named_step_id = named_steps.named_step_id
        WHERE 
            commands.command_id = $1
        "#,
        command_id
    )
    .fetch_optional(db_pool)
    .await?;
    match maybe_command {
        Some(command) => Ok(command),
        None => {
            let error = anyhow::Error::new(RepositoryError::new(format!(
                "No command for {}",
                command_id
            )));
            Err(error)
        }
    }
}

pub async fn get_all(
    db_pool: &PgPool,
    limit: i64,
    offset: i64,
) -> Result<Vec<db::CommandWithJoinsRow>, anyhow::Error> {
    let commands = query_as!(
        db::CommandWithJoinsRow,
        r#"
        SELECT 
            commands.command_id,
            commands.named_command_id,
            named_commands.name AS named_command,
            commands.status,
            commands.complete,
            commands.requested_at,
            commands.initiator,
            commands.source_system,
            commands.reason,
            commands.bypass_steps,
            commands.tags,
            commands.context,
            commands.identity_hash,
            workflow_steps.workflow_step_id,
            workflow_steps.named_step_id,
            named_steps.name AS named_step,
            workflow_steps.depends_on_step_id AS workflow_step_depends_on_step_id,
            workflow_steps.status AS workflow_step_status,
            workflow_steps.retryable AS workflow_step_retryable,
            workflow_steps.retry_limit AS workflow_step_retry_limit,
            workflow_steps.in_process AS workflow_step_in_process,
            workflow_steps.processed AS workflow_step_processed,
            workflow_steps.processed_at AS workflow_step_processed_at,
            workflow_steps.attempts AS workflow_step_attempts,
            workflow_steps.last_attempted_at AS workflow_step_last_attempted_at,
            workflow_steps.backoff_request_seconds AS workflow_step_backoff_request_seconds,
            workflow_steps.inputs AS workflow_step_inputs,
            workflow_steps.results AS workflow_step_results
        FROM commands
            INNER JOIN named_commands USING (named_command_id)
            INNER JOIN workflow_steps USING (command_id)
            INNER JOIN named_steps ON workflow_steps.named_step_id = named_steps.named_step_id
        ORDER BY commands.requested_at DESC
        LIMIT $1
        OFFSET $2
        "#,
        limit,
        offset
    )
    .fetch_all(db_pool)
    .await?;
    Ok(commands)
}
