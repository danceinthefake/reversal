import os
from pathlib import Path

basedir = os.path.abspath(os.path.dirname(__file__))

DEBUG = False
TESTING = False
CSRF_ENABLED = True
SECRET_KEY = os.getenv('SECRET_KEY', 'dev-secret-key')
SQLALCHEMY_TRACK_MODIFICATIONS = False

# Database configuration
DB_TYPE = os.getenv('DB_TYPE', 'sqlite').lower()  # Default to SQLite if not specified

if DB_TYPE == 'postgresql':
    SQLALCHEMY_DATABASE_URI = os.getenv(
        'DATABASE_URL',
        'postgresql://postgres:postgres@localhost:5432/reversal'
    )
elif DB_TYPE == 'sqlite':
    # Create a 'data' directory if it doesn't exist
    Path(os.path.join(basedir, 'data')).mkdir(exist_ok=True)
    SQLALCHEMY_DATABASE_URI = os.getenv(
        'DATABASE_URL',
        f'sqlite:///{os.path.join(basedir, "data", "reversal.db")}'
    )
else:
    raise ValueError(f"Unsupported database type: {DB_TYPE}. Use 'postgresql' or 'sqlite'")
