# Stage 1: Build dependencies
FROM python:3.13.7-alpine as builder

# Install build dependencies
RUN apk add --no-cache \
    postgresql-dev \
    gcc \
    musl-dev \
    libffi-dev \
    python3-dev

WORKDIR /install

# Copy and install requirements
COPY requirements.txt .
RUN pip install --no-cache-dir --prefix=/install -r requirements.txt

# Stage 2: Final image
FROM python:3.13.7-alpine

# Set environment variables
ENV PYTHONDONTWRITEBYTECODE=1 \
    PYTHONUNBUFFERED=1 \
    FLASK_APP=app.py \
    FLASK_RUN_HOST=0.0.0.0 \
    DB_TYPE=sqlite \
    PATH="/app/.local/bin:$PATH" \
    PYTHONPATH="/app/.local/lib/python3.13/site-packages:$PYTHONPATH"

WORKDIR /app

# Install runtime dependencies only
RUN apk add --no-cache \
    postgresql-client \
    curl \
    && addgroup -S appgroup \
    && adduser -S appuser -G appgroup \
    && mkdir -p /app/data /app/.local \
    && chown -R appuser:appgroup /app

# Copy installed dependencies from builder
COPY --from=builder /install /app/.local

# Copy application files
COPY --chown=appuser:appgroup app.py config.py ./

# Switch to non-root user
USER appuser

# Create volume for SQLite database
VOLUME ["/app/data"]

# Health check with reduced frequency for production
HEALTHCHECK --interval=1m --timeout=30s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:5000/ || exit 1

# Expose port
EXPOSE 5000

# Run the application with optimized settings
CMD ["python", "-m", "flask", "run", "--no-reload"]
