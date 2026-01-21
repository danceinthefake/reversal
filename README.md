# Reversal

A Rust/Axum application that reverses text in the URL path and stores the results. Supports both SQLite and PostgreSQL databases.

## Features
- Text reversal API
- Database storage of all reversals
- Flexible database backend (SQLite or PostgreSQL)
- Docker support for deployment

## Configuration

### Database Setup
The application supports two database types:

#### SQLite (Default)
```bash
export DB_TYPE=sqlite
export DATABASE_URL=sqlite:data/reversal.db?mode=rwc
```

#### PostgreSQL
```bash
export DB_TYPE=postgresql
export DATABASE_URL="postgres://<username>:<password>@<host>:<port>/<database>"
```

### Local Development

1. Build and run:
```bash
cargo run
```

2. Or build a release binary:
```bash
cargo build --release
./target/release/reversal
```

### Docker Deployment
```bash
docker compose up --build
```

## Usage

The application runs on port 3000 by default.

### Basic Usage
1. View the welcome page (shows current database configuration):
```bash
curl http://localhost:3000
```

2. Reverse any text by adding it to the path:
```bash
curl http://localhost:3000/wearenotalone
```

Response:
```
enolatoneraew
```

Each request is stored in the configured database with:
- Original path
- Reversed result
- Unique ID

## Requirements
- Rust 1.75+
- PostgreSQL (optional)
- Docker (optional)
