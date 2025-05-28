# EchoLayer API Documentation

## Overview

The EchoLayer API provides endpoints for managing users, content, and the Echo Index™ system. All API endpoints are prefixed with `/api/v1`.

## Base URL

```
http://localhost:8080/api/v1
```

## Authentication

Most endpoints require authentication using JWT tokens. Include the token in the Authorization header:

```
Authorization: Bearer <your-jwt-token>
```

## Response Format

All responses follow a consistent format:

```json
{
  "success": true,
  "data": {},
  "error": null,
  "timestamp": "2024-01-01T00:00:00Z"
}
```

## Error Handling

Error responses include detailed information:

```json
{
  "success": false,
  "data": null,
  "error": "Error message",
  "timestamp": "2024-01-01T00:00:00Z"
}
```

## Endpoints

### Health Check

#### GET /health

Check API health status.

**Response:**
```json
{
  "success": true,
  "data": {
    "status": "healthy",
    "version": "1.0.0",
    "timestamp": "2024-01-01T00:00:00Z"
  }
}
```

### User Management

#### POST /users

Create a new user account.

**Request Body:**
```json
{
  "username": "alice_crypto",
  "email": "alice@example.com",
  "social_accounts": []
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "username": "alice_crypto",
    "email": "alice@example.com",
    "wallet_address": null,
    "echo_score": 0.0,
    "total_content_created": 0,
    "total_rewards_earned": 0.0,
    "created_at": "2024-01-01T00:00:00Z",
    "updated_at": "2024-01-01T00:00:00Z"
  }
}
```

#### GET /users/{id}

Get user profile by ID.

**Path Parameters:**
- `id` (string): User UUID

**Response:**
```json
{
  "success": true,
  "data": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "username": "alice_crypto",
    "email": "alice@example.com",
    "wallet_address": "9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM",
    "echo_score": 85.5,
    "total_content_created": 12,
    "total_rewards_earned": 1250.75,
    "created_at": "2024-01-01T00:00:00Z",
    "updated_at": "2024-01-01T00:00:00Z"
  }
}
```

#### GET /users/{id}/profile

Get complete user profile with social accounts and recent content.

**Response:**
```json
{
  "success": true,
  "data": {
    "user": { /* user object */ },
    "social_accounts": [
      {
        "id": "platform_id",
        "platform": "twitter",
        "username": "alice_crypto",
        "verified": true
      }
    ],
    "recent_content": [
      {
        "id": "content_id",
        "text": "Latest thoughts on DeFi...",
        "platform": "twitter",
        "echo_score": 78.5,
        "propagation_count": 25,
        "created_at": "2024-01-01T00:00:00Z"
      }
    ]
  }
}
```

### Content Management

#### POST /content

Create new content and calculate Echo Index™.

**Request Body:**
```json
{
  "text": "The future of decentralized attention is here with innovative signal-aware technology that tracks authentic influence across platforms.",
  "platform": "twitter",
  "original_url": "https://twitter.com/user/status/123456789"
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "id": "content-uuid",
    "author_id": "user-uuid",
    "text": "The future of decentralized attention...",
    "platform": "twitter",
    "original_url": "https://twitter.com/user/status/123456789",
    "echo_index": {
      "originality_depth_factor": 0.85,
      "audience_weight_rating": 0.72,
      "transmission_path_mapping": 0.0,
      "quote_frequency": 0.0,
      "overall_score": 0.47
    },
    "propagation_count": 0,
    "total_interactions": 0,
    "created_at": "2024-01-01T00:00:00Z",
    "updated_at": "2024-01-01T00:00:00Z"
  }
}
```

#### GET /content/{id}

Get content by ID with current Echo Index™.

**Path Parameters:**
- `id` (string): Content UUID

**Response:**
```json
{
  "success": true,
  "data": {
    "id": "content-uuid",
    "author_id": "user-uuid",
    "text": "Content text...",
    "platform": "twitter",
    "original_url": "https://twitter.com/user/status/123456789",
    "echo_index": {
      "originality_depth_factor": 0.85,
      "audience_weight_rating": 0.72,
      "transmission_path_mapping": 0.45,
      "quote_frequency": 0.23,
      "overall_score": 0.62
    },
    "propagation_count": 15,
    "total_interactions": 87,
    "created_at": "2024-01-01T00:00:00Z",
    "updated_at": "2024-01-01T12:30:00Z"
  }
}
```

#### GET /content

Get paginated list of content with filters.

**Query Parameters:**
- `page` (integer, optional): Page number (default: 1)
- `limit` (integer, optional): Items per page (default: 20)
- `platform` (string, optional): Filter by platform
- `author_id` (string, optional): Filter by author ID
- `sort` (string, optional): Sort field (echo_score, created_at, propagation_count)
- `order` (string, optional): Sort order (asc, desc)

**Response:**
```json
{
  "success": true,
  "data": {
    "content": [
      { /* content objects */ }
    ],
    "pagination": {
      "page": 1,
      "limit": 20,
      "total": 150,
      "total_pages": 8
    }
  }
}
```

### Echo Index™ Calculation

#### POST /content/{id}/calculate-echo-index

Manually trigger Echo Index™ recalculation for content.

**Path Parameters:**
- `id` (string): Content UUID

**Response:**
```json
{
  "success": true,
  "data": {
    "content_id": "content-uuid",
    "echo_index": {
      "originality_depth_factor": 0.85,
      "audience_weight_rating": 0.72,
      "transmission_path_mapping": 0.45,
      "quote_frequency": 0.23,
      "overall_score": 0.62
    },
    "updated_at": "2024-01-01T12:30:00Z"
  }
}
```

### Propagation Tracking

#### POST /content/{id}/propagations

Record content propagation event.

**Path Parameters:**
- `id` (string): Content UUID

**Request Body:**
```json
{
  "propagator_id": "user-uuid",
  "platform": "telegram",
  "propagation_type": "share",
  "weight": 0.8
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "propagation_id": "propagation-uuid",
    "content_id": "content-uuid",
    "propagator_id": "user-uuid",
    "platform": "telegram",
    "propagation_type": "share",
    "weight": 0.8,
    "timestamp": "2024-01-01T12:30:00Z"
  }
}
```

#### GET /content/{id}/propagations

Get propagation history for content.

**Response:**
```json
{
  "success": true,
  "data": {
    "propagations": [
      {
        "id": "propagation-uuid",
        "propagator_id": "user-uuid",
        "platform": "telegram",
        "propagation_type": "share",
        "weight": 0.8,
        "timestamp": "2024-01-01T12:30:00Z"
      }
    ],
    "total_propagations": 15,
    "platforms": ["twitter", "telegram", "linkedin"],
    "propagation_velocity": 2.5
  }
}
```

### Analytics

#### GET /analytics/echo-index

Get system-wide Echo Index™ analytics.

**Query Parameters:**
- `timeframe` (string, optional): Time period (24h, 7d, 30d, all)
- `platform` (string, optional): Filter by platform

**Response:**
```json
{
  "success": true,
  "data": {
    "timeframe": "7d",
    "total_content": 1500,
    "average_echo_score": 0.65,
    "top_performers": [
      {
        "content_id": "uuid",
        "echo_score": 0.95,
        "author": "alice_crypto"
      }
    ],
    "platform_distribution": {
      "twitter": 45,
      "telegram": 30,
      "linkedin": 25
    },
    "trend_data": [
      {
        "date": "2024-01-01",
        "average_score": 0.62,
        "content_count": 125
      }
    ]
  }
}
```

#### GET /analytics/propagation

Get propagation analytics.

**Response:**
```json
{
  "success": true,
  "data": {
    "total_propagations": 5000,
    "propagation_velocity": 3.2,
    "most_viral_content": [
      {
        "content_id": "uuid",
        "propagation_count": 150,
        "reach": 50000
      }
    ],
    "platform_propagation": {
      "twitter": 2000,
      "telegram": 1800,
      "linkedin": 1200
    }
  }
}
```

### Rewards

#### GET /users/{id}/rewards

Get user's reward history and statistics.

**Response:**
```json
{
  "success": true,
  "data": {
    "total_earned": 2500.75,
    "pending_rewards": 125.50,
    "reward_history": [
      {
        "id": "reward-uuid",
        "amount": 50.25,
        "reason": "High Echo Index content",
        "content_id": "content-uuid",
        "timestamp": "2024-01-01T12:30:00Z"
      }
    ],
    "leaderboard_position": 15
  }
}
```

## Rate Limiting

- API calls are limited to 1000 requests per hour per IP address
- Authenticated users have higher limits (5000 requests per hour)
- Rate limit headers are included in responses:
  - `X-RateLimit-Limit`: Maximum requests allowed
  - `X-RateLimit-Remaining`: Remaining requests in current window
  - `X-RateLimit-Reset`: Timestamp when rate limit resets

## Error Codes

| Code | Description |
|------|-------------|
| 400  | Bad Request - Invalid input data |
| 401  | Unauthorized - Missing or invalid authentication |
| 403  | Forbidden - Insufficient permissions |
| 404  | Not Found - Resource does not exist |
| 409  | Conflict - Resource already exists |
| 429  | Too Many Requests - Rate limit exceeded |
| 500  | Internal Server Error - Server-side error |

## WebSocket API

Real-time updates are available via WebSocket connection:

```
ws://localhost:8080/ws
```

### Events

#### echo_index_updated
```json
{
  "event": "echo_index_updated",
  "data": {
    "content_id": "uuid",
    "echo_index": { /* echo index object */ }
  }
}
```

#### propagation_recorded
```json
{
  "event": "propagation_recorded",
  "data": {
    "content_id": "uuid",
    "propagation": { /* propagation object */ }
  }
}
```

## SDK Examples

### JavaScript/TypeScript

```typescript
import { EchoLayerAPI } from '@echolayer/sdk';

const api = new EchoLayerAPI({
  baseURL: 'http://localhost:8080/api/v1',
  token: 'your-jwt-token'
});

// Create content
const content = await api.content.create({
  text: 'My innovative DeFi analysis...',
  platform: 'twitter',
  originalUrl: 'https://twitter.com/user/status/123'
});

// Get Echo Index
const echoIndex = await api.content.getEchoIndex(content.id);
```

### Python

```python
from echolayer_sdk import EchoLayerAPI

api = EchoLayerAPI(
    base_url='http://localhost:8080/api/v1',
    token='your-jwt-token'
)

# Create content
content = api.content.create(
    text='My innovative DeFi analysis...',
    platform='twitter',
    original_url='https://twitter.com/user/status/123'
)

# Get Echo Index
echo_index = api.content.get_echo_index(content['id'])
```

## Testing

Use the provided test data and endpoints for development:

```bash
# Health check
curl http://localhost:8080/api/v1/health

# Create test user
curl -X POST http://localhost:8080/api/v1/users \
  -H "Content-Type: application/json" \
  -d '{"username":"test_user","email":"test@example.com","social_accounts":[]}'
``` 