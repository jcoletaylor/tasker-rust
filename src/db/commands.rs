use anyhow;
use sqlx::{query_as, Error as SqlxError, PgPool};

use crate::errors::RepositoryError;
use crate::helpers;
use crate::models::db;

pub async fn get(command_id: i64, db_pool: &PgPool) -> Result<db::Command, anyhow::Error> {
    let maybe_command = query_as!(
        db::Command,
        r#"
        SELECT commands.* FROM commands WHERE commands.command_id = $1
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

pub async fn get_all(db_pool: &PgPool) -> Result<Vec<db::Command>, anyhow::Error> {
    let commands = query_as!(
        db::Command,
        r#"
        SELECT commands.* FROM commands
        "#,
    )
    .fetch_all(db_pool)
    .await?;
    Ok(commands)
}
