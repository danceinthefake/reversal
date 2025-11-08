# Reversal 

A Flask application that reverses text in the URL path and stores the results. Supports both SQLite and PostgreSQL databases.

## Features
- Text reversal API
- Database storage of all reversals
- Flexible database backend (SQLite or PostgreSQL)
- Docker support for development

## Configuration

### Database Setup
The application supports two database types:

#### SQLite (Default)
```bash
# Use SQLite (default if DB_TYPE not set)
export DB_TYPE=sqlite

# Optional: Custom SQLite database path
export DATABASE_URL=sqlite:///path/to/your/database.db
```

#### PostgreSQL
```bash
# Use PostgreSQL
export DB_TYPE=postgresql
export DATABASE_URL="postgresql://<username>:<password>@<host>:<port>/<database>"
```

### Initial Setup

1. Set up Python virtual environment and install dependencies:
```bash
python -m venv .virtualenv
source .virtualenv/bin/activate
pip install -r requirements.txt
```

2. Configure environment variables:
```bash
export SECRET_KEY="your-secret-key"  # Optional, defaults to dev-secret-key
# Set database configuration as shown above
```

3. Initialize the database:
```bash
flask db init
flask db migrate -m "Initial migration"
flask db upgrade
```

### Docker Development Setup
```bash
docker-compose -f compose.yml up

# First time only
docker-compose exec app flask db init

# After database schema changes
docker-compose exec app flask db migrate
docker-compose exec app flask db upgrade
```

## Usage

The application runs on port 5000 by default.

### Basic Usage
1. View the welcome page (shows current database configuration):
```bash
curl http://localhost:5000
```

2. Reverse any text by adding it to the path:
```bash
curl http://localhost:5000/wearenotalone
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
- Python 3.x
- Flask 3.x
- SQLAlchemy 2.x
- PostgreSQL (optional)
- Docker (optional, for development)