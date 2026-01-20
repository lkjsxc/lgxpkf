# Docker Compose

## Services

- `app` runs the distroless image with baked migrations at `/app/db/migrations`.
- `db` stores data in the named volume `db_data` (no bind mounts).
- `PUBLIC_BASE_URL` must be set so GIS redirect URIs are deterministic.
- Frontend TypeScript assets are compiled during the image build.

## Usage

- `docker compose build` builds the app image.
- `docker compose up -d` starts the stack.

## Verification

- `GET /health` for process liveness.
- `GET /ready` for database readiness.
