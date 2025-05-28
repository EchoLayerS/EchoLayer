# EchoLayer Deployment Guide

## Overview

This guide covers the deployment process for EchoLayer across different environments: development, staging, and production. The system supports multiple deployment strategies including Docker Compose for development and Kubernetes for production.

## Prerequisites

### System Requirements

**Minimum Hardware Requirements:**
- CPU: 4 cores (8 recommended)
- RAM: 8GB (16GB recommended)
- Storage: 100GB SSD (500GB recommended)
- Network: 1Gbps connection

**Software Dependencies:**
- Docker 20.0+
- Docker Compose 2.0+
- Node.js 18+
- Rust 1.75+
- PostgreSQL 15+
- Redis 7+
- Nginx 1.20+

### Development Tools

```bash
# Install required tools
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
nvm install 18 && nvm use 18

# Install Docker
curl -fsSL https://get.docker.com -o get-docker.sh
sh get-docker.sh

# Install Docker Compose
sudo curl -L "https://github.com/docker/compose/releases/download/v2.20.0/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
sudo chmod +x /usr/local/bin/docker-compose
```

## Environment Setup

### 1. Development Environment

#### Quick Start with Docker Compose

```bash
# Clone the repository
git clone https://github.com/EchoLayerS/EchoLayer.git
cd EchoLayer

# Run the installation script
chmod +x scripts/install.sh
./scripts/install.sh

# Start all services
cd docker
docker-compose up -d

# View logs
docker-compose logs -f
```

#### Service URLs (Development)
- Frontend: http://localhost:3000
- Backend API: http://localhost:8080
- PostgreSQL: localhost:5432
- Redis: localhost:6379
- Solana Test Validator: localhost:8899
- Prometheus: http://localhost:9090
- Grafana: http://localhost:3001

#### Manual Development Setup

```bash
# Backend setup
cd backend
cargo build --release
export DATABASE_URL=postgresql://echolayer_user:echolayer_pass@localhost:5432/echolayer
export REDIS_URL=redis://localhost:6379
cargo run

# Frontend setup (new terminal)
cd frontend
npm install
npm run dev

# Smart contracts setup (new terminal)
cd smart-contracts
anchor build
anchor deploy --provider.cluster localnet
```

### 2. Production Environment

#### Prerequisites

```bash
# Install Kubernetes
curl -LO "https://dl.k8s.io/release/$(curl -L -s https://dl.k8s.io/release/stable.txt)/bin/linux/amd64/kubectl"
sudo install -o root -g root -m 0755 kubectl /usr/local/bin/kubectl

# Install Helm
curl https://baltocdn.com/helm/signing.asc | gpg --dearmor | sudo tee /usr/share/keyrings/helm.gpg > /dev/null
echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/helm.gpg] https://baltocdn.com/helm/stable/debian/ all main" | sudo tee /etc/apt/sources.list.d/helm-stable-debian.list
sudo apt-get update
sudo apt-get install helm
```

#### Kubernetes Deployment

Create namespace and secrets:

```bash
# Create namespace
kubectl create namespace echolayer

# Create secrets
kubectl create secret generic echolayer-secrets \
  --from-literal=database-url=postgresql://user:pass@postgres:5432/echolayer \
  --from-literal=redis-url=redis://redis:6379 \
  --from-literal=jwt-secret=your-super-secret-jwt-key \
  --namespace=echolayer

# Create ConfigMap
kubectl create configmap echolayer-config \
  --from-literal=rust-log=info \
  --from-literal=server-host=0.0.0.0 \
  --from-literal=server-port=8080 \
  --namespace=echolayer
```

Deploy PostgreSQL:

```yaml
# k8s/postgres.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: postgres
  namespace: echolayer
spec:
  replicas: 1
  selector:
    matchLabels:
      app: postgres
  template:
    metadata:
      labels:
        app: postgres
    spec:
      containers:
      - name: postgres
        image: postgres:15-alpine
        env:
        - name: POSTGRES_DB
          value: echolayer
        - name: POSTGRES_USER
          value: echolayer_user
        - name: POSTGRES_PASSWORD
          valueFrom:
            secretKeyRef:
              name: postgres-secret
              key: password
        ports:
        - containerPort: 5432
        volumeMounts:
        - name: postgres-storage
          mountPath: /var/lib/postgresql/data
      volumes:
      - name: postgres-storage
        persistentVolumeClaim:
          claimName: postgres-pvc
---
apiVersion: v1
kind: Service
metadata:
  name: postgres
  namespace: echolayer
spec:
  selector:
    app: postgres
  ports:
  - port: 5432
    targetPort: 5432
```

Deploy Redis:

```yaml
# k8s/redis.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: redis
  namespace: echolayer
spec:
  replicas: 1
  selector:
    matchLabels:
      app: redis
  template:
    metadata:
      labels:
        app: redis
    spec:
      containers:
      - name: redis
        image: redis:7-alpine
        ports:
        - containerPort: 6379
        volumeMounts:
        - name: redis-storage
          mountPath: /data
      volumes:
      - name: redis-storage
        persistentVolumeClaim:
          claimName: redis-pvc
---
apiVersion: v1
kind: Service
metadata:
  name: redis
  namespace: echolayer
spec:
  selector:
    app: redis
  ports:
  - port: 6379
    targetPort: 6379
```

Deploy Backend:

```yaml
# k8s/backend.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: echolayer-backend
  namespace: echolayer
spec:
  replicas: 3
  selector:
    matchLabels:
      app: echolayer-backend
  template:
    metadata:
      labels:
        app: echolayer-backend
    spec:
      containers:
      - name: backend
        image: echolayer/backend:latest
        ports:
        - containerPort: 8080
        env:
        - name: DATABASE_URL
          valueFrom:
            secretKeyRef:
              name: echolayer-secrets
              key: database-url
        - name: REDIS_URL
          valueFrom:
            secretKeyRef:
              name: echolayer-secrets
              key: redis-url
        - name: JWT_SECRET
          valueFrom:
            secretKeyRef:
              name: echolayer-secrets
              key: jwt-secret
        - name: RUST_LOG
          valueFrom:
            configMapKeyRef:
              name: echolayer-config
              key: rust-log
        resources:
          requests:
            memory: "512Mi"
            cpu: "250m"
          limits:
            memory: "1Gi"
            cpu: "500m"
        livenessProbe:
          httpGet:
            path: /api/v1/health
            port: 8080
          initialDelaySeconds: 30
          periodSeconds: 10
        readinessProbe:
          httpGet:
            path: /api/v1/health
            port: 8080
          initialDelaySeconds: 5
          periodSeconds: 5
---
apiVersion: v1
kind: Service
metadata:
  name: echolayer-backend
  namespace: echolayer
spec:
  selector:
    app: echolayer-backend
  ports:
  - port: 8080
    targetPort: 8080
  type: ClusterIP
```

Deploy Frontend:

```yaml
# k8s/frontend.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: echolayer-frontend
  namespace: echolayer
spec:
  replicas: 2
  selector:
    matchLabels:
      app: echolayer-frontend
  template:
    metadata:
      labels:
        app: echolayer-frontend
    spec:
      containers:
      - name: frontend
        image: echolayer/frontend:latest
        ports:
        - containerPort: 3000
        env:
        - name: NEXT_PUBLIC_API_URL
          value: "https://api.echolayers.xyz/api/v1"
        - name: NEXT_PUBLIC_WS_URL
          value: "wss://api.echolayers.xyz/ws"
        resources:
          requests:
            memory: "256Mi"
            cpu: "100m"
          limits:
            memory: "512Mi"
            cpu: "200m"
---
apiVersion: v1
kind: Service
metadata:
  name: echolayer-frontend
  namespace: echolayer
spec:
  selector:
    app: echolayer-frontend
  ports:
  - port: 3000
    targetPort: 3000
  type: ClusterIP
```

Ingress Configuration:

```yaml
# k8s/ingress.yaml
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: echolayer-ingress
  namespace: echolayer
  annotations:
    kubernetes.io/ingress.class: nginx
    cert-manager.io/cluster-issuer: letsencrypt-prod
    nginx.ingress.kubernetes.io/ssl-redirect: "true"
spec:
  tls:
  - hosts:
    - echolayers.xyz
    - api.echolayers.xyz
    secretName: echolayer-tls
  rules:
  - host: echolayers.xyz
    http:
      paths:
      - path: /
        pathType: Prefix
        backend:
          service:
            name: echolayer-frontend
            port:
              number: 3000
  - host: api.echolayers.xyz
    http:
      paths:
      - path: /
        pathType: Prefix
        backend:
          service:
            name: echolayer-backend
            port:
              number: 8080
```

Apply Kubernetes manifests:

```bash
# Apply all configurations
kubectl apply -f k8s/

# Check deployment status
kubectl get pods -n echolayer
kubectl get services -n echolayer
kubectl get ingress -n echolayer
```

## CI/CD Pipeline

### GitHub Actions Workflow

```yaml
# .github/workflows/deploy.yml
name: Deploy EchoLayer

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3
    
    - name: Setup Node.js
      uses: actions/setup-node@v3
      with:
        node-version: '18'
        cache: 'npm'
        cache-dependency-path: frontend/package-lock.json
    
    - name: Setup Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
    
    - name: Install frontend dependencies
      run: cd frontend && npm ci
    
    - name: Run frontend tests
      run: cd frontend && npm test
    
    - name: Run backend tests
      run: cd backend && cargo test
    
    - name: Build smart contracts
      run: cd smart-contracts && anchor build

  build-and-deploy:
    needs: test
    runs-on: ubuntu-latest
    if: github.ref == 'refs/heads/main'
    
    steps:
    - uses: actions/checkout@v3
    
    - name: Login to DockerHub
      uses: docker/login-action@v2
      with:
        username: ${{ secrets.DOCKER_USERNAME }}
        password: ${{ secrets.DOCKER_PASSWORD }}
    
    - name: Build and push backend
      uses: docker/build-push-action@v4
      with:
        context: backend
        file: docker/Dockerfile.backend
        push: true
        tags: echolayer/backend:latest
    
    - name: Build and push frontend
      uses: docker/build-push-action@v4
      with:
        context: frontend
        file: docker/Dockerfile.frontend
        push: true
        tags: echolayer/frontend:latest
    
    - name: Deploy to Kubernetes
      uses: azure/k8s-deploy@v1
      with:
        namespace: echolayer
        manifests: |
          k8s/backend.yaml
          k8s/frontend.yaml
        images: |
          echolayer/backend:latest
          echolayer/frontend:latest
```

## Database Management

### Migrations

```bash
# Run database migrations
cd backend
sqlx migrate run

# Create new migration
sqlx migrate add create_users_table

# Revert last migration
sqlx migrate revert
```

### Backup and Restore

```bash
# Backup database
pg_dump -h localhost -U echolayer_user -d echolayer > backup.sql

# Restore database
psql -h localhost -U echolayer_user -d echolayer < backup.sql

# Automated backup script
#!/bin/bash
DATE=$(date +%Y%m%d_%H%M%S)
pg_dump -h localhost -U echolayer_user -d echolayer | gzip > "backup_${DATE}.sql.gz"
```

## Monitoring and Logging

### Prometheus Configuration

```yaml
# docker/prometheus.yml
global:
  scrape_interval: 15s

scrape_configs:
  - job_name: 'echolayer-backend'
    static_configs:
      - targets: ['backend:8080']
  
  - job_name: 'echolayer-frontend'
    static_configs:
      - targets: ['frontend:3000']
  
  - job_name: 'postgres'
    static_configs:
      - targets: ['postgres:5432']
  
  - job_name: 'redis'
    static_configs:
      - targets: ['redis:6379']
```

### Grafana Dashboards

```json
{
  "dashboard": {
    "title": "EchoLayer Metrics",
    "panels": [
      {
        "title": "API Response Time",
        "type": "graph",
        "targets": [
          {
            "expr": "histogram_quantile(0.95, http_request_duration_seconds_bucket)",
            "legendFormat": "95th percentile"
          }
        ]
      },
      {
        "title": "Echo Index Calculations",
        "type": "singlestat",
        "targets": [
          {
            "expr": "rate(echo_index_calculations_total[5m])",
            "legendFormat": "per second"
          }
        ]
      }
    ]
  }
}
```

## Security Configuration

### SSL/TLS Setup

```bash
# Install cert-manager
kubectl apply -f https://github.com/cert-manager/cert-manager/releases/download/v1.12.0/cert-manager.yaml

# Create ClusterIssuer
kubectl apply -f - <<EOF
apiVersion: cert-manager.io/v1
kind: ClusterIssuer
metadata:
  name: letsencrypt-prod
spec:
  acme:
    server: https://acme-v02.api.letsencrypt.org/directory
    email: admin@echolayers.xyz
    privateKeySecretRef:
      name: letsencrypt-prod
    solvers:
    - http01:
        ingress:
          class: nginx
EOF
```

### Firewall Configuration

```bash
# UFW configuration
sudo ufw default deny incoming
sudo ufw default allow outgoing
sudo ufw allow ssh
sudo ufw allow 80/tcp
sudo ufw allow 443/tcp
sudo ufw enable
```

## Performance Optimization

### Database Optimization

```sql
-- Create indexes for better performance
CREATE INDEX CONCURRENTLY idx_content_created_at ON content(created_at);
CREATE INDEX CONCURRENTLY idx_propagations_timestamp ON propagations(timestamp);
CREATE INDEX CONCURRENTLY idx_users_echo_score ON users(echo_score DESC);

-- Vacuum and analyze
VACUUM ANALYZE;

-- Update statistics
ANALYZE;
```

### Caching Strategy

```bash
# Redis configuration for production
redis-cli CONFIG SET maxmemory 2gb
redis-cli CONFIG SET maxmemory-policy allkeys-lru
redis-cli CONFIG SET save "900 1 300 10 60 10000"
```

## Troubleshooting

### Common Issues

1. **Database Connection Issues**
```bash
# Check PostgreSQL status
kubectl logs -n echolayer postgres-xxx

# Test connection
psql -h localhost -U echolayer_user -d echolayer -c "SELECT 1;"
```

2. **Backend Service Not Responding**
```bash
# Check backend logs
kubectl logs -n echolayer echolayer-backend-xxx

# Check health endpoint
curl http://localhost:8080/api/v1/health
```

3. **Frontend Build Issues**
```bash
# Clear Next.js cache
cd frontend
rm -rf .next
npm run build
```

### Performance Issues

```bash
# Monitor resource usage
kubectl top pods -n echolayer
kubectl top nodes

# Check database performance
psql -c "SELECT * FROM pg_stat_activity;"

# Monitor Redis
redis-cli info memory
redis-cli info stats
```

## Scaling

### Horizontal Pod Autoscaler

```yaml
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: echolayer-backend-hpa
  namespace: echolayer
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: echolayer-backend
  minReplicas: 2
  maxReplicas: 10
  metrics:
  - type: Resource
    resource:
      name: cpu
      target:
        type: Utilization
        averageUtilization: 70
  - type: Resource
    resource:
      name: memory
      target:
        type: Utilization
        averageUtilization: 80
```

### Database Scaling

```bash
# Read replicas setup
kubectl apply -f k8s/postgres-replica.yaml

# Connection pooling
kubectl apply -f k8s/pgbouncer.yaml
```

This deployment guide provides comprehensive instructions for setting up EchoLayer in various environments, from development to production, with proper monitoring, security, and scaling considerations. 