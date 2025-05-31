use sqlx::{Error, PgPool};

pub async fn get_value(pool: &PgPool) -> Result<u64, Error> {
    let row: (String,) = sqlx::query_as("SELECT value::text FROM checkpoint WHERE id = $1")
        .bind(1)
        .fetch_one(pool)
        .await?;

    let value = row
        .0
        .parse::<u64>()
        .unwrap_or(0);
    Ok(value)
}

pub async fn update_value(pool: &PgPool, new_value: u64) -> Result<(), Error> {
    sqlx::query("UPDATE checkpoint SET value = $1 WHERE id = $2")
        .bind(new_value as i64)
        .bind(1)
        .execute(pool)
        .await?;
    Ok(())
}
