# URL Shortener

A full-stack URL shortener application built with Rust (Axum), Redis, and vanilla HTML/CSS/JavaScript.

## Features

- ✅ Shorten long URLs with 6-character random IDs
- ✅ Redirect shortened URLs to original destinations
- ✅ Modern, responsive web interface
- ✅ Copy-to-clipboard functionality
- ✅ Redis-based storage for fast lookups
- ✅ 404 handling for non-existent URLs

## Tech Stack

- **Backend**: Rust with Axum framework
- **Database**: Redis (key-value store)
- **Frontend**: Vanilla HTML, CSS, and JavaScript
- **ID Generation**: nanoid (6-character random IDs)

## Prerequisites

1. **Rust** (install from [rustup.rs](https://rustup.rs/))
2. **Redis** (running on a server - could be local or remote)

## Quick Start

1. **Setup environment variables**:

   ```bash
   cp .env.example .env
   ```

   Edit `.env` and update the Redis connection details:

   ```
   REDIS_URL=redis://username:password@pi:6379
   SERVER_HOST=0.0.0.0
   SERVER_PORT=3000
   BASE_URL=http://localhost:3000
   ```

2. **Run the application**:

   ```bash
   cd url-shortener
   cargo run
   ```

3. **Open your browser** and navigate to:
   ```
   http://localhost:3000
   ```

## API Endpoints

### POST /api

Create a shortened URL.

**Request:**

```json
{
  "url": "https://example.com"
}
```

**Response:**

```json
{
  "short": "http://localhost:3000/abc123"
}
```

### GET /:id

Redirect to the original URL or 404 page if not found.

## Project Structure

```
url-shortener/
├── Cargo.toml          # Rust dependencies
├── src/
│   └── main.rs         # Rust backend with Axum
├── static/
│   ├── index.html      # Frontend interface
│   └── 404.html        # 404 error page
└── README.md
```

## Configuration

The application uses environment variables for configuration. Copy `.env.example` to `.env` and modify as needed:

- `REDIS_URL`: Redis connection string (format: `redis://username:password@hostname:port`)
- `SERVER_HOST`: Host to bind the server to (default: `0.0.0.0`)
- `SERVER_PORT`: Port to run the server on (default: `3000`)
- `BASE_URL`: Base URL for generating short links (update for production)

## Development

- The backend serves static files from the `static/` directory
- Redis stores shortened URLs with the format: `key = short_id, value = original_url`
- CORS is enabled for development

## Production Considerations

For production deployment, consider:

- Use environment variables for configuration
- Set up proper CORS policies
- Use a production Redis instance
- Add rate limiting
- Add URL validation and security checks
- Use HTTPS
- Add analytics and monitoring
