# Environment Setup Guide

This guide covers the environment configuration required for EchoLayer development and production deployments.

## Quick Start

1. Copy the environment template:
```bash
cp docker/.env.example .env
```

2. Update the values in `.env` according to your environment
3. Run the development environment:
```bash
npm run docker:dev
```

## Environment Variables

### Database Configuration

| Variable | Description | Default | Required |
|----------|-------------|---------|----------|
| `POSTGRES_DB` | PostgreSQL database name | `echolayer` | Yes |
| `POSTGRES_USER` | PostgreSQL username | `echolayer` | Yes |
| `POSTGRES_PASSWORD` | PostgreSQL password | - | Yes |
| `DATABASE_URL` | Full database connection string | - | Yes |

### Redis Configuration

| Variable | Description | Default | Required |
|----------|-------------|---------|----------|
| `REDIS_PASSWORD` | Redis authentication password | - | Yes |
| `REDIS_URL` | Full Redis connection string | - | Yes |

### Security Configuration

| Variable | Description | Default | Required |
|----------|-------------|---------|----------|
| `JWT_SECRET` | JWT signing secret (min 32 chars) | - | Yes |
| `ENCRYPTION_KEY` | Data encryption key | - | Yes |

### Blockchain Configuration

| Variable | Description | Default | Required |
|----------|-------------|---------|----------|
| `SOLANA_RPC_URL` | Solana RPC endpoint | `https://api.devnet.solana.com` | Yes |
| `SOLANA_PRIVATE_KEY` | Solana wallet private key | - | Yes |

### Frontend Configuration

| Variable | Description | Default | Required |
|----------|-------------|---------|----------|
| `NEXT_PUBLIC_API_URL` | Backend API URL | `http://localhost:8080` | Yes |
| `NEXT_PUBLIC_SOLANA_NETWORK` | Solana network | `devnet` | Yes |
| `NEXT_PUBLIC_SITE_URL` | Frontend URL | `http://localhost:3000` | Yes |

### Backend Configuration

| Variable | Description | Default | Required |
|----------|-------------|---------|----------|
| `RUST_ENV` | Runtime environment | `development` | Yes |
| `RUST_LOG` | Log level | `debug` | No |
| `SERVER_HOST` | Server bind address | `0.0.0.0` | No |
| `SERVER_PORT` | Server port | `8080` | No |
| `CORS_ORIGINS` | Allowed CORS origins | `http://localhost:3000` | Yes |

### Social Media APIs

#### Twitter/X Integration
| Variable | Description | Required |
|----------|-------------|----------|
| `TWITTER_API_KEY` | Twitter API key | Yes |
| `TWITTER_API_SECRET` | Twitter API secret | Yes |
| `TWITTER_ACCESS_TOKEN` | Twitter access token | Yes |
| `TWITTER_ACCESS_TOKEN_SECRET` | Twitter access token secret | Yes |

#### Telegram Integration
| Variable | Description | Required |
|----------|-------------|----------|
| `TELEGRAM_BOT_TOKEN` | Telegram bot token | Yes |

#### LinkedIn Integration
| Variable | Description | Required |
|----------|-------------|----------|
| `LINKEDIN_CLIENT_ID` | LinkedIn client ID | Yes |
| `LINKEDIN_CLIENT_SECRET` | LinkedIn client secret | Yes |

### Monitoring Configuration

| Variable | Description | Default | Required |
|----------|-------------|---------|----------|
| `GRAFANA_ADMIN_PASSWORD` | Grafana admin password | - | Yes |

## Environment-Specific Setup

### Development Environment

```bash
# Database
POSTGRES_PASSWORD=dev_password_123
REDIS_PASSWORD=dev_redis_123

# Security (use strong secrets in production)
JWT_SECRET=dev_jwt_secret_key_minimum_32_characters
ENCRYPTION_KEY=dev_encryption_key_here

# Solana (use devnet for development)
SOLANA_RPC_URL=https://api.devnet.solana.com
NEXT_PUBLIC_SOLANA_NETWORK=devnet

# APIs (development)
NEXT_PUBLIC_API_URL=http://localhost:8080
NEXT_PUBLIC_SITE_URL=http://localhost:3000

# Logging
RUST_ENV=development
RUST_LOG=debug

# CORS
CORS_ORIGINS=http://localhost:3000,http://localhost:3001
```

### Production Environment

```bash
# Database (use strong passwords)
POSTGRES_PASSWORD=your_secure_production_password
REDIS_PASSWORD=your_secure_redis_password

# Security (generate strong secrets)
JWT_SECRET=your_production_jwt_secret_minimum_32_characters
ENCRYPTION_KEY=your_production_encryption_key

# Solana (use mainnet for production)
SOLANA_RPC_URL=https://api.mainnet-beta.solana.com
NEXT_PUBLIC_SOLANA_NETWORK=mainnet-beta

# APIs (production URLs)
NEXT_PUBLIC_API_URL=https://api.echolayers.xyz
NEXT_PUBLIC_SITE_URL=https://echolayers.xyz

# Logging
RUST_ENV=production
RUST_LOG=info

# CORS (production domains only)
CORS_ORIGINS=https://echolayers.xyz,https://www.echolayers.xyz

# SSL
SSL_CERT_PATH=/etc/ssl/certs/echolayers.xyz.crt
SSL_KEY_PATH=/etc/ssl/private/echolayers.xyz.key
```

## Security Best Practices

### Password Generation

Generate secure passwords using:
```bash
# Generate random password
openssl rand -base64 32

# Generate JWT secret
openssl rand -hex 32
```

### Secret Management

- **Development**: Use `.env` files (never commit to git)
- **Production**: Use environment variables or secret management services
- **Docker**: Use Docker secrets for sensitive data

### Environment Isolation

- Use different API keys for development and production
- Separate database instances for each environment
- Use different Solana networks (devnet vs mainnet)

## Troubleshooting

### Common Issues

1. **Database Connection Failed**
   - Check `DATABASE_URL` format
   - Ensure PostgreSQL is running
   - Verify credentials

2. **Redis Connection Failed**
   - Check `REDIS_URL` format
   - Ensure Redis is running
   - Verify password

3. **JWT Errors**
   - Ensure `JWT_SECRET` is at least 32 characters
   - Check for special characters in secret

4. **CORS Errors**
   - Add frontend URL to `CORS_ORIGINS`
   - Check protocol (http vs https)
   - Verify port numbers

### Health Checks

Use these endpoints to verify services:

```bash
# Backend health
curl http://localhost:8080/health

# Frontend health
curl http://localhost:3000/api/health

# Database connection
psql $DATABASE_URL -c "SELECT 1;"

# Redis connection
redis-cli -u $REDIS_URL ping
```

## Environment Validation

Create a script to validate your environment:

```bash
#!/bin/bash
# validate-env.sh

echo "Validating EchoLayer environment..."

# Check required variables
required_vars=(
  "POSTGRES_PASSWORD"
  "REDIS_PASSWORD"
  "JWT_SECRET"
  "SOLANA_RPC_URL"
  "NEXT_PUBLIC_API_URL"
)

for var in "${required_vars[@]}"; do
  if [ -z "${!var}" ]; then
    echo "❌ Missing required variable: $var"
    exit 1
  else
    echo "✅ $var is set"
  fi
done

echo "🎉 Environment validation passed!"
```

Run validation:
```bash
chmod +x validate-env.sh
./validate-env.sh
``` 