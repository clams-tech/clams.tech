# Clams Website and LNURL Server

A Sveltekit static website for clams.tech and LNURL server implementation that works with Phoenixd.

## Running the Website Locally

- `cd website`
- `bun i`
- `bun dev`

## Running the Blog Locally

- `cd blog`
- `npm install`
- `npm run serve`

## Running the Docs Locally

- `cd docs`
- `npm install`
- `npm start`

## Running the LNURL Server Locally

- `cargo run`

## Development with Docker (Website, Blog, Docs, Server)

To run all services together for local development and testing:

```bash
# Start all services
docker compose up --build

# Start specific services only
docker compose up website blog server

# Stop all services
docker compose down
```

Services will be available at:
- **Website**: http://localhost:5173
- **Blog**: http://localhost:1111
- **Docs**: http://localhost:3000  
- **Server**: http://localhost:8080

All services support hot reloading when you change files.

## Quick Start

For the simplest development experience, just run:

```bash
# Start all services
docker compose up

# Or run in background
docker compose up -d

# Stop all services
docker compose down
```

## Common Development Commands

```bash
# View logs for all services
docker compose logs

# View logs for specific service
docker compose logs blog

# Follow logs in real-time
docker compose logs -f

# Restart a specific service
docker compose restart blog

# Rebuild and restart all services
docker compose up --build

# View running services
docker compose ps
```

## Acknowledgements

The LNURL server is a fork of [lnurl-server](https://github.com/benthecarman/lnurl-server) that has been modified to work with Phoenixd instead of LND.
