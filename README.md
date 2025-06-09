# Web Spider: Distributed Rust Web Crawler

A fast, scalable, and concurrent web crawler built in Rust, using Redis as a distributed queue and Docker for orchestration.

---

## Features

- Built in Rust for high performance.
- Redis-backed crawling queue (BFS-style).
- Respects normalization and deduplication of URLs.
- Containerized using Docker & Docker Compose.
- Configurable concurrency and depth control.

---

## Getting Started

### Prerequisites

Make sure you have the following installed:

- [Docker](https://docs.docker.com/get-docker/)
- [Docker Compose](https://docs.docker.com/compose/)

---

### Build and Run

Clone the repo:

```bash
git clone https://github.com/starkbaknet/web-spider.git
cd web-spider
```

Build and start the crawler:

```bash
docker compose up --build
```

To stop and clean everything:

```bash
docker compose down -v
```

---

## Configuration

All crawler settings are passed via environment variables inside `docker-compose.yml`:

| Variable          | Description                        | Default                |
| ----------------- | ---------------------------------- | ---------------------- |
| `REDIS_HOST`      | Redis hostname                     | `redis`                |
| `REDIS_PORT`      | Redis port                         | `6379`                 |
| `STARTING_URL`    | The initial seed URL to crawl from | `https://starkbak.net` |
| `MAX_CONCURRENCY` | Number of concurrent tasks         | `10`                   |
| `MAX_PAGES`       | Maximum number of pages to crawl   | `100`                  |

Modify these values in the `docker-compose.yml` file as needed and then rename `.env.example` to `.env` and modify values same as `docker-compose.yml`.

---

## Project Structure

```
.
├── src/                 # Rust source code
├── Dockerfile           # Multistage Docker build
├── docker-compose.yml   # Service orchestration
├── Cargo.toml
├── .env.example.        # should rename to .env
└── README.md
```

---
