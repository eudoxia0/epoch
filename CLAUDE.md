# Intro

Epoch is a self-hosted journal web application written in Rust. It allows users to create daily journal entries with Markdown support and track personal achievements. The application uses a simple SQLite database for persistence and serves a web interface through axum.

## Directory Structure

```
src/
  assets/ # static assets
    favicon.png
    reset.css
    style.css
    ...
  controllers/ # Rails-style HTTP controllers
    ...
  models/ # Database access objects
    ...
  types/ # Domain types with custom validation, SQL mapping.
    ...
  cli.rs # `clap`-based CLI.
  config.rs # configuration object
  error.rs # error type
  main.rs # entrypoint
  server.rs # axum server setup
templates/          # Askama (Jinja2-style) templates
  _base.jinja2
  ...
```

# State Management

The application uses a simple shared state pattern:

```rust
pub struct ServerState {
    pub db: Arc<Mutex<Connection>>,
}
```

A single SQLite connection is wrapped in Arc<Mutex<>> and shared across all request handlers. Each request acquires a lock and starts a transaction.

# Schema

Located in `src/models/schema.sql`.

# Error Handling (error.rs)

Single error type: `AppError` with a String message.

Type alias: `Fallible<T> = Result<T, AppError>`

# Pattern: Database Access

```rust
let mut db: MutexGuard<'_, Connection> = state.db.lock().await;
let tx: Transaction<'_> = db.transaction()?;
// ... perform operations ...
tx.commit()?; // Or let it drop for implicit rollback
```
