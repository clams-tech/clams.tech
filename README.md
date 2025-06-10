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

## Development with Docker (Website, Docs, Server)

To run the main services together for local development and testing:

```bash
# Start website, docs, and server
docker compose -f docker-compose.dev.yml up --build

# Start specific services only
docker compose -f docker-compose.dev.yml up website server

# Stop all services
docker compose -f docker-compose.dev.yml down
```

Services will be available at:
- **Website**: http://localhost:5173
- **Docs**: http://localhost:3000  
- **Server**: http://localhost:8080

**Blog**: Run locally due to Docker complexity:
```bash
cd blog
npm run serve
```
Blog will be available at: http://localhost:1111

All services support hot reloading when you change files.

## Acknowledgements

The LNURL server is a fork of [lnurl-server](https://github.com/benthecarman/lnurl-server) that has been modified to work with Phoenixd instead of LND.
