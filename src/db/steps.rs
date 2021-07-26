use anyhow;
use sqlx::{query_as, PgPool};

use crate::errors::RepositoryError;
use crate::models::db;

pub async fn get(
    task_id: i64,
    workflow_step_id: i64,
    db_pool: &PgPool,
) -> Result<db::WorkflowStepWithJoinsRow, anyhow::Error> {
    let maybe_workflow_step= query_as!(
        db::WorkflowStepWithJoinsRow,
        r#"
        SELECT
            workflow_steps.*,
            named_steps.name AS named_step
        FROM workflow_steps
            INNER JOIN named_steps USING (named_step_id)
        WHERE 
            workflow_steps.task_id = $1
            AND workflow_steps.workflow_step_id = $2
        "#,
        task_id,
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
    db_pool: &PgPool,
    task_id: i64,
    limit: i64,
    offset: i64
) -> Result<Vec<db::WorkflowStepWithJoinsRow>, anyhow::Error> {
    let workflow_steps = query_as!(
        db::WorkflowStepWithJoinsRow,
        r#"
        SELECT
            workflow_steps.*,
            named_steps.name AS named_step
        FROM workflow_steps
            INNER JOIN named_steps USING (named_step_id)
        WHERE 
            workflow_steps.task_id = $1
        ORDER BY workflow_steps.workflow_step_id ASC
        LIMIT $2
        OFFSET $3
        "#,
        task_id,
        limit,
        offset
    )
    .fetch_all(db_pool)
    .await?;
    Ok(workflow_steps)
}
