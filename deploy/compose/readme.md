# Us - Local Development Environment

Docker Compose setup for local environment.

## Services

All services are accessible via Traefik reverse proxy:

- **Traefik Dashboard**: http://localhost:8080
- **NATS Monitoring**: http://nats.us.local
- **Garage S3 API**: http://s3.us.local
- **Garage Admin**: http://garage.us.local
- **Kratos Public**: http://auth.us.local
- **Kratos Admin**: http://auth-admin.us.local
- **Hydra Public**: http://oauth.us.local
- **Hydra Admin**: http://oauth-admin.us.local

## Prerequisites

Add to `/etc/hosts`:

127.0.0.1 nats.us.local
127.0.0.1 s3.us.local
127.0.0.1 garage.us.local
127.0.0.1 auth.us.local
127.0.0.1 auth-admin.us.local
127.0.0.1 oauth.us.local
127.0.0.1 oauth-admin.us.local
127.0.0.1 app.us.local

## Setup Environment Files

Copy example env files and customize:

```bash
cd deploy/compose/env
cp postgres.env.example postgres.env
cp garage.env.example garage.env
cp kratos.env.example kratos.env
cp hydra.env.example hydra.env
```

Edit each .env file and change default passwords/secrets.

## Usage
```bash
# Start all services
docker compose -f docker-compose.dev.yaml up -d

# View logs
docker compose -f docker-compose.dev.yaml logs -f

# Check health
docker compose -f docker-compose.dev.yaml ps

# Stop all services
docker compose -f docker-compose.dev.yaml down

# Stop and remove volumes
docker compose -f docker-compose.dev.yaml down -v
```

## Health Checks

All services have health checks configured. Check status:
```bash
docker compose -f docker-compose.dev.yaml ps
```

## Initial Setup

After first start, create Garage bucket:
```bash
docker exec us-garage garage bucket create us-media
docker exec us-garage garage key create us-media-key
```

Verify services:
```bash
curl http://auth.us.local/health/ready
curl http://oauth.us.local/health/ready
curl http://nats.us.local/healthz
```

## Environment Files

Each service has its own env file in env/:
- postgres.env - PostgreSQL credentials
- garage.env - Garage S3 secrets
- kratos.env - Kratos config
- hydra.env - Hydra config

WARNING: Never commit these files with production secrets!