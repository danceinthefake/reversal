use axum::{
    extract::{Path, State},
    response::Html,
    routing::get,
    Router,
};
use sqlx::AnyPool;
use std::env;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone)]
struct AppState {
    pool: AnyPool,
    db_type: String,
    db_url: String,
}

fn reverse_text(text: &str) -> String {
    text.chars().rev().collect()
}

async fn welcome(State(state): State<AppState>) -> Html<String> {
    let db_info = format!(
        "Currently using {} database at: {}",
        state.db_type.to_uppercase(),
        state.db_url
    );

    Html(format!(
        r#"
<h1>Welcome to emocleW.</h1>
<h2>When your request path become reversed.</h2>
<p>Try any path now</p>
<hr>
<small>{}</small>
"#,
        db_info
    ))
}

async fn reverse_req(
    State(state): State<AppState>,
    Path(text): Path<String>,
) -> Html<String> {
    let reversed = reverse_text(&text);

    let query = if state.db_type == "sqlite" {
        "INSERT INTO reversal (path, result) VALUES (?, ?)"
    } else {
        "INSERT INTO reversal (path, result) VALUES ($1, $2)"
    };

    let _ = sqlx::query(query)
        .bind(&text)
        .bind(&reversed)
        .execute(&state.pool)
        .await;

    Html(format!(
        r#"
<h1>{}</h1>
"#,
        reversed
    ))
}

async fn init_db(pool: &AnyPool, db_type: &str) -> Result<(), sqlx::Error> {
    let create_table = if db_type == "sqlite" {
        r#"
        CREATE TABLE IF NOT EXISTS reversal (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            path TEXT NOT NULL,
            result TEXT NOT NULL
        )
        "#
    } else {
        r#"
        CREATE TABLE IF NOT EXISTS reversal (
            id SERIAL PRIMARY KEY,
            path TEXT NOT NULL,
            result TEXT NOT NULL
        )
        "#
    };

    sqlx::query(create_table).execute(pool).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let db_type = env::var("DB_TYPE").unwrap_or_else(|_| "sqlite".to_string()).to_lowercase();

    let db_url = if db_type == "sqlite" {
        env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:data/reversal.db?mode=rwc".to_string())
    } else {
        env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/reversal".to_string())
    };

    sqlx::any::install_default_drivers();

    let pool = AnyPool::connect(&db_url)
        .await
        .expect("Failed to connect to database");

    init_db(&pool, &db_type).await.expect("Failed to initialize database");

    let state = AppState {
        pool,
        db_type: db_type.clone(),
        db_url: db_url.clone(),
    };

    let app = Router::new()
        .route("/", get(welcome))
        .route("/{text}", get(reverse_req))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("{}:{}", host, port);

    tracing::info!("Starting server on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
