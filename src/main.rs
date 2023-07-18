use axum::{
    extract::Extension, extract::Json, extract::Path, headers::HeaderMap, http::StatusCode,
    routing::get, routing::post, routing::put, Router,
};
use color_eyre::eyre::eyre;
use color_eyre::Result;
use dotenvy::dotenv;
use dotenvy_macro::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use sqlx::{
    postgres::{PgPoolOptions, PgRow},
    Row,
};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv()?;
    color_eyre::install()?;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(dotenv!("DATABASE_URL"))
        .await?;
    sqlx::migrate!().run(&pool).await?;

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/users/create", post(create_user))
        .route("/users/auth", get(authorize))
        .route("/syncs/progress", put(update_progress))
        .route("/syncs/progress/:document", get(get_progress))
        .layer(Extension(pool));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse()?)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

async fn create_user(Extension(pool): Extension<PgPool>, Json(payload): Json<User>) -> StatusCode {
    match sqlx::query_as!(
        User,
        "SELECT * FROM user_ WHERE username=$1",
        payload.username
    )
    .fetch_one(&pool)
    .await
    {
        Ok(_) => StatusCode::PAYMENT_REQUIRED,
        Err(_) => {
            match sqlx::query!(
                "INSERT INTO user_ (username, password) VALUES ($1, crypt($2 , gen_salt('bf')))",
                payload.username,
                payload.password
            )
            .execute(&pool)
            .await
            {
                Ok(_) => StatusCode::CREATED,
                Err(_) => StatusCode::PAYMENT_REQUIRED,
            }
        }
    }
}
async fn authorize(Extension(pool): Extension<PgPool>, headers: HeaderMap) -> StatusCode {
    match authenticate(axum::Extension(pool), headers).await {
        Ok(res) => match res {
            true => StatusCode::OK,
            false => StatusCode::UNAUTHORIZED,
        },
        // Do Something more here?
        Err(_) => StatusCode::UNAUTHORIZED,
    }
}
async fn update_progress(Json(payload): Json<User>) -> StatusCode {
    StatusCode::OK
}
async fn get_progress(Path(document): Path<String>) {}

#[derive(Deserialize, Serialize)]
struct User {
    username: String,
    password: String,
}

#[derive(Deserialize)]
struct UpdateProgress {
    document: String,
    progress: String,
    percentage: f64,
    device: String,
    device_id: String,
}

async fn authenticate(Extension(pool): Extension<PgPool>, headers: HeaderMap) -> Result<bool> {
    match headers.get("x-auth-user").zip(headers.get("x-auth-key")) {
        Some((username, password)) => Ok(sqlx::query_as!(
            User,
            "SELECT * FROM user_ WHERE username = $1 AND password = crypt($2, password);",
            username.to_str()?,
            password.to_str()?
        )
        .fetch_one(&pool)
        .await
        .is_ok()),
        None => Err(eyre!("Authentication headers empty")),
    }
}
