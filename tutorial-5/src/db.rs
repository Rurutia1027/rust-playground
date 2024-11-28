use sqlx::{
    postgres::PgPoolOptions, Connection, Executor, PgConnection, PgPool,
};
use tracing_subscriber::fmt::format;

pub async fn get_db_pool(name: &str, max_connections: u32) -> PgPool {
    let name_query = format!("SET application_name = '{}';", name);
    PgPoolOptions::new()
        .after_connect(move |conn, _meta| {
            let name_query = name_query.clone();
            Box::pin(async move {
                conn.execute(name_query.as_ref()).await?;
                Ok(())
            })
        })
        .max_connections(max_connections)
        // this db url will be extracted as env conf
        .connect("postgresql://admin:admin@localhost:5432/defaultdb")
        .await
        .expect("expect DB to be available to connect")
}

pub async fn get_db_connection(name: &str) -> sqlx::PgConnection {
    let mut conn = PgConnection::connect(
        "postgresql://admin:admin@localhost:5432/defaultdb",
    )
    .await
    .expect("expect DB to be available to connect");

    let query = format!("SET application_name = '{}'", name);
    sqlx::query(&query).execute(&mut conn).await.unwrap();
    conn
}

#[cfg(test)]
pub mod tests {
    use async_trait::async_trait;
    use nanoid::nanoid;
}
