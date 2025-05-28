# EchoLayer API Documentation

## Overview

The EchoLayer API provides endpoints for managing the decentralized attention ecosystem, including Echo Index calculations, content tracking, and reward distribution.

**Base URL**: `https://api.echolayers.xyz`  
**Version**: v1  
**Authentication**: JWT Bearer Token

---

## Authentication

### POST /auth/login
Authenticate user and receive JWT token.

**Request Body**:
```json
{
  "wallet_address": "string",
  "signature": "string",
  "message": "string"
}
```

**Response**:
```json
{
  "success": true,
  "data": {
    "token": "jwt_token_string",
    "user": {
      "id": "uuid",
      "wallet_address": "string",
      "username": "string",
      "echo_score": 0,
      "created_at": "timestamp"
    }
  }
}
```

### POST /auth/refresh
Refresh JWT token.

**Headers**: `Authorization: Bearer <token>`

**Response**:
```json
{
  "success": true,
  "data": {
    "token": "new_jwt_token_string"
  }
}
```

---

## User Management

### GET /users/profile
Get current user profile.

**Headers**: `Authorization: Bearer <token>`

**Response**:
```json
{
  "success": true,
  "data": {
    "id": "uuid",
    "wallet_address": "string",
    "username": "string",
    "email": "string",
    "echo_score": 1250,
    "rank": 42,
    "total_rewards": "100.50",
    "connected_platforms": ["twitter", "telegram"],
    "preferences": {
      "notifications": true,
      "public_profile": true
    },
    "stats": {
      "content_created": 15,
      "propagations_initiated": 8,
      "echo_drops_received": 12
    },
    "created_at": "timestamp",
    "updated_at": "timestamp"
  }
}
```

### PUT /users/profile
Update user profile.

**Headers**: `Authorization: Bearer <token>`

**Request Body**:
```json
{
  "username": "string",
  "email": "string",
  "preferences": {
    "notifications": true,
    "public_profile": false
  }
}
```

---

## Echo Index System

### GET /echo-index/calculate
Calculate Echo Index for specific content.

**Query Parameters**:
- `content_id` (required): Content identifier
- `platform` (required): Social platform (twitter, telegram, linkedin)

**Response**:
```json
{
  "success": true,
  "data": {
    "content_id": "string",
    "echo_index": 85.4,
    "components": {
      "odf": {
        "score": 78.2,
        "weight": 0.30,
        "weighted_score": 23.46
      },
      "awr": {
        "score": 92.8,
        "weight": 0.25,
        "weighted_score": 23.20
      },
      "tpm": {
        "score": 81.6,
        "weight": 0.25,
        "weighted_score": 20.40
      },
      "qf": {
        "score": 73.6,
        "weight": 0.20,
        "weighted_score": 14.72
      }
    },
    "calculated_at": "timestamp"
  }
}
```

### GET /echo-index/leaderboard
Get Echo Index leaderboard.

**Query Parameters**:
- `limit` (optional): Number of results (default: 50, max: 100)
- `offset` (optional): Pagination offset (default: 0)
- `timeframe` (optional): daily, weekly, monthly, all_time (default: all_time)

**Response**:
```json
{
  "success": true,
  "data": {
    "leaderboard": [
      {
        "rank": 1,
        "user_id": "uuid",
        "username": "user123",
        "echo_score": 2847.5,
        "total_content": 45,
        "avg_echo_index": 92.3
      }
    ],
    "pagination": {
      "total": 1250,
      "limit": 50,
      "offset": 0,
      "has_more": true
    }
  }
}
```

---

## Content Management

### POST /content/create
Create new content for tracking.

**Headers**: `Authorization: Bearer <token>`

**Request Body**:
```json
{
  "platform": "twitter",
  "external_id": "tweet_id_123",
  "content_type": "text",
  "title": "Amazing blockchain insight",
  "body": "Content body text...",
  "media_urls": ["https://example.com/image.jpg"],
  "tags": ["blockchain", "defi"],
  "platform_metadata": {
    "tweet_url": "https://twitter.com/user/status/123",
    "retweet_count": 0,
    "like_count": 0
  }
}
```

**Response**:
```json
{
  "success": true,
  "data": {
    "id": "uuid",
    "platform": "twitter",
    "external_id": "tweet_id_123",
    "status": "active",
    "echo_index": 0,
    "created_at": "timestamp"
  }
}
```

### GET /content/{id}
Get content details by ID.

**Path Parameters**:
- `id`: Content UUID

**Response**:
```json
{
  "success": true,
  "data": {
    "id": "uuid",
    "user_id": "uuid",
    "platform": "twitter",
    "external_id": "tweet_id_123",
    "content_type": "text",
    "title": "Amazing blockchain insight",
    "body": "Content body text...",
    "echo_index": 85.4,
    "propagation_count": 12,
    "total_rewards": "25.75",
    "status": "active",
    "platform_metadata": {
      "tweet_url": "https://twitter.com/user/status/123",
      "retweet_count": 45,
      "like_count": 123,
      "reply_count": 8
    },
    "created_at": "timestamp",
    "updated_at": "timestamp"
  }
}
```

### GET /content/search
Search content with filters.

**Query Parameters**:
- `q` (optional): Search query
- `platform` (optional): Filter by platform
- `user_id` (optional): Filter by user
- `min_echo_index` (optional): Minimum Echo Index score
- `tags` (optional): Comma-separated tags
- `limit` (optional): Results limit (default: 20, max: 100)
- `offset` (optional): Pagination offset

**Response**:
```json
{
  "success": true,
  "data": {
    "content": [
      {
        "id": "uuid",
        "title": "Amazing blockchain insight",
        "echo_index": 85.4,
        "platform": "twitter",
        "created_at": "timestamp"
      }
    ],
    "pagination": {
      "total": 150,
      "limit": 20,
      "offset": 0,
      "has_more": true
    }
  }
}
```

---

## Propagation Tracking

### POST /propagation/track
Track content propagation event.

**Headers**: `Authorization: Bearer <token>`

**Request Body**:
```json
{
  "content_id": "uuid",
  "propagation_type": "share",
  "source_platform": "twitter",
  "target_platform": "telegram",
  "source_user_id": "uuid",
  "target_user_id": "uuid",
  "metadata": {
    "share_url": "https://t.me/channel/123",
    "engagement_metrics": {
      "views": 250,
      "interactions": 12
    }
  }
}
```

**Response**:
```json
{
  "success": true,
  "data": {
    "id": "uuid",
    "content_id": "uuid",
    "propagation_type": "share",
    "echo_boost": 1.2,
    "reward_amount": "5.25",
    "created_at": "timestamp"
  }
}
```

### GET /propagation/history
Get propagation history for user or content.

**Query Parameters**:
- `content_id` (optional): Filter by content
- `user_id` (optional): Filter by user
- `limit` (optional): Results limit (default: 50)
- `offset` (optional): Pagination offset

**Response**:
```json
{
  "success": true,
  "data": {
    "propagations": [
      {
        "id": "uuid",
        "content_id": "uuid",
        "propagation_type": "share",
        "source_platform": "twitter",
        "target_platform": "telegram",
        "echo_boost": 1.2,
        "reward_amount": "5.25",
        "created_at": "timestamp"
      }
    ],
    "pagination": {
      "total": 75,
      "limit": 50,
      "offset": 0,
      "has_more": true
    }
  }
}
```

---

## Echo Drop Rewards

### GET /rewards/balance
Get user reward balance.

**Headers**: `Authorization: Bearer <token>`

**Response**:
```json
{
  "success": true,
  "data": {
    "total_balance": "156.75",
    "available_balance": "142.50",
    "pending_balance": "14.25",
    "token_symbol": "ECH",
    "wallet_address": "solana_wallet_address",
    "last_updated": "timestamp"
  }
}
```

### GET /rewards/history
Get reward transaction history.

**Headers**: `Authorization: Bearer <token>`

**Query Parameters**:
- `type` (optional): earned, withdrawn, pending
- `limit` (optional): Results limit (default: 50)
- `offset` (optional): Pagination offset

**Response**:
```json
{
  "success": true,
  "data": {
    "transactions": [
      {
        "id": "uuid",
        "type": "earned",
        "amount": "12.50",
        "reason": "content_propagation",
        "content_id": "uuid",
        "transaction_hash": "solana_tx_hash",
        "status": "completed",
        "created_at": "timestamp"
      }
    ],
    "pagination": {
      "total": 25,
      "limit": 50,
      "offset": 0,
      "has_more": false
    }
  }
}
```

### POST /rewards/withdraw
Withdraw rewards to wallet.

**Headers**: `Authorization: Bearer <token>`

**Request Body**:
```json
{
  "amount": "50.00",
  "wallet_address": "solana_wallet_address"
}
```

**Response**:
```json
{
  "success": true,
  "data": {
    "transaction_id": "uuid",
    "amount": "50.00",
    "fee": "0.25",
    "net_amount": "49.75",
    "transaction_hash": "solana_tx_hash",
    "status": "pending",
    "estimated_completion": "timestamp"
  }
}
```

---

## Analytics

### GET /analytics/dashboard
Get user analytics dashboard data.

**Headers**: `Authorization: Bearer <token>`

**Query Parameters**:
- `timeframe` (optional): 7d, 30d, 90d, 1y (default: 30d)

**Response**:
```json
{
  "success": true,
  "data": {
    "timeframe": "30d",
    "summary": {
      "total_echo_score": 1250,
      "content_count": 15,
      "propagation_count": 8,
      "rewards_earned": "45.75",
      "rank_change": +5
    },
    "echo_score_trend": [
      {
        "date": "2024-01-01",
        "score": 1200
      },
      {
        "date": "2024-01-02",
        "score": 1250
      }
    ],
    "platform_breakdown": {
      "twitter": {
        "content_count": 8,
        "avg_echo_index": 85.2,
        "total_rewards": "25.50"
      },
      "telegram": {
        "content_count": 4,
        "avg_echo_index": 72.8,
        "total_rewards": "12.25"
      }
    },
    "top_content": [
      {
        "id": "uuid",
        "title": "Best performing content",
        "echo_index": 95.8,
        "rewards": "15.75"
      }
    ]
  }
}
```

### GET /analytics/global
Get global platform analytics.

**Query Parameters**:
- `timeframe` (optional): 24h, 7d, 30d (default: 24h)

**Response**:
```json
{
  "success": true,
  "data": {
    "timeframe": "24h",
    "global_stats": {
      "total_users": 12450,
      "active_users": 3250,
      "total_content": 45670,
      "total_propagations": 18920,
      "total_rewards_distributed": "15678.50"
    },
    "platform_stats": {
      "twitter": {
        "content_count": 25000,
        "avg_echo_index": 78.5,
        "top_performer": {
          "content_id": "uuid",
          "echo_index": 98.7
        }
      }
    },
    "trending_content": [
      {
        "id": "uuid",
        "title": "Trending content title",
        "echo_index": 95.8,
        "propagation_count": 45
      }
    ]
  }
}
```

---

## Platform Integration

### POST /platforms/connect
Connect social media platform.

**Headers**: `Authorization: Bearer <token>`

**Request Body**:
```json
{
  "platform": "twitter",
  "access_token": "platform_access_token",
  "access_token_secret": "platform_access_token_secret",
  "platform_user_id": "twitter_user_id"
}
```

**Response**:
```json
{
  "success": true,
  "data": {
    "platform": "twitter",
    "status": "connected",
    "platform_username": "twitter_handle",
    "verification_status": "verified",
    "connected_at": "timestamp"
  }
}
```

### GET /platforms/status
Get connected platforms status.

**Headers**: `Authorization: Bearer <token>`

**Response**:
```json
{
  "success": true,
  "data": {
    "connected_platforms": [
      {
        "platform": "twitter",
        "status": "active",
        "username": "twitter_handle",
        "last_sync": "timestamp",
        "sync_status": "success"
      },
      {
        "platform": "telegram",
        "status": "active",
        "username": "telegram_username",
        "last_sync": "timestamp",
        "sync_status": "success"
      }
    ]
  }
}
```

---

## Error Responses

### Standard Error Format
```json
{
  "success": false,
  "error": {
    "code": "ERROR_CODE",
    "message": "Human readable error message",
    "details": {
      "field": "validation error details"
    }
  }
}
```

### Common Error Codes
- `UNAUTHORIZED` (401): Invalid or missing authentication
- `FORBIDDEN` (403): Insufficient permissions
- `NOT_FOUND` (404): Resource not found
- `VALIDATION_ERROR` (422): Request validation failed
- `RATE_LIMIT_EXCEEDED` (429): Too many requests
- `INTERNAL_ERROR` (500): Server error

---

## Rate Limiting

- **General API**: 100 requests per minute per user
- **Analytics endpoints**: 20 requests per minute per user
- **Authentication endpoints**: 10 requests per minute per IP

Rate limit headers included in all responses:
- `X-RateLimit-Limit`: Request limit per window
- `X-RateLimit-Remaining`: Requests remaining in current window
- `X-RateLimit-Reset`: Time when rate limit resets

---

## Webhooks

### Webhook Events
EchoLayer can send webhooks for the following events:

- `content.created`: New content created
- `content.propagated`: Content propagation event
- `reward.earned`: User earned rewards
- `echo_index.updated`: Echo Index score updated

### Webhook Payload Format
```json
{
  "event": "content.propagated",
  "timestamp": "timestamp",
  "data": {
    "content_id": "uuid",
    "propagation_type": "share",
    "echo_boost": 1.2,
    "reward_amount": "5.25"
  }
}
```

---

## SDKs and Libraries

### JavaScript/TypeScript SDK
```bash
npm install @echolayer/sdk
```

### Python SDK
```bash
pip install echolayer-sdk
```

### Rust SDK
```toml
[dependencies]
echolayer-sdk = "0.1.0"
```

---

For more detailed examples and implementation guides, visit our [Developer Portal](https://developers.echolayers.xyz). 