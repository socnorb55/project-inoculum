use std::env;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Database;

mod error {
    use axum::Json;
    use axum::http::StatusCode;
    use axum::response::{IntoResponse, Response};
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum Error {
        #[error("database error: {0}")]
        Db(String),
    }

    impl IntoResponse for Error {
        fn into_response(self) -> Response {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(self.to_string())).into_response()
        }
    }

    impl From<surrealdb::Error> for Error {
        fn from(e: surrealdb::Error) -> Self {
            eprintln!("{e}");
            Self::Db(e.to_string())
        }
    }
}

pub async fn get_database_client() -> Result<Surreal<Client>, error::Error> {
    dotenvy::dotenv().ok();

    let database_name: String = env::var("SURREAL_DB_NAME")
        .map_err(|_| error::Error::Db("missing SURREAL_DB_NAME".into()))?;

    let database_namespace: String = env::var("SURREAL_DB_NAMESPACE")
        .map_err(|_| error::Error::Db("missing SURREAL_DB_NAMESPACE".into()))?;

    let database_password: String = env::var("SURREAL_DB_PASSWORD")
        .map_err(|_| error::Error::Db("missing SURREAL_DB_PASSWORD".into()))?;

    let database_url: String = env::var("SURREAL_DB_URL")
        .map_err(|_| error::Error::Db("missing SURREAL_DB_URL".into()))?;

    let database_username: String = env::var("SURREAL_DB_USERNAME")
        .map_err(|_| error::Error::Db("missing SURREAL_DB_USERNAME".into()))?;

    let database: Surreal<Client> = Surreal::new::<Ws>(&database_url).await?;

    database
        .signin(Database {
            namespace: &database_namespace,
            database: &database_name,
            username: &database_username,
            password: &database_password,
        })
        .await?;

    Ok(database)
}
