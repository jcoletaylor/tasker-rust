use anyhow;
use sqlx::{query_as, Error as SqlxError, PgPool};

use crate::errors::RepositoryError;
use crate::helpers;
use crate::models::db;

pub async fn get(
    command_id: i64,
    workflow_step_id: i64,
    db_pool: &PgPool,
) -> Result<db::WorkflowStep, anyhow::Error> {
    let maybe_workflow_step= query_as!(
        db::WorkflowStep,
        r#"
        SELECT workflow_steps.* FROM workflow_steps WHERE workflow_steps.command_id = $1 AND workflow_steps.workflow_step_id = $2
        "#,
        command_id,
        workflow_step_id
    )
    .fetch_optional(db_pool)
    .await?;
    match maybe_workflow_step {
        Some(workflow_step) => Ok(workflow_step),
        None => {
            let error = anyhow::Error::new(RepositoryError::new(format!(
                "No workflow_step for {}",
                workflow_step_id
            )));
            Err(error)
        }
    }
}

pub async fn get_all(
    command_id: i64,
    db_pool: &PgPool,
) -> Result<Vec<db::WorkflowStep>, anyhow::Error> {
    let workflow_steps = query_as!(
        db::WorkflowStep,
        r#"
        SELECT workflow_steps.* FROM workflow_steps WHERE workflow_steps.command_id = $1
        "#,
        command_id
    )
    .fetch_all(db_pool)
    .await?;
    Ok(workflow_steps)
}
