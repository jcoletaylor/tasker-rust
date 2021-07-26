use anyhow;
use sqlx::{query_as, PgPool};

use crate::errors::RepositoryError;
use crate::models::db;

pub async fn get(
    task_id: i64,
    db_pool: &PgPool,
) -> Result<db::TaskWithJoinsRow, anyhow::Error> {
    let maybe_task = query_as!(
        db::TaskWithJoinsRow,
        r#"
        SELECT 
            tasks.task_id,
            tasks.named_task_id,
            named_tasks.name AS named_task,
            tasks.status,
            tasks.complete,
            tasks.requested_at,
            tasks.initiator,
            tasks.source_system,
            tasks.reason,
            tasks.bypass_steps,
            tasks.tags,
            tasks.context,
            tasks.identity_hash,
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
        FROM tasks
            INNER JOIN named_tasks USING (named_task_id)
            INNER JOIN workflow_steps USING (task_id)
            INNER JOIN named_steps ON workflow_steps.named_step_id = named_steps.named_step_id
        WHERE 
            tasks.task_id = $1
        "#,
        task_id
    )
    .fetch_optional(db_pool)
    .await?;
    match maybe_task {
        Some(command) => Ok(command),
        None => {
            let error = anyhow::Error::new(RepositoryError::new(format!(
                "No command for {}",
                task_id
            )));
            Err(error)
        }
    }
}

pub async fn get_all(
    db_pool: &PgPool,
    limit: i64,
    offset: i64,
) -> Result<Vec<db::TaskWithJoinsRow>, anyhow::Error> {
    let tasks = query_as!(
        db::TaskWithJoinsRow,
        r#"
        SELECT 
            tasks.task_id,
            tasks.named_task_id,
            named_tasks.name AS named_task,
            tasks.status,
            tasks.complete,
            tasks.requested_at,
            tasks.initiator,
            tasks.source_system,
            tasks.reason,
            tasks.bypass_steps,
            tasks.tags,
            tasks.context,
            tasks.identity_hash,
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
        FROM tasks
            INNER JOIN named_tasks USING (named_task_id)
            INNER JOIN workflow_steps USING (task_id)
            INNER JOIN named_steps ON workflow_steps.named_step_id = named_steps.named_step_id
        ORDER BY tasks.requested_at DESC
        LIMIT $1
        OFFSET $2
        "#,
        limit,
        offset
    )
    .fetch_all(db_pool)
    .await?;
    Ok(tasks)
}
