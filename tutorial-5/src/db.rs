use sqlx::{
    migrate, postgres::PgPoolOptions, Connection, Executor, PgConnection,
    PgPool,
};

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

    use super::*;
    use async_trait::async_trait;
    use sqlx::{postgres::PgPoolOptions, Row};
    use test_context::AsyncTestContext;

    const ALPHABET: [char; 16] = [
        '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'a', 'b', 'c', 'd',
        'e', 'f',
    ];

    pub async fn get_test_db_connection() -> sqlx::PgConnection {
        get_db_connection("testing").await
    }

    #[derive(Debug)]
    pub struct TestDb {
        pub pool: PgPool,
        pub name: String,
    }

    #[async_trait]
    impl AsyncTestContext for TestDb {
        async fn setup() -> TestDb {
            let mut test_db = TestDb::new().await;

            sqlx::query!(
                "
                CREATE TABLE IF NOT EXISTS key_value_store (
                    key VARCHAR(255) PRIMARY KEY,
                    value TEXT
                );
                "
            )
            .execute(&test_db.pool)
            .await
            .unwrap();

            test_db
        }

        async fn teardown(self) {
            self.pool.close().await;
            let mut connection = get_test_db_connection().await;
            sqlx::query(&format!("DROP DATABASE {}", self.name))
                .execute(&mut connection)
                .await
                .unwrap();
        }
    }

    impl TestDb {
        pub async fn new() -> Self {
            let name = format!("testdb_{}", nanoid::nanoid!(10, &ALPHABET));
            let mut connection = get_test_db_connection().await;
            sqlx::query(&format!("CREATE DATABASE {name}"))
                .execute(&mut connection)
                .await
                .unwrap();

            let pool = PgPoolOptions::new()
                .max_connections(1)
                .connect(
                    &"postgresql://admin:admin@localhost:5432/defaultdb"
                        .to_string()
                        .replace("defaultdb", &name),
                )
                .await
                .unwrap();

            sqlx::migrate!("./migrations").run(&pool).await.unwrap();

            Self { pool, name }
        }
    }
}
